use graphql::context::*;
use database::models::*;
use graphql::resolvers::*;

pub struct QueryRoot {}
pub struct MutationRoot {}

graphql_object!(QueryRoot: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field user(
        &executor
        id: String as "id of the user"
    ) -> Option<User> {
        users::get_user_by_id(executor, id)
    }

    field users(&executor) -> Vec<User> {
        users::get_all_users(executor)
    }
});

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    description: "The root mutation object of the schema"

    field create_user(
        &executor,
        username,
        password
    ) -> Option<User> as "Creates a new user" {
        users::create_user(
            executor,
            username,
            password
        )
    }
});

graphql_object!(User: Context as "User" |&self| {
    description: "Represents a user or lecture"

    field id() -> String {
        self.id.to_string()
    }

    field username() -> String {
        self.username.to_string()
    }

    field password() -> String {
        self.password.to_string()
    }
});
