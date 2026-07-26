pub mod z3_solver;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct DynamicConfig {
    pub max_amount_limit: i64,
}

#[derive(Clone)]
pub struct FraudEngine {
    pub config: Arc<RwLock<DynamicConfig>>,
}

impl FraudEngine {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(DynamicConfig { max_amount_limit: 1000 })),
        }
    }

    pub async fn evaluate(&self, amount: f64, account_age: i64, pwd_changed: bool) -> (String, f32) {
        let current_limit = self.config.read().await.max_amount_limit;

        let is_fraud = tokio::task::spawn_blocking(move || {
            z3_solver::Z3Checker::is_fraudulent(amount, account_age, pwd_changed, current_limit)
        })
        .await
        .unwrap_or(false);

        if is_fraud {
            ("REJECTED".to_string(), 1.0)
        } else {
            ("APPROVED".to_string(), 0.0)
        }
    }
}
