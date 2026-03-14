use foxtive::cache::drivers::RedisCacheDriver;
use foxtive::database::DbConfig;
use foxtive::helpers::env;
use foxtive::rabbitmq::config::RabbitmqConfig;
use foxtive::redis::config::RedisConfig;
use foxtive::results::AppResult;
use foxtive::setup::{CacheDriverSetup, FoxtiveSetup};
use foxtive::Environment;
use std::sync::Arc;

pub async fn setup(env_prefix: &str) -> AppResult<FoxtiveSetup> {
    let environment = Environment::from_env(&format!("{env_prefix}_APP_ENVIRONMENT"))?;

    let cache_setup = CacheDriverSetup::Redis(|redis| Arc::new(RedisCacheDriver::new(redis)));

    let redis_dsn = env::var(env_prefix, "REDIS_DSN")?;
    let redis_pool_max_size = env::var(env_prefix, "REDIS_POOL_MAX_SIZE")?.parse()?;
    let redis_pool_config = foxtive::redis::config::PoolConfig::new(redis_pool_max_size);
    let redis_config = RedisConfig::create(&redis_dsn).pool_config(redis_pool_config);

    let db_config = {
        let db_dsn = env::var(env_prefix, "DB_DSN")?;
        let db_pool_max_size = env::var(env_prefix, "DB_POOL_MAX_SIZE")?.parse()?;
        DbConfig::create(&db_dsn).max_size(db_pool_max_size)
    };

    let rmq_config = {
        let rmq_dsn = env::var(env_prefix, "RMQ_DSN")?;
        let rmq_pool_max_size = env::var(env_prefix, "RMQ_POOL_MAX_SIZE")?.parse()?;
        let rmq_pool_config = foxtive::rabbitmq::config::PoolConfig::new(rmq_pool_max_size);
        RabbitmqConfig::create(&rmq_dsn).pool_config(rmq_pool_config)
    };

    Ok(FoxtiveSetup {
        db_config,
        rmq_config,
        redis_config,
        env: environment,
        cache_driver_setup: cache_setup,
        template_directory: "resources/templates/**/*.html".to_string(),
        env_prefix: env_prefix.to_string(),
        public_key: include_str!("../../../../resources/keys/app-auth-public-key.pem").to_owned(),
        private_key: include_str!("../../../../resources/keys/app-auth-private-key.pem").to_owned(),
        jwt_iss_public_key: include_str!("../../../../resources/keys/app-auth-public-key.pem")
            .to_owned(),
        app_key: env::var(env_prefix, "APP_KEY")?,
        app_code: env::var(env_prefix, "APP_CODE")?,
        app_name: env::var(env_prefix, "APP_NAME")?,
        jwt_token_lifetime: env::var(env_prefix, "AUTH_TOKEN_LIFETIME")?.parse()?,
    })
}
