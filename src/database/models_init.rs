// Database query result
pub struct DBQueryResult<T> {
    pub items: Vec<T>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

// Graphql pagination PageInfo (https://graphql.org/learn/pagination/)
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

// Pagination parameters struct
#[derive(GraphQLInputObject)]
pub struct PagingParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

// Pagination parameters impl
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

// Default parameters for pagination
impl Default for PagingParams {
    fn default() -> Self {
        PagingParams {
            limit: Some(DEFAULT_PAGE_SIZE),
            cursor: None,
        }
    }
}
