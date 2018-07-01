use database::schema::parties;
use chrono;

// Graphql query filter parameters for user
#[derive(GraphQLInputObject)]
pub struct PartiesFilterParams {
    pub uuid: Option<String>,
    pub title: Option<String>,
}

// Default filter parameters for user
impl Default for PartiesFilterParams {
    fn default() -> Self {
        PartiesFilterParams {
            uuid: None,
            title: None,
        }
    }
}

// Party struct
#[derive(GraphQLObject)]
#[graphql(description = "A simple user")]
#[derive(Serialize, Queryable)]
pub struct Party {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// New user in database
#[derive(Deserialize, Insertable)]
#[table_name = "parties"]
pub struct DbNewParty<'a> {
    pub uuid: &'a str,
    pub title: &'a str,
}

// New user
#[derive(AsChangeset)]
#[table_name = "parties"]
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature")]
pub struct NewParty {
    pub title: String
}
