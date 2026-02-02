use sea_orm::FromQueryResult;

#[derive(FromQueryResult)]
pub struct ExistsDTO {
    exist: bool
}

impl ExistsDTO {

    pub fn get_into_exist(self) -> bool {

        self.exist

    }

}