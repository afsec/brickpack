pub(super) mod applets;
pub(super) mod departments;
pub(super) mod permissions;
pub(super) mod runner;
pub(super) mod statuses;
pub(super) mod users;

use design_scaffold::paging::{PagingLimit, PagingOffset};
use serde::Deserialize;

const X_TOTAL_COUNT: &'static str = "X-Total-Count";

const REQUEST_BODY_MAX_SIZE: u64 = 1 * 1024 * 1024; // 1 MBytes

#[derive(Debug, Default, Deserialize)]
struct Paging {
    #[serde(default)]
    limit: PagingLimit,
    #[serde(default)]
    offset: PagingOffset,
}
impl Paging {
    fn get_limit(&self) -> &PagingLimit {
        &self.limit
    }
    fn get_offset(&self) -> &PagingOffset {
        &self.offset
    }
}

impl TryFrom<String> for Paging {
    type Error = design_scaffold::Error;
    fn try_from(paging_str: String) -> Result<Self, Self::Error> {
        Ok(serde_qs::from_str(&paging_str)?)
    }
}
