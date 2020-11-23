use super::models::{IAPRequest, StatusResponse};
use anyhow::{anyhow, Result};
use futures::stream::StreamExt;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
// use serde::{Deserialize, Serialize};

const SANDBOX_URL: &str = "https://sandbox.itunes.apple.com/verifyReceipt";
const PRODUCTION_URL: &str = "https://buy.itunes.apple.com/verifyReceipt";

pub struct ReqClient {
    production_url: String,
    sandbox_url: String,
    password: String,
    exclude_old_transactions: bool,
}

impl ReqClient {
    pub fn new(password: &str, exclude_old_transactions: bool) -> ReqClient {
        ReqClient {
            production_url: PRODUCTION_URL.into(),
            sandbox_url: SANDBOX_URL.into(),
            password: password.to_string(),
            exclude_old_transactions,
        }
    }

    pub async fn verify(&self, receipt: String) -> Result<Vec<u8>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let req_body = IAPRequest {
            receipt_data: receipt,
            password: self.password.clone(),
            exclude_old_transactions: self.exclude_old_transactions,
        };

        let req_body = serde_json::to_string(&req_body)?;
        let req = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json; charset=utf-8")
            .uri(&self.production_url)
            .body(Body::from(req_body.clone()))?;

        let resp = client.request(req).await?;

        if resp.status() >= hyper::StatusCode::INTERNAL_SERVER_ERROR {
            return Err(anyhow!(
                "Received http status code {} from the App Store Sandbox: {:?}",
                resp.status().to_string(),
                hyper::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            ));
        }

        let mut data_slice: Vec<u8> = Vec::new();
        let mut body = resp.into_body();
        while let Some(chunk) = body.next().await {
            data_slice.extend(&chunk?);
        }

        let status_code: StatusResponse = serde_json::from_slice(&data_slice)?;

        if status_code.status == 21007 {
            let req = Request::builder()
                .method("POST")
                .header("Content-Type", "application/json; charset=utf-8")
                .uri(&self.sandbox_url)
                .body(Body::from(req_body))?;

            let resp = client.request(req).await?;
            let mut body = resp.into_body();
            let mut data_slice: Vec<u8> = Vec::new();
            while let Some(chunk) = body.next().await {
                data_slice.extend(&chunk?);
            }

            return Ok(data_slice);
        }

        return Ok(data_slice);
    }
}
