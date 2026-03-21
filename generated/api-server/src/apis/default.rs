use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum HealthResponse {
    /// All dependencies are healthy.
    Status200_AllDependenciesAreHealthy
    (models::PingResponse),
    /// One or more dependencies are unhealthy.
    Status503_OneOrMoreDependenciesAreUnhealthy
    (models::PingResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PingResponse {
    /// Service is alive.
    Status200_ServiceIsAlive
    (String)
}




/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Dependency health check.
    ///
    /// Health - GET /health
    async fn health(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<HealthResponse, E>;

    /// Liveness check.
    ///
    /// Ping - GET /ping
    async fn ping(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<PingResponse, E>;
}
