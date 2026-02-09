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

impl ReportViewDTO {
    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn set_date(&mut self, date: String) {
        self.date = date;
    }
}
