use crate::{client::Client, configuration::Configuration};
use std::cell::RefCell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("client ID is not provided or it is invalid")]
    InvalidClientId,
    #[error("client secret is not provided or it is invalid")]
    InvalidClientSecret,
    #[error("client error")]
    HttpClientError(#[from] crate::client::ClientError),
}

pub struct Api {
    configuration: RefCell<Configuration>,
    client: Option<Client>,
}

impl Api {
    pub fn new(configuration: &RefCell<Configuration>) -> Api {
        Api {
            configuration: configuration.clone(),
            client: None,
        }
    }

    pub fn configuration(&self) -> RefCell<Configuration> {
        self.configuration.clone()
    }

    pub async fn login(&mut self) -> Result<(), ApiError> {
        let project_id = self.configuration.borrow().project_id();
        let environment_id = self.configuration.borrow().environment_id();
        //let account = self.configuration.borrow().account();
        let client_id = self.configuration.borrow().client_id();
        let client_secret = self.configuration.borrow().client_secret();

        if client_id.is_none() || client_id.clone().unwrap().is_empty() {
            return Err(ApiError::InvalidClientId);
        }
        let client_id = client_id.unwrap();

        if client_secret.is_none() || client_secret.clone().unwrap().is_empty() {
            return Err(ApiError::InvalidClientSecret);
        }
        let client_secret = client_secret.unwrap();

        let mut client = Client::new(project_id, environment_id, client_id, client_secret)?;
        client.login().await?;

        Ok(())
    }

    pub fn logoff(&mut self) -> Result<(), ApiError> {
        self.client = None;

        todo!("Implement logoff");
    }
}
