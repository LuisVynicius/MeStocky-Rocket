use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::{dtos::category_dtos::CategoryDTO, tb_category};

pub async fn get_all_categories(
    database: &DatabaseConnection
) -> Vec<CategoryDTO> {

    let categories = tb_category::Entity::find().all(database).await;

    categories.unwrap()
        .into_iter()
        .map(|model| CategoryDTO::new(model.id, model.name) )
        .collect()

}