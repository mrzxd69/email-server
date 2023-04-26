use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Body {
    pub email: String
}
