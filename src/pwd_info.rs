use std::fmt::{Display, Formatter};

pub struct PwdInfo {
    pub password: String,
    pub scores: f64,
}

impl Display for PwdInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "password: {}  score: {:.3}", self.password, self.scores)
    }
}

impl PwdInfo {
    pub fn new(password: String, scores: f64) -> Self {
        PwdInfo { password, scores }
    }
}
