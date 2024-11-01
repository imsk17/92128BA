#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Movie {
    pub id: String,
    pub name: String,
    pub year: u16,
    pub was_good: bool,
}
