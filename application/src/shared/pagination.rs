// application/pagination.rs
use serde::Deserialize;

pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_PAGE_SIZE: u32 = 10;
pub const MAX_PAGE_SIZE: u32 = 100;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn to_str(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}
