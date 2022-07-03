#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub long: String,
    pub hits: i32,
}
