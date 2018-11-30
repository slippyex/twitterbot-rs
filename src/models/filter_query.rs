#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilterQuery {
    pub q: String,
    pub count: u32,
    pub result_type: String,
}
