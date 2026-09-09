use serde::{Serialize, Deserialize};

use colored::Colorize;
use url::Url;

use std::fmt::{Display, Write};

use crate::network::LoginInfo;

// RIP: use std::borrow::Borrow; I have no idea why you even existed or if I even wrote you.

#[derive(Clone, Serialize, Deserialize)]
#[serde(from="ServerSerializer")]
#[serde(into="ServerSerializer")]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub struct Server {
    #[serde(with = "url_serde")]
    url: Url, 
    pub login_info: Option<LoginInfo>,
}

impl Server {

    pub fn new(url: Url, login_info: Option<LoginInfo>) -> Self {
        Self {url, login_info}
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn set_login_info(&mut self, login_info: LoginInfo) {
        self.login_info = Some(login_info)
    }

    pub fn is_signed_in(&self) -> bool {
        self.login_info.is_some()
    }

    pub fn user_string(&self) -> String {
        return format!(
            "{} on {}",
            self.login_info.clone().map_or(String::from("(Not signed in)"), |x| x.email),
            self.url(),
        )
    }

    // I opt to use &strs instead of Options as the arguments, although setting query = None is really fun...
    // But it'd be a lotta extra text that could just be an empty &str
    // https://stackoverflow.com/questions/55079070/how-to-accept-str-string-and-string-in-a-single-function
    pub(super) fn url_with_params(&self, route: &str, query: impl AsRef<str>) -> Url
    {
        let mut url = self.url.clone();
        url.set_path(route);
        url.set_query(Some(query.as_ref()));
        return url;
    }

    pub fn as_string(&self, show_password: bool, selected: bool) -> String {
        let mut s = String::new();

        if selected {
            writeln!(s, "{}", Colorize::yellow(self.url().as_str())).expect("String writing should always work");
        } else {
            writeln!(s, "{}", self.url().as_str()).expect("String writing should always work");
        }
        writeln!(s, "{}", self.login_info.clone().map_or(
            String::from("(Not signed in)"), 
            |x| format!("Signed in as {}", x.as_string(show_password)))
        ).expect("String writing should always work");
        s
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string(false, false))?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ServerSerializer {
    pub url: String,
    pub login_info: Option<LoginInfo>
}

// aw shoot this cant get the client just from deserializing...
impl From<ServerSerializer> for Server {
    fn from(value: ServerSerializer) -> Self {
        Self::new(Url::parse(&value.url).expect("Could not parse server!"), value.login_info)
    }
}

impl Into<ServerSerializer> for Server {
    fn into(self) -> ServerSerializer {
        ServerSerializer { url: String::from(self.url.as_str()), login_info: self.login_info.clone() }
    }
}