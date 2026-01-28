use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::{entities::{dtos::{product_dtos::ProductChangeQuantityDTO, report_dtos::ReportViewDTO}, tb_report::{self, ActiveModel}}, services::{service_product, service_return_reason}};

pub async fn get_all_reports(
    database: &DatabaseConnection,
) -> Vec<ReportViewDTO> {

    let reports = tb_report::Entity::find().all(database).await;

    let products = {
   
        let mut  vec = Vec::new();

        for model in reports.unwrap() {

            let product = service_product::find_product_by_id(database, model.product_id).await.unwrap();
            let reason = service_return_reason::find_reason_by_id(database, model.reason_id).await.unwrap();

            vec.push(
                ReportViewDTO::new(
                    model.id,
                    match model.change_type {
                        1 => true,
                        _ => false
                    },
                    model.quantity,
                    product.name,
                    reason.name,
                    model.date.to_string()
            ));

        }

        vec
   
    };

    return products;

}

pub async fn create_report(
    database: &DatabaseConnection,
    product_change_quantity_dto: ProductChangeQuantityDTO
) -> Result<&'static str, ()> {

    let report = ActiveModel {
        product_id: ActiveValue::Set(*product_change_quantity_dto.get_id()),
        reason_id: ActiveValue::Set(*product_change_quantity_dto.get_reason_id()),
        change_type: ActiveValue::Set(match product_change_quantity_dto.get_change_type() {
            &true => 1,
            &false => 0
        }),
        quantity: ActiveValue::Set(*product_change_quantity_dto.get_quantity()),
        date: ActiveValue::Set(Utc::now().naive_local()),
        ..Default::default()
    };

    let result = tb_report::Entity::insert(report).exec(database).await;

    match result {

        Ok(_) => Ok("Relatório criado com sucesso"),
        Err(_) => Err(())

    }

}