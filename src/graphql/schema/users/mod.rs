use database::models::users::*;
use database::models_init::*;

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct UserConnection {
    #[graphql(description = "This contains the User results")]
    pub edges: Vec<User>,
    #[graphql(name = "pageInfo")]
    pub page_info: PageInfo,
    pub cursor: Option<String>,
}
