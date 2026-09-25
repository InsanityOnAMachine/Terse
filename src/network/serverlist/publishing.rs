use bytesize::ByteSize;
use time_format::DateFormat;
use indoc::writedoc;
use std::fmt::Formatter;
use std::fmt::Display;
use crate::posts::Post;
use serde::Deserialize;
use crate::network::Server;

use super::ServerList;

use anyhow::Error;


// Assuming you are logged in, and you get a valid post to send,
// This represents all responses the server can send back
// that you'd have no way of checking client-side
// Coincidentally, Result might not be a good suffix for this;
// It ain't a Result<>.
#[derive(Deserialize)]
pub enum PublishingResult {
    Published,
    PostTooLarge { max_post_size: u64, max_title_size: u64 },
    Timeout { time_remaining: u64 },
}

impl Display for PublishingResult {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::PostTooLarge { max_post_size, max_title_size }=> {
                // https://stackoverflow.com/questions/33334994/multi-line-string-in-rust-with-preserved-leading-whitespace
                writedoc!(
                    f,
                    "
                    The server rejected your post because part of it was too large;
                    Make sure:
                        - The content is within {}
                        - The title is within {} characters
                    ",
                    ByteSize::b(*max_post_size),
                    max_title_size
                )   
            }
            Self::Timeout { time_remaining } => {
                writedoc!(
                    f,
                    "
                    You need to wait {} until you can publish another post
                    ",
                    time_format::format_common_utc(*time_remaining as i64, DateFormat::Custom("%H hours and %M minutes")).unwrap_or(String::from("and I don't know how long 'cause I couldn't parse the time"))
                ) 
            }
            Self::Published => write!(f, "The post was published successfully")
        }
    }
}

impl ServerList {

    pub fn publish(&self, server: &Server, post: Post) -> Result<PublishingResult, Error> {
        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
        
        if !server.is_signed_in() {
            return Err(Error::msg("You aren't signed in to this server, so you cannot publish; try using the    trs --server login    command"))
        }

        // https://stackoverflow.com/questions/499591/are-https-urls-encrypted
        // So I can place the login info within the query! Yippee!
        Ok(
            self.client.post(server.url_with_params("posts", format!("user={}", serde_json::to_string(&server.login_info.clone().unwrap())?)))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&post)?)
            .send()?
            .json::<PublishingResult>()?
        )
    }

}