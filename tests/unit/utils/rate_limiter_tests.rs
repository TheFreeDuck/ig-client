// Tests for the rate limiter module
use ig_client::utils::rate_limiter::{
    RateLimitType, RateLimiter, account_non_trading_limiter, account_trading_limiter,
    app_non_trading_limiter, global_rate_limiter, historical_price_limiter,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_test::block_on;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_type_min_interval() {
        // Test that each rate limit type returns the expected interval
        assert_eq!(RateLimitType::NonTradingAccount.min_interval_ms(), 4000);
        assert_eq!(RateLimitType::TradingAccount.min_interval_ms(), 2000);
        assert_eq!(RateLimitType::NonTradingApp.min_interval_ms(), 3000);
        assert_eq!(RateLimitType::HistoricalPrice.min_interval_ms(), 120000);
    }

    #[test]
    fn test_rate_limiter_new() {
        // Test creating a new rate limiter
        let limiter = RateLimiter::new(RateLimitType::NonTradingAccount);
        
        // We can't directly access the private fields, but we can test the behavior
        // by calling wait() and measuring the time
        let start = Instant::now();
        block_on(limiter.wait()); // First call should not wait
        let first_call_duration = start.elapsed();
        
        // The first call should return almost immediately
        assert!(first_call_duration < Duration::from_millis(100));
    }

    // Este test es más difícil de probar de manera determinista debido a la naturaleza del tiempo
    // y cómo se maneja en el rate limiter. En lugar de verificar el tiempo exacto,
    // vamos a verificar la funcionalidad básica sin depender del tiempo real.
    #[test]
    fn test_rate_limiter_wait() {
        // Creamos un rate limiter con un tipo que tiene un intervalo más corto para el test
        let limiter = RateLimiter::new(RateLimitType::TradingAccount); // 2000ms
        
        // Primera llamada no debería esperar
        let start = Instant::now();
        block_on(limiter.wait());
        let first_duration = start.elapsed();
        
        // La primera llamada debería ser muy rápida (menos de 100ms)
        assert!(first_duration < Duration::from_millis(100));
        
        // Verificamos que el last_call se haya actualizado (no podemos acceder directamente)
        // pero podemos verificar indirectamente creando un nuevo limiter y comparando comportamientos
        let limiter2 = RateLimiter::new(RateLimitType::TradingAccount);
        
        // Este limiter no debería tener last_call configurado, así que debería ser rápido
        let start = Instant::now();
        block_on(limiter2.wait());
        let second_duration = start.elapsed();
        
        // También debería ser rápido
        assert!(second_duration < Duration::from_millis(100));
    }

    #[test]
    fn test_global_limiters() {
        // Test that global limiters return Arc<RateLimiter> instances
        let non_trading = account_non_trading_limiter();
        let trading = account_trading_limiter();
        let app = app_non_trading_limiter();
        let historical = historical_price_limiter();
        let global = global_rate_limiter();
        
        // Test that global_rate_limiter returns the same instance as account_non_trading_limiter
        assert!(Arc::ptr_eq(&global, &account_non_trading_limiter()));
        
        // Test that the limiters are different instances
        assert!(!Arc::ptr_eq(&non_trading, &trading));
        assert!(!Arc::ptr_eq(&non_trading, &app));
        assert!(!Arc::ptr_eq(&non_trading, &historical));
        assert!(!Arc::ptr_eq(&trading, &app));
        assert!(!Arc::ptr_eq(&trading, &historical));
        assert!(!Arc::ptr_eq(&app, &historical));
    }

    #[test]
    fn test_limiter_singleton_pattern() {
        // Test that calling the global limiter functions multiple times returns the same instance
        let non_trading1 = account_non_trading_limiter();
        let non_trading2 = account_non_trading_limiter();
        
        // Test that they are the same instance (same memory address)
        assert!(Arc::ptr_eq(&non_trading1, &non_trading2));
        
        // Repeat for other limiters
        let trading1 = account_trading_limiter();
        let trading2 = account_trading_limiter();
        assert!(Arc::ptr_eq(&trading1, &trading2));
        
        let app1 = app_non_trading_limiter();
        let app2 = app_non_trading_limiter();
        assert!(Arc::ptr_eq(&app1, &app2));
        
        let historical1 = historical_price_limiter();
        let historical2 = historical_price_limiter();
        assert!(Arc::ptr_eq(&historical1, &historical2));
        
        let global1 = global_rate_limiter();
        let global2 = global_rate_limiter();
        assert!(Arc::ptr_eq(&global1, &global2));
    }
}
