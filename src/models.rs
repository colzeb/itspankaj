use crate::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub date_create: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub date_created: &'a str,
}
