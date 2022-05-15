use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;

use crate::errors::{ApiError, ClientError};
use common::models::{
    FindServerForFileReq, FindServerForFileRes, OfferReq, OfferRes, RegisterOrRefreshServerReq,
};

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
            .post("http://localhost:8000/api/server/register")
            .json(&req_body)
            .send()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?
            .json::<HashMap<String, bool>>()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?;

        Ok(())
    }

    pub async fn find_servers(
        &self,
        req_body: FindServerForFileReq,
    ) -> Result<FindServerForFileRes, ClientError> {
        let url = String::from("http://localhost:8000/api/server/") + &req_body.file_id;
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?
            .json::<FindServerForFileRes>()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?;

        Ok(resp)
    }

    pub async fn send_offer(
        &self,
        url: String,
        req_body: OfferReq,
    ) -> Result<OfferRes, ClientError> {
        let url = String::from(url + "/on-offer");
        let res = self
            .client
            .post(url)
            .json(&req_body)
            .send()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?
            .json::<OfferRes>()
            .await
            .map_err(|err| ClientError::ApiError(ApiError::ReqwestError(err)))?;

        Ok(res)
    }
}
