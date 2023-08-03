use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PagingLimit(u32);
impl Default for PagingLimit {
    fn default() -> Self {
        PagingLimit(20)
    }
}
impl PagingLimit {
    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct PagingOffset(u32);
impl Default for PagingOffset {
    fn default() -> Self {
        PagingOffset(0)
    }
}
impl PagingOffset {
    pub fn get(&self) -> u32 {
        self.0
    }
}
