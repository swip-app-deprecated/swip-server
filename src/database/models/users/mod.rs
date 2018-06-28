use chrono;
use database::schema::users;

// Graphql query filter parameters for user
#[derive(GraphQLInputObject)]
pub struct UsersFilterParams {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

// Default filter parameters for user
impl Default for UsersFilterParams {
    fn default() -> Self {
        UsersFilterParams {
            uuid: None,
            name: None,
            active: None,
        }
    }
}

// User struct
#[derive(GraphQLObject)]
#[graphql(description = "A simple user")]
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// New user in database
#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct DbNewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub active: bool,
}

// New user
#[derive(AsChangeset)]
#[table_name = "users"]
#[derive(GraphQLInputObject)]
#[graphql(description = "A simple user")]
pub struct NewUser {
    pub name: String,
    pub active: bool,
}
