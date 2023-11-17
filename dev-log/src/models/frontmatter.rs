use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: String,
    file_name: String,
    description: String,
    posted: String,
    order: u32,
}

impl Frontmatter {
    pub fn order(&self) -> u32 {
        self.order
    }
}