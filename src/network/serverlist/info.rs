use serde::Deserialize;
use std::fmt::Display;
use std::fmt::Formatter;
use bytesize::ByteSize;

use crate::network::Server;
use reqwest::StatusCode;
use anyhow::Error;
use super::ServerList;

#[derive(Deserialize)]
pub struct ServerStats {
    pub server_name: String,
    version: String,
    users: u32,
    posts: u32,
    age: f64,
    max_post_size: u64,
    max_title_size: u64,
}

impl Display for ServerStats {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: that bookmarked clean-list library
        // TODO: coloration
        writeln!(f, "{} running Terse version {}", self.server_name, self.version)?;
        writeln!(f, "Users on server: {}", self.users)?;
        writeln!(f, "Answers on server: {}", self.posts)?;
        writeln!(f, "Server instance age: {}", self.age)?;
        writeln!(f, "Maximum post size: {}", ByteSize::b(self.max_post_size))?;
        writeln!(f, "Maximum title size: {} characters", self.max_title_size)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerValidityError {
    // No contact at all; the site doesn't exist
    #[error("I couldn't connect to the server")]
    CannotConnect,
    // There's no /exists-and-is-a-terse-server path
    #[error("The server isn't a Terse server")]
    ServerIsNotATerseServer,
    // Any other status code
    #[error("The server returned an unexpected status code: {0}")]
    Other(StatusCode),
}

impl From<reqwest::Error> for ServerValidityError {
    fn from(_value: reqwest::Error) -> Self {
        ServerValidityError::CannotConnect
    }
}

impl ServerList {
	pub fn exists_and_is_a_terse_server(&self, server: &Server) -> Result<(), ServerValidityError> {
        let status = self.client.get(server.url_with_params("exists-and-is-a-terse-server", ""))
        .send()?.status();

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(ServerValidityError::ServerIsNotATerseServer),
            _ => Err(ServerValidityError::Other(status))
        }
    }

    pub fn get_stats(&self, server: &Server) -> Result<ServerStats, Error> {
        Ok(
            self.client.get(server.url_with_params("stats", ""))
            .send() ?
            .json::<ServerStats>()?
        )
    }
}