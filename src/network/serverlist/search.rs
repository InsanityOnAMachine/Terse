use crate::network::SearchResultHeader;
use crate::network::SearchResult;
use crate::network::Server;

use crate::posts::Post;

use super::ServerList;

use anyhow::Error;


impl ServerList {
	pub fn search(&self, server: &Server, query: String) -> Result<Vec<SearchResult>, Error> {
        let headers = self.client.get(server.url_with_params("search", format!("query={}", query)))
            .send()?
            .json::<Vec<SearchResultHeader>>()?;

        Ok(headers.into_iter().map(|x| SearchResult::new(x, server.clone())).collect())
    }

    pub fn get_post(&self, server: &Server, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        Ok(
            self.client.get(server.url_with_params("posts", format!("id={id}")))
            .send()?
            .json::<Post>()?
        )
    }

    pub fn get_result_post(&mut self, search_result: &SearchResult) -> Post {
        self.get_post(&search_result.server, search_result.header.postid).unwrap()
    }
}