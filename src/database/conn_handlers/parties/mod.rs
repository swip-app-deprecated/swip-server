use database::connection::*;

// Party creation struct
pub struct CreateParty {
    pub title: String,
}

// Result type for party creation
impl Message for CreateParty {
    type Result = Result<models::Party, Error>;
}

// Handle "CreateParty" in executor
impl Handler<CreateParty> for DbExecutor {
    type Result = Result<models::Party, Error>;

    fn handle(&mut self, msg: CreateParty, _: &mut Self::Context) -> Self::Result {
        use database::queries::db_create_party;
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let party = models::NewParty {
            title: msg.title,
            active: true,
        };
        Ok(db_create_party(&conn, &party).unwrap())
    }
}
