use database::models::*;
use graphql::context::*;
use chrono::prelude::*;
use uuid::Uuid;
use juniper::Executor;
use diesel;

pub fn create_user(
    executor: &Executor<Context>,
    user_username: String,
    user_password: String
) -> Option<User> {
    use diesel::prelude::*;
    use database::models::*;
    use database::schema::users::dsl::*;

    let new_user = NewUser {
        username: user_username,
        password: user_password
    };

    let context = executor.context();
    let ref database = context.database;

    match diesel::insert(&new_user).into(users).get_result(&**database) {
        Ok(user) => Some(user),
        Err(_) => None
    }
}

pub fn get_all_users(executor: &Executor<Context>) -> Vec<User> {
    use diesel::prelude::*;
    use database::models::*;
    use database::schema::users::dsl::*;

    let context = executor.context();
    let ref database = context.database;

    let results = users
        // .filter(published.eq(true))
        .load::<User>(&**database)
        .expect("Error loading users");

    results
}

pub fn get_user_by_id<'a>(executor: &'a Executor<Context>, user_id: String)
    -> Option<User>
{
    use diesel::prelude::*;
    use database::models::*;
    use database::schema::users::dsl::*;

    let user_id = match Uuid::parse_str(&user_id[..]) {
        Ok(value) => value,
        Err(_) => {
            return None
        }
    };
    let context = executor.context();
    let ref database = context.database;
    let results = users
        .filter(published.eq(true))
        .filter(id.eq(user_id))
        .load::<User>(&**database)
        .expect("Error loading user");

    match results.get(0) {
        Some(user) => Some(user.to_owned()),
        _ => None
    }
}
