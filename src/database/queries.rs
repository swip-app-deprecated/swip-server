use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::models::*;
use chrono::NaiveDateTime;

fn generate_uuid() -> String {
    let uuid: String = format!("{}", Uuid::new_v4());
    uuid
}

pub fn db_create_user(conn: &MysqlConnection, new_user: &NewUser) -> Result<User, String> {
    use database::schema::users::dsl;

    let uuid = generate_uuid();

    let user = DbNewUser {
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

pub fn db_update_user(conn: &MysqlConnection, uuid: &str, user: &NewUser) -> Result<User, String> {
    use database::schema::users::dsl;
    let res = dsl::users.filter(dsl::uuid.eq(&uuid));
    match diesel::update(res).set(user).execute(&*conn) {
        Ok(_) => db_find_user_by_uuid(&conn, &uuid),
        Err(err) => Err(format!("Unable to update user {}", err)),
    }
}

pub fn db_find_user_by_uuid(conn: &MysqlConnection, uuid: &str) -> Result<User, String> {
    use database::schema::users::dsl;

    let mut items = dsl::users
        .filter(dsl::uuid.eq(&uuid))
        .load::<User>(&*conn)
        .expect("Error loading user");

    match items.pop() {
        Some(item) => Ok(item),
        None => Err(format!("No user found")),
    }
}

pub fn db_find_users(
    conn: &MysqlConnection,
    filter: &UsersFilterParams,
    paging: &PagingParams,
) -> Result<DBQueryResult<User>, String> {
    use database::schema::users::dsl;

    let limit = paging.get_limit() as i64;
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
        .load::<User>(&*conn)
        .expect("Error loading users");

    let next_cursor = match items.last() {
        Some(item) => Some(format!("{}", item.created_at.timestamp())),
        None => None,
    };

    let has_more = (items.len() as i64) == limit;

    Ok(DBQueryResult {
        items: items,
        cursor: next_cursor,
        has_more: has_more,
    })
}
