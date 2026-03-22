use std::time::Duration;

use basalt_admin_internal_api_server::models;

use basalt_vultiserver_client::apis::configuration::Configuration as VultiserverConfig;
use basalt_vultiserver_client::apis::health_api as vultiserver_health;

use basalt_networking_internal_client::apis::configuration::Configuration as NetworkingConfig;
use basalt_networking_internal_client::apis::default_api as networking_api;

pub async fn check_vultiserver(config: &VultiserverConfig) -> models::ContainerStatus {
    let name = "vultiserver".to_string();
    match vultiserver_health::ping(config).await {
        Ok(body) => models::ContainerStatus::new(name, true, body),
        Err(e) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
    }
}

pub async fn check_networking(config: &NetworkingConfig) -> models::ContainerStatus {
    let name = "networking".to_string();
    match networking_api::health(config).await {
        Ok(body) => models::ContainerStatus::new(name, true, body),
        Err(e) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
    }
}

pub async fn check_redis(client: &redis::Client) -> models::ContainerStatus {
    let name = "redis".to_string();
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
    })
    .await;
    match result {
        Ok(Ok(response)) => models::ContainerStatus::new(name, true, response),
        Ok(Err(e)) => models::ContainerStatus::new(name, false, format!("{e}")),
        Err(_) => models::ContainerStatus::new(name, false, "timeout".to_string()),
    }
}
