use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub link: String,
    pub keywords: Vec<String>,
}

#[derive(Default)]
pub struct LinkBuilder {
    link: String,
    keywords: Vec<String>,
}

impl Link {
    pub fn builder() -> LinkBuilder {
        LinkBuilder::default()
    }
}

impl LinkBuilder {
    pub fn link(mut self, link: &str) -> Self {
        self.link = String::from(link);
        self
    }
    pub fn keyword(mut self, keyword: &str) -> Self {
        self.keywords.push(String::from(keyword));
        self
    }
    pub fn build(self) -> Link {
        Link {
            link: self.link,
            keywords: self.keywords,
        }
    }
}
