use super::schema::links;

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub long: String,
    pub hits: i32,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct AddLink<'a> {
    pub short: &'a str,
    pub long: &'a str,
}
