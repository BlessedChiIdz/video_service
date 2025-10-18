use std::str::FromStr;

pub fn read_env<T: FromStr>(key: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let env_value = std::env::var(key);
    let env_str: String = match env_value {
        Ok(val) => {
            tracing::info!("{:?} env set, value = {}", key, val);
            val
        }
        Err(e) => {
            tracing::error!("{:?} env is not set: {}", key, e);
            std::process::exit(1);
        }
    };

    env_str.parse().unwrap_or_else(|e| {
        tracing::error!(
            "Failed to parse environment variable '{}' with value '{}' as target type: {}",
            key, env_str, e
        );
        std::process::exit(1);
    })
}