use serde::Deserialize;
use crate::network::Server;

use ratatui::text::Text;

#[derive(Deserialize, Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
/// This is what the Server returns to us when we search it;
/// a detailed link to a post, not bound to any particular Server.
pub struct SearchResultHeader {
    pub title: String,
    pub postid: u16,
}

/// This is basically a detailed pointer to a specific post, on a specific Server.
#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct SearchResult {
    pub header: SearchResultHeader,
    pub server: Server,
}


impl SearchResult {
    pub fn new(header: SearchResultHeader, server: Server) -> Self {
        Self {header, server}
    }
}

// https://www.reddit.com/r/rust/comments/7zm0j2/intofrom_for_nonconsuming_conversions/
impl<'a> From<&'a SearchResultHeader> for Text<'a> {
    fn from(value: &'a SearchResultHeader) -> Self {
        return Self::from(value.title.as_str());
    }
}
