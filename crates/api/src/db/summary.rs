use crate::models::UploadSummary;
use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait SummaryDb {
    async fn create_summary(
        &self,
        user_id: Uuid,
        filename: &str,
        total_trips: i32,
        cost_actual: Decimal,
        cost_optimized: Decimal,
        savings: Decimal,
    ) -> Result<UploadSummary, sqlx::Error>;

    async fn get_summaries_by_user(&self, user_id: Uuid) -> Result<Vec<UploadSummary>, sqlx::Error>;
}

#[async_trait]
impl SummaryDb for PgPool {
    async fn create_summary(
        &self,
        user_id: Uuid,
        filename: &str,
        total_trips: i32,
        cost_actual: Decimal,
        cost_optimized: Decimal,
        savings: Decimal,
    ) -> Result<UploadSummary, sqlx::Error> {
        let summary = sqlx::query_as!(
            UploadSummary,
            r#"
            INSERT INTO upload_summaries (user_id, filename, total_trips, cost_actual, cost_optimized, savings)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, filename, total_trips, cost_actual, cost_optimized, savings, uploaded_at
            "#,
            user_id,
            filename,
            total_trips,
            cost_actual,
            cost_optimized,
            savings
        )
        .fetch_one(self)
        .await?;

        Ok(summary)
    }

    async fn get_summaries_by_user(&self, user_id: Uuid) -> Result<Vec<UploadSummary>, sqlx::Error> {
        let summaries = sqlx::query_as!(
            UploadSummary,
            r#"
            SELECT id, user_id, filename, total_trips, cost_actual, cost_optimized, savings, uploaded_at
            FROM upload_summaries
            WHERE user_id = $1
            ORDER BY uploaded_at DESC
            "#,
            user_id
        )
        .fetch_all(self)
        .await?;

        Ok(summaries)
    }
}