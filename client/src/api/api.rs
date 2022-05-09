use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;

use crate::errors::{ApiError, ClientError};
use common::models::RegisterOrRefreshServerReq;

#[derive(Clone)]
pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self { client }
    }

    pub async fn discovery_hello(&self) -> Result<(), ClientError> {
        self.client
            .get("http://localhost:8000/")
            .send()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?
            .json::<HashMap<String, bool>>()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?;

        Ok(())
    }

    pub async fn register_server(
        &self,
        req_body: RegisterOrRefreshServerReq,
    ) -> Result<(), ClientError> {
        self.client
            .post("http://localhost:8000/api/register")
            .json(&req_body)
            .send()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?
            .json::<HashMap<String, bool>>()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?;

        Ok(())
    }
}
