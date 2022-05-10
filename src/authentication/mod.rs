#[derive(Clone)]
pub enum Authentication {
    Basic { username: String, password: String },
    None
}