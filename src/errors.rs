
#[derive(Debug)]
pub struct CustomError {
    status: i32,
    msg: String
}

impl CustomError {
    pub fn new(s: i32, m: String) -> Self {
        Self { status: s, msg: m }
    }
}
