use crate::error::AppError;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct NotificationManager;

impl NotificationManager {
    /// Log notification send per D-102
    pub async fn log_notification(
        db: &PgPool,
        consumer_id: Option<Uuid>,
        order_id: Option<Uuid>,
        notification_type: &str,
        email_address: &str,
        ses_message_id: &str,
    ) -> Result<(), AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.notification_log (id, consumer_id, order_id, notification_type, email_address, delivery_status, ses_message_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'sent', $6, $7, $7)
            "#,
        )
        .bind(&id)
        .bind(&consumer_id)
        .bind(&order_id)
        .bind(notification_type)
        .bind(email_address)
        .bind(ses_message_id)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }

    /// Build order confirmation email per D-102
    pub fn build_order_confirmation_email(
        store_name: &str,
        public_order_number: &str,
        total_egp: i64,
        recipient_email: &str,
    ) -> EmailNotification {
        let total_egp_str = format!("{},{:.2}", total_egp / 100, total_egp % 100);

        EmailNotification {
            notification_type: "order_placed".to_string(),
            recipient_email: recipient_email.to_string(),
            subject: format!("Order {} confirmed", public_order_number),
            html_body: format!(
                r#"<html><body><h1>Order Confirmation</h1>
                <p>Thank you for your order at {}!</p>
                <p><strong>Order Number:</strong> {}</p>
                <p><strong>Total:</strong> {} ج.م</p>
                </body></html>"#,
                store_name, public_order_number, total_egp_str
            ),
        }
    }

    /// Build refund approval email per D-102
    pub fn build_refund_approval_email(
        public_order_number: &str,
        refund_amount_egp: i64,
        reason: &str,
        recipient_email: &str,
    ) -> EmailNotification {
        let amount_str = format!("{},{:.2}", refund_amount_egp / 100, refund_amount_egp % 100);

        EmailNotification {
            notification_type: "refund_approved".to_string(),
            recipient_email: recipient_email.to_string(),
            subject: format!("Refund approved for order {}", public_order_number),
            html_body: format!(
                r#"<html><body><h1>Refund Approved</h1>
                <p>Your refund for order {} has been approved.</p>
                <p><strong>Amount:</strong> {} ج.م</p>
                <p><strong>Reason:</strong> {}</p>
                </body></html>"#,
                public_order_number, amount_str, reason
            ),
        }
    }
}
