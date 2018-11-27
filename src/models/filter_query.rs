#[derive(Serialize, Deserialize, Clone)]
pub struct FilterQuery {
    pub q: String,
    pub count: u8,
    pub result_type: String,
}
