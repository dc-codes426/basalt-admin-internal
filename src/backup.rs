use std::time::Duration;

use crate::MinioConfig;

/// External S3-compatible endpoint for storing backups.
#[derive(Clone)]
pub struct BackupConfig {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub secure: bool,
    /// Path to the auth SQLite database file (mounted read-only).
    pub auth_db_path: String,
    /// How often to run backups.
    pub interval: Duration,
}

impl BackupConfig {
    /// Returns None if backup is not configured (BACKUP_ENDPOINT not set or empty).
    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("BACKUP_ENDPOINT").ok().filter(|s| !s.is_empty())?;
        let bucket = std::env::var("BACKUP_BUCKET").unwrap_or_else(|_| "basalt-backups".into());
        let access_key = std::env::var("BACKUP_ACCESS_KEY").ok().filter(|s| !s.is_empty())?;
        let secret_key = std::env::var("BACKUP_SECRET_KEY").ok().filter(|s| !s.is_empty())?;
        let secure = endpoint.starts_with("https://");
        let auth_db_path =
            std::env::var("BACKUP_AUTH_DB_PATH").unwrap_or_else(|_| "/auth-data/auth.db".into());
        let interval_secs: u64 = std::env::var("BACKUP_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600); // default: hourly

        Some(Self {
            endpoint,
            bucket,
            access_key,
            secret_key,
            secure,
            auth_db_path,
            interval: Duration::from_secs(interval_secs),
        })
    }
}

/// Run periodic backups: auth DB + MinIO vault objects → external S3.
pub async fn periodic_backup(backup: BackupConfig, source_minio: MinioConfig) {
    let mut interval = tokio::time::interval(backup.interval);

    // Skip the immediate first tick — let services stabilize.
    interval.tick().await;

    loop {
        interval.tick().await;
        tracing::info!("starting backup cycle");

        let dest = match build_client(&backup) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("failed to build backup S3 client: {e}");
                continue;
            }
        };

        ensure_bucket(&dest, &backup.bucket).await;

        // 1. Back up the auth database
        backup_auth_db(&dest, &backup).await;

        // 2. Mirror MinIO vault objects
        backup_minio_objects(&dest, &backup, &source_minio).await;

        tracing::info!("backup cycle complete");
    }
}

fn build_client(config: &BackupConfig) -> Result<minio_rsc::Minio, String> {
    let provider =
        minio_rsc::provider::StaticProvider::new(&config.access_key, &config.secret_key, None);
    let endpoint = config
        .endpoint
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    minio_rsc::Minio::builder()
        .endpoint(endpoint)
        .provider(provider)
        .secure(config.secure)
        .build()
        .map_err(|e| format!("{e}"))
}

fn build_source_client(config: &MinioConfig) -> Result<minio_rsc::Minio, String> {
    let provider =
        minio_rsc::provider::StaticProvider::new(&config.access_key, &config.secret_key, None);
    let endpoint = config
        .endpoint
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let secure = config.endpoint.starts_with("https://");
    minio_rsc::Minio::builder()
        .endpoint(endpoint)
        .provider(provider)
        .secure(secure)
        .build()
        .map_err(|e| format!("{e}"))
}

async fn ensure_bucket(client: &minio_rsc::Minio, bucket: &str) {
    match client.make_bucket(bucket, false).await {
        Ok(_) => tracing::info!("created backup bucket: {bucket}"),
        Err(_) => {} // already exists
    }
}

async fn backup_auth_db(dest: &minio_rsc::Minio, config: &BackupConfig) {
    let data = match tokio::fs::read(&config.auth_db_path).await {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("failed to read auth DB at {}: {e}", config.auth_db_path);
            return;
        }
    };

    let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
    let key = format!("auth-db/{timestamp}.db");
    let size = data.len();

    match dest
        .put_object(&config.bucket, &key, data.into())
        .await
    {
        Ok(_) => tracing::info!("backed up auth DB ({size} bytes) → {key}"),
        Err(e) => tracing::error!("failed to upload auth DB: {e}"),
    }
}

async fn backup_minio_objects(
    dest: &minio_rsc::Minio,
    config: &BackupConfig,
    source_config: &MinioConfig,
) {
    let source = match build_source_client(source_config) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("failed to build source MinIO client: {e}");
            return;
        }
    };

    let objects = match source
        .list_objects(&source_config.bucket, Default::default())
        .await
    {
        Ok(result) => result.contents,
        Err(e) => {
            tracing::error!("failed to list source MinIO objects: {e}");
            return;
        }
    };

    let mut backed_up = 0u64;
    for obj in &objects {
        let key = &obj.key;
        let dest_key = format!("vaults/{key}");

        // Download from source
        let data = match source.get_object(&source_config.bucket, key).await {
            Ok(resp) => resp.bytes().await.unwrap_or_default(),
            Err(e) => {
                tracing::warn!("failed to download {key} from source MinIO: {e}");
                continue;
            }
        };

        // Upload to backup destination
        match dest
            .put_object(&config.bucket, &dest_key, data.into())
            .await
        {
            Ok(_) => backed_up += 1,
            Err(e) => tracing::warn!("failed to upload {dest_key} to backup: {e}"),
        }
    }

    tracing::info!("backed up {backed_up}/{} vault objects", objects.len());
}
