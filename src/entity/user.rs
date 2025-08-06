pub struct User {
    pub id: Option<String>,
    pub name: String,
}
impl User {
    pub fn new(id: Option<String>, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}
