use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use database::models;
use database::models_init::{DBQueryResult, PagingParams};

// Generate uuid v4 for parties
fn generate_uuid() -> String {
    let uuid: String = format!("{}", Uuid::new_v4());
    uuid
}

// Create party in database
pub fn db_create_party(
    conn: &MysqlConnection,
    new_party: &models::parties::NewParty,
) -> Result<models::parties::Party, String> {
    use database::schema::parties::dsl;

    let uuid = generate_uuid();

    let party = models::parties::DbNewParty {
        uuid: &uuid,
        title: &new_party.title
    };

    diesel::insert_into(dsl::parties)
        .values(&party)
        .execute(&*conn)
        .expect("Error inserting party");

    db_find_party_by_uuid(&conn, &uuid)
}

// Update party in database
pub fn db_update_party(
    conn: &MysqlConnection,
    uuid: &str,
    party: &models::parties::NewParty,
) -> Result<models::parties::Party, String> {
    use database::schema::parties::dsl;
    let res = dsl::parties.filter(dsl::uuid.eq(&uuid));
    match diesel::update(res).set(party).execute(&*conn) {
        Ok(_) => db_find_party_by_uuid(&conn, &uuid),
        Err(err) => Err(format!("Unable to update party {}", err)),
    }
}

// Find party by a uuid v4
pub fn db_find_party_by_uuid(
    conn: &MysqlConnection,
    uuid: &str,
) -> Result<models::parties::Party, String> {
    use database::schema::parties::dsl;

    let mut items = dsl::parties
        .filter(dsl::uuid.eq(&uuid))
        .load::<models::parties::Party>(&*conn)
        .expect("Error loading party");

    match items.pop() {
        Some(item) => Ok(item),
        None => Err("No party found".to_owned()),
    }
}

// Find party by parties query filter
pub fn db_find_parties(
    conn: &MysqlConnection,
    filter: &models::parties::PartiesFilterParams,
    paging: &PagingParams,
) -> Result<DBQueryResult<models::parties::Party>, String> {
    use database::schema::parties::dsl;

    let limit = i64::from(paging.get_limit());
    let current_cursor = NaiveDateTime::from_timestamp(paging.get_cursor(), 0);

    let mut query = dsl::parties.into_boxed().order(dsl::created_at);

    if let Some(ref res) = filter.uuid {
        query = query.filter(dsl::uuid.eq(res));
    }

    if let Some(ref res) = filter.title {
        query = query.filter(dsl::title.eq(res));
    }

    let items = query
        .filter(dsl::created_at.gt(current_cursor))
        .limit(limit)
        .load::<models::parties::Party>(&*conn)
        .expect("Error loading parties");

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
