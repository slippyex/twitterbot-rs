
use crate::models::FilterQuery;

#[derive(Serialize, Deserialize)]
pub struct FilterRule {
    pub id: u8,
    pub query: FilterQuery,
    pub retweet: bool,
}
