# 🛡️ Fraud Engine Zero — Motor Anti-Fraude em Tempo Real

> Motor de avaliação e prevenção a fraudes transacionais de ultra-baixa latência, construído em **Rust**, utilizando verificação formal de regras com **Z3 SMT Solver**, ingestão via **gRPC** e streaming de eventos em tempo real via **WebSockets**.

---

## 📽️ Demonstração em Tempo Real

🎬 **[Clique aqui para assistir à demonstração em tempo real no Streamable](https://streamable.com/4pzz49)**


> **Destaque do Teste:** Simulação operacional recebendo transações via ingestão gRPC, validando regras de risco no motor Rust/Z3 e transmitindo métricas e eventos ao vivo para a dashboard via WebSocket com latência sub-milissegundo.

---

## 🎯 Sobre o Projeto

O **Fraud Engine Zero** foi desenvolvido para resolver o desafio de análise de risco e detecção de fraudes em ambientes bancários e de meios de pagamento de altíssimo volume. 

Diferente de abordagens tradicionais baseadas apenas em regras imperativas lentas, o motor combina **verificação lógica formal (SMT Solver)** com a performance bruta e segurança de memória do **Rust**, garantindo tomadas de decisão determinísticas em microssegundos ($\mu s$).

### 🚀 Principais Diferenciais Técnicos:
- **Latência Sub-milissegundo ($\mu s$):** Processamento assíncrono paralelo alimentado pelo runtime `Tokio`.
- **Lógica Formal Garantida (Z3 SMT):** Validação matemática de restrições de fraude (evita falsos positivos e conflitos entre regras).
- **Ingestão gRPC de Alto Desempenho:** Comunicação binária para comunicação eficiente entre microsserviços/bancos parceiros.
- **Painel Telemétrico via WebSocket:** Streaming bidirecional para monitoramento em tempo real sem necessidade de *polling*.
- **Observabilidade Pronta para Produção:** Exportador de métricas no padrão Prometheus.

---

## 🏗️ Arquitetura do Sistema

```text
[ Simulador de Banco ] 
         │ (gRPC / Protocol Buffers)
         ▼
┌────────────────────────────────────────────────────────┐
│               RUST FRAUD ENGINE ZERO                   │
│                                                        │
│  ┌──────────────────┐       ┌───────────────────────┐  │
│  │ Tokio Async Core │ ────> │ Z3 SMT Solver (Lógica)│  │
│  └──────────────────┘       └───────────────────────┘  │
│           │                             │              │
│           ▼                             ▼              │
│  ┌──────────────────┐       ┌───────────────────────┐  │
│  │ Audit Logger     │       │ Prometheus Metrics    │  │
│  └──────────────────┘       └───────────────────────┘  │
└────────────────────────────────────────────────────────┘
         │ (WebSockets / JSON Stream)
         ▼
[ Dashboard Frontend em Tempo Real ]

🛠️ Tecnologias Utilizadas
Linguagem Backend: Rust (Edição 2021)
Runtime Assíncrono: tokio
Comunicação gRPC: tonic / prost (Protocol Buffers)
Motor de Lógica Formal: z3 (SMT Solver - Bit-vectors, Arith, Arrays)
Servidor Web / WebSocket: axum / tokio-tungstenite
Métricas e Telemetria: prometheus
Interface/Dashboard: React / Tailwind CSS / WebSockets
⚙️ Como Executar o Projeto Localmente
Pré-requisitos
Rust & Cargo (versão mais recente)
Clang / LLVM e suporte à biblioteca do Z3 (libz3-dev)
Protobuf Compiler (protobuf-compiler)
1. Clonar o Repositório
2.git clone https://github.com/eduardoolivera142345-byte/fraud-engine-zero.git
cd fraud-engine-zero
2. Compilar em Modo de Alta Performance (Release)
cargo build --release
3. Executar o Motor Anti-Fraude
cargo run --release

O motor iniciará os serviços nas seguintes portas:
gRPC: 0.0.0.0:50051
WebSocket: ws://0.0.0.0:8080
Prometheus Metrics: http://0.0.0.0:9090/metrics
4. Simular Carga Transacional
Em outro terminal, execute o script de simulação:
chmod +x simular_banco.sh
./simular_banco.sh
📡 Portas e Endpoints
ServiçoProtocoloEndereço / PortaDescrição
Ingestão gRPCHTTP/2 (gRPC)0.0.0.0:50051Avaliação de transações síncronas
Realtime StreamWebSocketws://0.0.0.0:8080Streaming de eventos para a dashboard
TelemetriaHTTP0.0.0.0:9090/metricsMétricas de tempo de resposta e vazão
📝 Licença
Este projeto está sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.
Developed by **Eduardo Oliveira** — [GitHub](https://github.com/eduardoolivera142345-byte)
