use database::schema::users;
use chrono;

pub struct DBQueryResult<T> {
    pub items: Vec<T>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

#[derive(GraphQLObject)]
#[graphql(description = "Page info")]
pub struct PageInfo {
    #[graphql(name = "startCursor")]
    pub start_cursor: Option<String>,
    #[graphql(name = "endCursor")]
    pub end_cursor: Option<String>,
    #[graphql(name = "hasNextPage")]
    pub has_next_page: bool,
}

const DEFAULT_PAGE_SIZE: i32 = 20;

#[derive(GraphQLInputObject)]
pub struct PagingParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl PagingParams {
    pub fn get_limit(&self) -> i32 {
        match self.limit {
            None => DEFAULT_PAGE_SIZE,
            Some(limit) => limit,
        }
    }

    pub fn get_cursor(&self) -> i64 {
        match self.cursor {
            None => 0,
            Some(ref cursor) => cursor.parse::<i64>().unwrap_or(0),
        }
    }
}

impl Default for PagingParams {
    fn default() -> Self {
        PagingParams {
            limit: Some(DEFAULT_PAGE_SIZE),
            cursor: None,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct UsersFilterParams {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

impl Default for UsersFilterParams {
    fn default() -> Self {
        UsersFilterParams {
            uuid: None,
            name: None,
            active: None,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature")]
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct DbNewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub active: bool,
}

#[derive(AsChangeset)]
#[table_name = "users"]
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature")]
pub struct NewUser {
    pub name: String,
    pub active: bool,
}
