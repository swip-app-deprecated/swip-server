use juniper::FieldResult;
use juniper::RootNode;

use database::models_init::*;

use database::models::users::*;
use database::models::parties::*;

use database::queries::users::*;
use database::queries::parties::*;

use graphql::executor::GraphQLExecutor;

use graphql::schema::users::*;
use graphql::schema::parties::*;


// Query that does not change things in the db
pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {

    // User query fields
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

    // Party query fields
    field party(&executor, uuid: String) -> FieldResult<Party> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_find_party_by_uuid(&conn, &uuid)?)
    }

    field parties(&executor,
                filter: Option<PartiesFilterParams>,
                paging: Option<PagingParams>
               ) -> FieldResult<PartyConnection> {

        let conn = executor.context().db_pool.get()?;
        let filter = filter.unwrap_or_default();
        let paging = paging.unwrap_or_default();

        let res = db_find_parties(&conn, &filter, &paging)?;

        Ok(
            PartyConnection {
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

// Query that does change things in the db
pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {

    // User mutation field
    field createUser(&executor, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_create_user(&conn, &user)?)
    }

    field updateUser(&executor, uuid: String, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_update_user(&conn, &uuid, &user)?)
    }

    // Party mutation field
    field createParty(&executor, party: NewParty) -> FieldResult<Party> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_create_party(&conn, &party)?)
    }

    field updateParty(&executor, uuid: String, party: NewParty) -> FieldResult<Party> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_update_party(&conn, &uuid, &party)?)
    }
});

// Create type Schema with Query and Mutation structs
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

// Create schema for graphql executor
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
