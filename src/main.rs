use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use headers::Host;
use http::Method;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use basalt_admin_internal_api_server::apis;
use basalt_admin_internal_api_server::apis::default::{
    HealthResponse, PingResponse as PingApiResponse,
};
use basalt_admin_internal_api_server::models;

use basalt_vultiserver_client::apis::configuration::Configuration as VultiserverConfig;
use basalt_vultiserver_client::apis::health_api as vultiserver_health;

use basalt_networking_internal_client::apis::configuration::Configuration as NetworkingConfig;
use basalt_networking_internal_client::apis::default_api as networking_api;

#[derive(Clone)]
struct Server {
    vultiserver_client: VultiserverConfig,
    networking_client: NetworkingConfig,
    redis_addr: String,
    redis_password: String,
}

impl AsRef<Server> for Server {
    fn as_ref(&self) -> &Server {
        self
    }
}

impl apis::ErrorHandler for Server {}

#[async_trait]
impl apis::default::Default for Server {
    async fn health(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<HealthResponse, ()> {
        let (vultiserver, networking, redis) = tokio::join!(
            check_vultiserver(&self.vultiserver_client),
            check_networking(&self.networking_client),
            check_redis(&self.redis_addr, &self.redis_password)
        );

        let all_healthy = vultiserver.healthy && networking.healthy && redis.healthy;
        let response = models::PingResponse::new(vec![vultiserver, networking, redis]);
        if all_healthy {
            Ok(HealthResponse::Status200_AllDependenciesAreHealthy(response))
        } else {
            Ok(HealthResponse::Status503_OneOrMoreDependenciesAreUnhealthy(response))
        }
    }

    async fn ping(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<PingApiResponse, ()> {
        Ok(PingApiResponse::Status200_ServiceIsAlive("pong".to_string()))
    }
}

async fn check_vultiserver(config: &VultiserverConfig) -> models::ContainerStatus {
    let name = "vultiserver".to_string();
    match vultiserver_health::ping(config).await {
        Ok(body) => models::ContainerStatus::new(name, true, body),
        Err(e) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
    }
}

async fn check_networking(config: &NetworkingConfig) -> models::ContainerStatus {
    let name = "networking".to_string();
    match networking_api::health(config).await {
        Ok(body) => models::ContainerStatus::new(name, true, body),
        Err(e) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
    }
}

async fn check_redis(addr: &str, password: &str) -> models::ContainerStatus {
    let name = "redis".to_string();
    let timeout = Duration::from_secs(5);
    let result = tokio::time::timeout(timeout, async {
        let mut stream = TcpStream::connect(addr).await?;
        if !password.is_empty() {
            stream.write_all(format!("AUTH {password}\r\n").as_bytes()).await?;
            let mut buf = [0u8; 64];
            let _ = stream.read(&mut buf).await?;
        }
        stream.write_all(b"PING\r\n").await?;
        let mut buf = [0u8; 64];
        let n = stream.read(&mut buf).await?;
        Ok::<_, std::io::Error>(String::from_utf8_lossy(&buf[..n]).trim().to_string())
    })
    .await;
    match result {
        Ok(Ok(response)) => {
            let healthy = response.contains("PONG");
            models::ContainerStatus::new(name, healthy, response)
        }
        Ok(Err(e)) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
        Err(_) => models::ContainerStatus::new(name, false, "timeout".to_string()),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "basalt_admin_internal=info".parse().unwrap()),
        )
        .init();

    let vultiserver_url = std::env::var("VULTISERVER_URL")
        .unwrap_or_else(|_| "http://vultiserver:8080".to_string());
    let networking_url = std::env::var("NETWORKING_INTERNAL_URL")
        .unwrap_or_else(|_| "http://networking:8080".to_string());
    let redis_host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_addr = format!("{redis_host}:{redis_port}");
    let redis_password = std::env::var("REDIS_PASSWORD").unwrap_or_default();

    let http_client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build HTTP client");

    let mut vultiserver_config = VultiserverConfig::new();
    vultiserver_config.base_path = vultiserver_url;
    vultiserver_config.client = http_client.clone();

    let mut networking_config = NetworkingConfig::new();
    networking_config.base_path = networking_url;
    networking_config.client = http_client;

    let server = Server {
        vultiserver_client: vultiserver_config,
        networking_client: networking_config,
        redis_addr,
        redis_password,
    };

    let app = basalt_admin_internal_api_server::server::new(server);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("basalt-admin-internal listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
