use std::collections::HashMap;
use std::time::{Duration, Instant};

use basalt_admin_internal_api_server::models;

use basalt_vultiserver_client::apis::configuration::Configuration as VultiserverConfig;
use basalt_vultiserver_client::apis::health_api as vultiserver_health;

use basalt_networking_internal_client::apis::configuration::Configuration as NetworkingConfig;
use basalt_networking_internal_client::apis::default_api as networking_api;

use reqwest::Client;

use crate::Server;

// --- Individual checks used by /health ---

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

pub async fn check_auth(client: &Client, base_url: &str) -> models::ContainerStatus {
    let name = "auth".to_string();
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        client
            .get(format!("{base_url}/ping"))
            .send()
            .await?
            .text()
            .await
    })
    .await;
    match result {
        Ok(Ok(body)) => models::ContainerStatus::new(name, true, body),
        Ok(Err(e)) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
        Err(_) => models::ContainerStatus::new(name, false, "timeout".to_string()),
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

// --- Health report checks used by /health/report ---

fn check_result(passed: bool, detail: String, latency_ms: Option<f64>) -> models::CheckResult {
    models::CheckResult { passed, detail, latency_ms }
}

async fn timed_ping_vultiserver(config: &VultiserverConfig) -> models::CheckResult {
    let start = Instant::now();
    match vultiserver_health::ping(config).await {
        Ok(body) => check_result(true, body, Some(start.elapsed().as_secs_f64() * 1000.0)),
        Err(e) => check_result(false, format!("unreachable: {e}"), None),
    }
}

async fn timed_ping_networking(config: &NetworkingConfig) -> models::CheckResult {
    let start = Instant::now();
    match networking_api::health(config).await {
        Ok(body) => check_result(true, body, Some(start.elapsed().as_secs_f64() * 1000.0)),
        Err(e) => check_result(false, format!("unreachable: {e}"), None),
    }
}

async fn timed_ping_auth(client: &Client, base_url: &str) -> models::CheckResult {
    let start = Instant::now();
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        client
            .get(format!("{base_url}/ping"))
            .send()
            .await?
            .text()
            .await
    })
    .await;
    match result {
        Ok(Ok(body)) => check_result(true, body, Some(start.elapsed().as_secs_f64() * 1000.0)),
        Ok(Err(e)) => check_result(false, format!("unreachable: {e}"), None),
        Err(_) => check_result(false, "timeout".to_string(), None),
    }
}

async fn timed_ping_redis(client: &redis::Client) -> models::CheckResult {
    let start = Instant::now();
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
    })
    .await;
    match result {
        Ok(Ok(response)) => check_result(true, response, Some(start.elapsed().as_secs_f64() * 1000.0)),
        Ok(Err(e)) => check_result(false, format!("{e}"), None),
        Err(_) => check_result(false, "timeout".to_string(), None),
    }
}

fn service_status(checks: &HashMap<String, models::CheckResult>) -> models::ServiceStatus {
    let all_passed = checks.values().all(|c| c.passed);
    if all_passed {
        models::ServiceStatus::Healthy
    } else {
        models::ServiceStatus::Unhealthy
    }
}

fn build_service_report(checks: HashMap<String, models::CheckResult>) -> models::ServiceReport {
    let status = service_status(&checks);
    models::ServiceReport::new(status, checks)
}

pub async fn build_health_report(
    redis_client: &redis::Client,
    vultiserver_config: &VultiserverConfig,
    networking_config: &NetworkingConfig,
    http_client: &Client,
    auth_url: &str,
) -> models::HealthReport {
    let (redis_ping, vultiserver_ping, networking_ping, auth_ping) = tokio::join!(
        timed_ping_redis(redis_client),
        timed_ping_vultiserver(vultiserver_config),
        timed_ping_networking(networking_config),
        timed_ping_auth(http_client, auth_url),
    );

    let redis_report = build_service_report(HashMap::from([
        ("ping".to_string(), redis_ping),
    ]));

    let vultiserver_report = build_service_report(HashMap::from([
        ("ping".to_string(), vultiserver_ping),
    ]));

    let networking_report = build_service_report(HashMap::from([
        ("ping".to_string(), networking_ping),
    ]));

    let auth_report = build_service_report(HashMap::from([
        ("ping".to_string(), auth_ping),
    ]));

    let services = models::HealthReportServices::new(
        redis_report,
        vultiserver_report,
        networking_report,
        auth_report,
    );

    let overall = if [&services.redis, &services.vultiserver, &services.networking, &services.auth]
        .iter()
        .all(|s| s.status == models::ServiceStatus::Healthy)
    {
        models::ServiceStatus::Healthy
    } else if [&services.redis, &services.vultiserver, &services.networking, &services.auth]
        .iter()
        .any(|s| s.status == models::ServiceStatus::Unhealthy)
    {
        models::ServiceStatus::Unhealthy
    } else {
        models::ServiceStatus::Degraded
    };

    models::HealthReport::new(chrono::Utc::now(), overall, services)
}

// --- Periodic log reporting ---

async fn redis_key_count(_client: &redis::Client) -> Option<u64> {
    // TODO: implement actual key count retrieval
    None
}

pub async fn periodic_log_report(server: Server) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;

        let report = build_health_report(
            &server.redis_client,
            &server.vultiserver_client,
            &server.networking_client,
            &server.http_client,
            &server.auth_url,
        )
        .await;

        let key_count = redis_key_count(&server.redis_client).await;

        match serde_json::to_string(&report) {
            Ok(json) => tracing::info!(target: "health_report", report = %json, redis_key_count = ?key_count, "periodic health report"),
            Err(e) => tracing::error!("failed to serialize health report: {e}"),
        }
    }
}
