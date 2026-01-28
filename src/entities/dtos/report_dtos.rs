use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReportViewDTO {
    id: u64,
    change_type: bool,
    quantity: u64,
    product: String,
    reason: String,
    date: String
}

impl ReportViewDTO {

    pub fn new(
        id: u64,
        change_type: bool,
        quantity: u64,
        product: String,
        reason: String,
        date: String
    ) -> Self {

        Self {
            id,
            change_type,
            quantity,
            product,
            reason,
            date
        }

    }

}