use chrono::Local;
use sea_orm::{
    ActiveValue, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, Statement,
};

use crate::{
    entities::{
        dtos::{product_dtos::ProductChangeQuantityDTO, report_dtos::ReportViewDTO},
        tb_report::{self, ActiveModel},
    },
    errors::BackendError,
};

pub async fn get_all_reports(
    database: &DatabaseConnection,
) -> Result<Vec<ReportViewDTO>, BackendError> {
    let stmt = Statement::from_string(
        DbBackend::MySql,
        r#"
            SELECT
                tb_report.id,
                tb_report.change_type,
                tb_report.quantity,
                tb_product.name AS product,
                tb_reason.name AS reason,
	            CAST(
                    tb_report.date AS CHAR
                ) AS date
            FROM tb_report
            JOIN tb_product
                ON tb_product.id = tb_report.product_id 
            JOIN tb_reason
                ON tb_reason.id = tb_report.reason_id;
        "#,
    );

    let result = ReportViewDTO::find_by_statement(stmt).all(database).await;

    match result {
        Ok(reports) => Ok(reports),
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

pub async fn create_report(
    database: &DatabaseConnection,
    product_change_quantity_dto: ProductChangeQuantityDTO,
) -> Result<(), BackendError> {
    let report = ActiveModel {
        product_id: ActiveValue::Set(*product_change_quantity_dto.get_id()),
        reason_id: ActiveValue::Set(*product_change_quantity_dto.get_reason_id()),
        change_type: ActiveValue::Set(match product_change_quantity_dto.get_change_type() {
            &true => 1,
            &false => 0,
        }),
        quantity: ActiveValue::Set(*product_change_quantity_dto.get_quantity()),
        date: ActiveValue::Set(Local::now().naive_local()),
        ..Default::default()
    };

    let result = tb_report::Entity::insert(report).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}
