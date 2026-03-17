use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Film {
    pub name: String,
    pub director: Option<String>,
    pub rating: Option<u8>,
    pub watched: bool,
    pub comments: Option<String>,
}

impl Film {
    pub fn new(name: String) -> Self {
        Self {
            name,
            director: None,
            rating: None,
            watched: false,
            comments: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Unwatched,
    Watched,
}
