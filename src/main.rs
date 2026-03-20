use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize)]
struct ContainerStatus {
    name: String,
    healthy: bool,
    detail: String,
}

#[derive(Serialize)]
struct PingResponse {
    containers: Vec<ContainerStatus>,
}

async fn health() -> &'static str {
    "ok"
}

async fn check_vultiserver() -> ContainerStatus {
    let name = "vultiserver".to_string();
    match reqwest::get("http://vultiserver:8080/ping").await {
        Ok(resp) => match resp.text().await {
            Ok(body) => ContainerStatus {
                name,
                healthy: true,
                detail: body,
            },
            Err(e) => ContainerStatus {
                name,
                healthy: false,
                detail: format!("failed to read response: {e}"),
            },
        },
        Err(e) => ContainerStatus {
            name,
            healthy: false,
            detail: format!("unreachable: {e}"),
        },
    }
}

async fn check_networking() -> ContainerStatus {
    let name = "networking".to_string();
    match reqwest::get("http://networking:8080/health").await {
        Ok(resp) => match resp.text().await {
            Ok(body) => ContainerStatus {
                name,
                healthy: true,
                detail: body,
            },
            Err(e) => ContainerStatus {
                name,
                healthy: false,
                detail: format!("failed to read response: {e}"),
            },
        },
        Err(e) => ContainerStatus {
            name,
            healthy: false,
            detail: format!("unreachable: {e}"),
        },
    }
}

async fn check_redis() -> ContainerStatus {
    let name = "redis".to_string();
    match TcpStream::connect("redis:6379").await {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(b"PING\r\n").await {
                return ContainerStatus {
                    name,
                    healthy: false,
                    detail: format!("write failed: {e}"),
                };
            }
            let mut buf = [0u8; 64];
            match stream.read(&mut buf).await {
                Ok(n) => {
                    let response = String::from_utf8_lossy(&buf[..n]);
                    let healthy = response.contains("PONG");
                    ContainerStatus {
                        name,
                        healthy,
                        detail: response.trim().to_string(),
                    }
                }
                Err(e) => ContainerStatus {
                    name,
                    healthy: false,
                    detail: format!("read failed: {e}"),
                },
            }
        }
        Err(e) => ContainerStatus {
            name,
            healthy: false,
            detail: format!("unreachable: {e}"),
        },
    }
}

async fn ping_containers() -> Json<PingResponse> {
    let (vultiserver, networking, redis) =
        tokio::join!(check_vultiserver(), check_networking(), check_redis());

    Json(PingResponse {
        containers: vec![vultiserver, networking, redis],
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/ping", get(ping_containers));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("basalt-admin-internal listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
