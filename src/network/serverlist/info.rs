use crate::network::Server;
use reqwest::StatusCode;
use versions::*;
use super::{ServerList, NetworkError};

impl ServerList {
	pub fn exists_and_is_a_terse_server(&self, server: &Server) -> Result<bool, NetworkError> {
        let status = self.client.get(server.url_with_params("exists-and-is-a-terse-server", ""))
        .send()?.status();

        match status {
            StatusCode::OK => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(NetworkError::UnexpectedStatusCode{code: status.canonical_reason().unwrap_or(status.as_str()).to_string()}.into())
        }
    }

    pub fn get_version(&self, server: &Server) -> Result<Version, NetworkError> {
        Ok(
            self.client.get(server.url_with_params("version", ""))
                .send()?
                .json::<Version>()?
        )
    }
}
