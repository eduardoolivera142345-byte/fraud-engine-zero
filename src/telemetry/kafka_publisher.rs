use tokio::spawn;

pub struct AuditLogger;

impl AuditLogger {
    pub fn log_transaction_async(tx_id: String, decision: String, score: f32, elapsed_us: u128) {
        spawn(async move {
            let log_payload = serde_json::json!({
                "transaction_id": tx_id,
                "decision": decision,
                "risk_score": score,
                "latency_us": elapsed_us,
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
            });

            println!("[AUDIT LOG - ASYNC] {}", log_payload);
        });
    }
}
