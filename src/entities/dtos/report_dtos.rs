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

#[derive(Serialize, Deserialize)]
pub struct ReportUpdateDTO {
    id: u64,
    reason_id: u64
}

impl ReportUpdateDTO {
    pub fn get_id(&self) -> &u64 {
        &self.id
    }

    pub fn get_reason_id(&self) -> &u64 {
        &self.reason_id
    }
}