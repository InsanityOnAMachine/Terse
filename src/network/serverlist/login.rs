
use serde::Deserialize;
use crate::network::Server;
use crate::network::LoginInfo;
use anyhow::Error;
use super::ServerList;

// What the server sends back when you ask to login
#[derive(Deserialize)]
pub enum LoginOption {
    PleaseVerify,
    Success,
    BadPassword,
}

impl ServerList {

// test the server, etc
    pub fn request_login(&self, server: &Server, login_info: &LoginInfo) -> Result<LoginOption, Error> {
        Ok(
            self.client.post(server.url_with_params("user/login", ""))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&login_info)?)
            .send()?
            .json::<LoginOption>()?
        )
    }

    // This, with some changes, could be merged with request_login
    pub fn verify_user(&self, server: &Server, login_info: &LoginInfo, code: &str) -> Result<bool, Error> {
        // Edge cases:
        // - The login info has like, a bad email address ("asfasodfhaoiuf/s.as.ai" or so)
        //     (but this fn is only called after request_login, so it'd be caught then)
        Ok(
            // I don't wanna hafta figure out how to stick the code in the body too.
            self.client.post(server.url_with_params("user/verify", format!("code={code}")))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&login_info)?)
            .send()?
            .json::<bool>()?
        )
    }
   }