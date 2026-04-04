use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use headers::Host;
use http::Method;
use std::time::Duration;

use basalt_admin_internal_api_server::apis;
use basalt_admin_internal_api_server::apis::default::{
    HealthReportResponse, HealthResponse, PingResponse as PingApiResponse,
};
use basalt_admin_internal_api_server::models;

use basalt_vultiserver_client::apis::configuration::Configuration as VultiserverConfig;
use basalt_networking_internal_client::apis::configuration::Configuration as NetworkingConfig;

mod checks;

#[derive(Clone)]
pub struct Server {
    pub vultiserver_client: VultiserverConfig,
    pub networking_client: NetworkingConfig,
    pub redis_client: redis::Client,
    pub http_client: reqwest::Client,
    pub auth_url: String,
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
        let (vultiserver, networking, redis, auth) = tokio::join!(
            checks::check_vultiserver(&self.vultiserver_client),
            checks::check_networking(&self.networking_client),
            checks::check_redis(&self.redis_client),
            checks::check_auth(&self.http_client, &self.auth_url),
        );

        let all_healthy = vultiserver.healthy && networking.healthy && redis.healthy && auth.healthy;
        let response = models::PingResponse::new(vec![vultiserver, networking, redis, auth]);
        if all_healthy {
            Ok(HealthResponse::Status200_AllDependenciesAreHealthy(response))
        } else {
            Ok(HealthResponse::Status503_OneOrMoreDependenciesAreUnhealthy(response))
        }
    }

    async fn health_report(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<HealthReportResponse, ()> {
        let report = checks::build_health_report(&self.redis_client, &self.vultiserver_client, &self.networking_client, &self.http_client, &self.auth_url).await;
        Ok(HealthReportResponse::Status200_ReportGeneratedSuccessfully(report))
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
    let auth_url = std::env::var("AUTH_URL")
        .unwrap_or_else(|_| "http://auth:3000".to_string());
    let redis_host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_password = std::env::var("REDIS_PASSWORD").unwrap_or_default();
    let redis_url = if redis_password.is_empty() {
        format!("redis://{redis_host}:{redis_port}")
    } else {
        format!("redis://:{redis_password}@{redis_host}:{redis_port}")
    };
    let redis_client = redis::Client::open(redis_url).expect("invalid redis connection URL");

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
    networking_config.client = http_client.clone();

    let server = Server {
        vultiserver_client: vultiserver_config,
        networking_client: networking_config,
        redis_client,
        http_client,
        auth_url,
    };

    let app = basalt_admin_internal_api_server::server::new(server);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind on 0.0.0.0:3000");
    tracing::info!("basalt-admin-internal listening on port 3000");
    axum::serve(listener, app)
        .await
        .expect("server failed");
}
