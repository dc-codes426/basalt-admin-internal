use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use headers::Host;
use http::Method;
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
            check_redis()
        );

        Ok(HealthResponse::Status200_HealthCheckResultsForAllDependencies(
            models::PingResponse::new(vec![vultiserver, networking, redis]),
        ))
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

async fn check_redis() -> models::ContainerStatus {
    let name = "redis".to_string();
    match TcpStream::connect("redis:6379").await {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(b"PING\r\n").await {
                return models::ContainerStatus::new(name, false, format!("write failed: {e}"));
            }
            let mut buf = [0u8; 64];
            match stream.read(&mut buf).await {
                Ok(n) => {
                    let response = String::from_utf8_lossy(&buf[..n]);
                    let healthy = response.contains("PONG");
                    models::ContainerStatus::new(name, healthy, response.trim().to_string())
                }
                Err(e) => models::ContainerStatus::new(name, false, format!("read failed: {e}")),
            }
        }
        Err(e) => models::ContainerStatus::new(name, false, format!("unreachable: {e}")),
    }
}

#[tokio::main]
async fn main() {
    let vultiserver_url = std::env::var("VULTISERVER_URL")
        .unwrap_or_else(|_| "http://vultiserver:8080".to_string());
    let networking_url = std::env::var("NETWORKING_INTERNAL_URL")
        .unwrap_or_else(|_| "http://networking:8080".to_string());

    let mut vultiserver_config = VultiserverConfig::new();
    vultiserver_config.base_path = vultiserver_url;

    let mut networking_config = NetworkingConfig::new();
    networking_config.base_path = networking_url;

    let server = Server {
        vultiserver_client: vultiserver_config,
        networking_client: networking_config,
    };

    let app = basalt_admin_internal_api_server::server::new(server);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("basalt-admin-internal listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
