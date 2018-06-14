use juniper::FieldResult;
use juniper::RootNode;

use database::models::*;
use graphql::executor::GraphQLExecutor;
use database::queries::*;

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct UserConnection {
    #[graphql(description = "This contains the User results")]
    pub edges: Vec<User>,
    #[graphql(name = "pageInfo")]
    pub page_info: PageInfo,
    pub cursor: Option<String>,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, uuid: String) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_find_user_by_uuid(&conn, &uuid)?)
    }

    field users(&executor,
                filter: Option<UsersFilterParams>,
                paging: Option<PagingParams>
               ) -> FieldResult<UserConnection> {

        let conn = executor.context().db_pool.get()?;
        let filter = filter.unwrap_or_default();
        let paging = paging.unwrap_or_default();

        let res = db_find_users(&conn, &filter, &paging)?;

        Ok(
            UserConnection {
                edges: res.items,
                page_info: PageInfo {
                    start_cursor: None,
                    end_cursor: None,
                    has_next_page: res.has_more,
                },
                cursor: res.cursor,
            }
        )
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {
    field createUser(&executor, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_create_user(&conn, &user)?)
    }
    field updateUser(&executor, uuid: String, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_update_user(&conn, &uuid, &user)?)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
