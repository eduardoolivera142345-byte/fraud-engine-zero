mod engine;
mod telemetry;

use std::sync::Arc;
use std::time::Instant;
use tonic::{transport::Server, Request, Response, Status};
use metrics_exporter_prometheus::PrometheusBuilder;

// Imports de WebSocket e Tokio Sync
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;

pub mod transaction {
    tonic::include_proto!("fraud.engine.v1");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("transaction_descriptor");
}

use transaction::fraud_engine_service_server::{FraudEngineService, FraudEngineServiceServer};
use transaction::{TransactionRequest, TransactionResponse};

pub struct MyService {
    engine: Arc<engine::FraudEngine>,
    ws_tx: broadcast::Sender<String>,
}

#[tonic::async_trait]
impl FraudEngineService for MyService {
    async fn evaluate_transaction(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let start = Instant::now();
        let req = request.into_inner();

        let (decision_str, score) = self.engine.evaluate(req.amount, 1, true).await;
        let elapsed_us = start.elapsed().as_micros() as i64;

        let decision_code: i32 = if decision_str == "APPROVED" { 0 } else { 1 };

        metrics::increment_counter!("fraud_transactions_total");
        metrics::gauge!("fraud_transaction_latency_us", elapsed_us as f64);

        let tx_id = if req.transaction_id.is_empty() {
            format!("tx_{}", start.elapsed().as_nanos())
        } else {
            req.transaction_id.clone()
        };

        // 📡 EVENTO WEBSOCKET (Zero Lock / Broadcast Assíncrono)
        let ws_event = json!({
            "transaction_id": tx_id,
            "account_origin": "0001-99823",
            "account_destination": "0001-11234",
            "amount": req.amount,
            "risk_score": score,
            "execution_time_us": elapsed_us,
            "decision": decision_str,
            "institution_node": "Alpha Bank Node-1"
        }).to_string();

        // Dispara para o Dashboard no canal WS sem bloquear a API gRPC
        let _ = self.ws_tx.send(ws_event);

        let response = TransactionResponse {
            transaction_id: req.transaction_id,
            decision: decision_code,
            risk_score: score,
            execution_time_us: elapsed_us,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📊 Prometheus Exporter
    if let Err(e) = PrometheusBuilder::new().with_http_listener(([0, 0, 0, 0], 9090)).install() {
        eprintln!("Aviso Prometheus Exporter: {}", e);
    } else {
        println!("📊 Prometheus Metrics rodando em http://0.0.0.0:9090/metrics");
    }

    // 📡 Canal Broadcast para o WebSocket (capacidade para 1024 mensagens concorrentes)
    let (ws_tx, _) = broadcast::channel::<String>(1024);
    let ws_tx_clone = ws_tx.clone();

    // 🌐 Task Assíncrona para o Servidor WebSocket na porta 8080
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.expect("Erro ao abrir porta 8080");
        println!("🌐 Servidor WebSocket Realtime rodando em ws://0.0.0.0:8080");

        while let Ok((stream, _)) = listener.accept().await {
            let mut rx = ws_tx_clone.subscribe();
            tokio::spawn(async move {
                if let Ok(ws_stream) = accept_async(stream).await {
                    let (mut write, _) = ws_stream.split();
                    while let Ok(msg) = rx.recv().await {
                        if write.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                }
            });
        }
    });

    // ⚡ Serviço de Reflection do gRPC
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(transaction::FILE_DESCRIPTOR_SET)
        .build()?;

    let addr = "0.0.0.0:50051".parse()?;
    let engine = Arc::new(engine::FraudEngine::new());
    let service = MyService { engine, ws_tx };

    println!("⚡ Engine Anti-Fraude rodando na porta gRPC {}", addr);

    Server::builder()
        .add_service(FraudEngineServiceServer::new(service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
