use z3::ast::{Bool, Int};
use z3::{Config, Context, Solver};

pub struct Z3Checker;

impl Z3Checker {
    pub fn is_fraudulent(amount: f64, account_age_hours: i64, pwd_changed: bool, max_amount_limit: i64) -> bool {
        // ⚡ FAST PATH (Atalhos na CPU em nanosegundos)
        // Se a regra é simples ou explícita, resolve direto no registrador da CPU sem alocar o Z3
        let limit_f64 = max_amount_limit as f64;
        
        // Regra 1: Valor extremamente alto = Fraude imediata (< 1 microsegundo)
        if amount > limit_f64 * 10.0 {
            return true;
        }

        // Regra 2: Conta antiga e valor baixo = Aprovado imediato (< 1 microsegundo)
        if !pwd_changed && account_age_hours > 72 && amount < limit_f64 {
            return false;
        }

        // 🧠 FALLBACK (Z3 Solver em C++ só para decisões complexas e ambíguas)
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let z3_amount = Int::from_i64(&ctx, amount as i64);
        let z3_age = Int::from_i64(&ctx, account_age_hours);
        let z3_pwd = Bool::from_bool(&ctx, pwd_changed);

        let limit = Int::from_i64(&ctx, max_amount_limit);
        let age_limit = Int::from_i64(&ctx, 24);

        let is_high_risk = Bool::and(
            &ctx,
            &[
                &z3_amount.gt(&limit),
                &z3_age.lt(&age_limit),
                &z3_pwd,
            ],
        );

        solver.assert(&is_high_risk);
        solver.check() == z3::SatResult::Sat
    }
}
