use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Todo {
    pub id: i32,
    pub text: String,
}
