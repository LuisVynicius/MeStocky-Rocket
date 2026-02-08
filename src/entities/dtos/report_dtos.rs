use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct ReportViewDTO {
    id: u64,
    change_type: bool,
    quantity: u64,
    product: String,
    reason: String,
    date: String,
}
