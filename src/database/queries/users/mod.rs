use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use database::models;
use database::models_init::{DBQueryResult, PagingParams};

// Generate uuid v4 for user
fn generate_uuid() -> String {
    let uuid: String = format!("{}", Uuid::new_v4());
    uuid
}

// Create user in database
pub fn db_create_user(
    conn: &MysqlConnection,
    new_user: &models::users::NewUser,
) -> Result<models::users::User, String> {
    use database::schema::users::dsl;

    let uuid = generate_uuid();

    let user = models::users::DbNewUser {
        uuid: &uuid,
        name: &new_user.name,
        active: true,
    };

    diesel::insert_into(dsl::users)
        .values(&user)
        .execute(&*conn)
        .expect("Error inserting user");

    db_find_user_by_uuid(&conn, &uuid)
}

// Update user in database
pub fn db_update_user(
    conn: &MysqlConnection,
    uuid: &str,
    user: &models::users::NewUser,
) -> Result<models::users::User, String> {
    use database::schema::users::dsl;
    let res = dsl::users.filter(dsl::uuid.eq(&uuid));
    match diesel::update(res).set(user).execute(&*conn) {
        Ok(_) => db_find_user_by_uuid(&conn, &uuid),
        Err(err) => Err(format!("Unable to update user {}", err)),
    }
}

// Find user by a uuid v4
pub fn db_find_user_by_uuid(
    conn: &MysqlConnection,
    uuid: &str,
) -> Result<models::users::User, String> {
    use database::schema::users::dsl;

    let mut items = dsl::users
        .filter(dsl::uuid.eq(&uuid))
        .load::<models::users::User>(&*conn)
        .expect("Error loading user");

    match items.pop() {
        Some(item) => Ok(item),
        None => Err("No user found".to_owned()),
    }
}

// Find user by users query filter
pub fn db_find_users(
    conn: &MysqlConnection,
    filter: &models::users::UsersFilterParams,
    paging: &PagingParams,
) -> Result<DBQueryResult<models::users::User>, String> {
    use database::schema::users::dsl;

    let limit = i64::from(paging.get_limit());
    let current_cursor = NaiveDateTime::from_timestamp(paging.get_cursor(), 0);

    let mut query = dsl::users.into_boxed().order(dsl::created_at);

    if let Some(ref res) = filter.active {
        query = query.filter(dsl::active.eq(res));
    }

    if let Some(ref res) = filter.uuid {
        query = query.filter(dsl::uuid.eq(res));
    }

    if let Some(ref res) = filter.name {
        query = query.filter(dsl::name.eq(res));
    }

    let items = query
        .filter(dsl::created_at.gt(current_cursor))
        .limit(limit)
        .load::<models::users::User>(&*conn)
        .expect("Error loading users");

    let next_cursor = match items.last() {
        Some(item) => Some(format!("{}", item.created_at.timestamp())),
        None => None,
    };

    let has_more = (items.len() as i64) == limit;

    Ok(DBQueryResult {
        items,
        has_more,
        cursor: next_cursor,
    })
}
