use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Invoice {
    pub store_id: Uuid,
    pub invoice_number: String,
    pub order_id: Uuid,
    pub merchant_name: String,
    pub merchant_tax_id: Option<String>,
    pub consumer_email: String,
    pub invoice_date: String,
    pub total_egp: i64,
}
