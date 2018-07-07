use database::models_init::*;
use database::models::parties::*;

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct PartyConnection {
    #[graphql(description = "This contains the Party results")]
    pub edges: Vec<Party>,
    #[graphql(name = "pageInfo")]
    pub page_info: PageInfo,
    pub cursor: Option<String>,
}