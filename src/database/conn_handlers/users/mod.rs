use database::connection::*;

// User creation struct
pub struct CreateUser {
    pub name: String,
}

// Result type for user creation
impl Message for CreateUser {
    type Result = Result<models::User, Error>;
}

// Handle "CreateUser" in executor
impl Handler<CreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use database::queries::db_create_user;
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let user = models::NewUser {
            name: msg.name,
            active: true,
        };
        Ok(db_create_user(&conn, &user).unwrap())
    }
}
