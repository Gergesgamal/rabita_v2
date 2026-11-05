use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use crate::error::AppError;
use uuid::Uuid;

pub struct S3Manager {
    client: S3Client,
}

impl S3Manager {
    pub fn new(client: S3Client) -> Self {
        S3Manager { client }
    }

    /// Generate presigned PUT URL for uploading to staging bucket.
    pub async fn get_presigned_upload_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: u64,
    ) -> Result<String, AppError> {
        let presigned = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(
                aws_sdk_s3::presigning::config::PresigningConfig::builder()
                    .expires_in(std::time::Duration::from_secs(expires_in))
                    .build()
                    .map_err(|e| AppError::new(
                        crate::error::ErrorCode::InternalError,
                        &format!("Presigning error: {}", e),
                    ))?,
            )
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 error: {}", e),
            ))?;

        Ok(presigned.uri().to_string())
    }

    /// Copy object from staging to media bucket.
    pub async fn promote_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> Result<(), AppError> {
        self.client
            .copy_object()
            .copy_source(format!("{}/{}", source_bucket, source_key))
            .bucket(dest_bucket)
            .key(dest_key)
            .send()
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 copy error: {}", e),
            ))?;

        // Delete from staging
        self.client
            .delete_object()
            .bucket(source_bucket)
            .key(source_key)
            .send()
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 delete error: {}", e),
            ))?;

        Ok(())
    }

    /// Move object to quarantine bucket.
    pub async fn quarantine_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        quarantine_bucket: &str,
    ) -> Result<(), AppError> {
        let quarantine_key = source_key.replace("staging/", "quarantine/");

        self.client
            .copy_object()
            .copy_source(format!("{}/{}", source_bucket, source_key))
            .bucket(quarantine_bucket)
            .key(&quarantine_key)
            .send()
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 copy error: {}", e),
            ))?;

        // Delete from staging
        self.client
            .delete_object()
            .bucket(source_bucket)
            .key(source_key)
            .send()
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 delete error: {}", e),
            ))?;

        Ok(())
    }

    /// Get public URL for media (no signing needed for public objects).
    pub fn get_public_url(&self, bucket: &str, key: &str) -> String {
        format!("https://{}.s3.amazonaws.com/{}", bucket, key)
    }

    /// Generate presigned GET URL for private/preview media.
    pub async fn get_presigned_download_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: u64,
    ) -> Result<String, AppError> {
        let presigned = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(
                aws_sdk_s3::presigning::config::PresigningConfig::builder()
                    .expires_in(std::time::Duration::from_secs(expires_in))
                    .build()
                    .map_err(|e| AppError::new(
                        crate::error::ErrorCode::InternalError,
                        &format!("Presigning error: {}", e),
                    ))?,
            )
            .await
            .map_err(|e| AppError::new(
                crate::error::ErrorCode::InternalError,
                &format!("S3 error: {}", e),
            ))?;

        Ok(presigned.uri().to_string())
    }
}
