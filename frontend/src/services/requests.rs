// use web_sys::console;

use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;
use crate::types::ErrorInfo;

use reqwest::{Client};

const API_ROOT: &str = dotenv!("API_ROOT");



/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);

    let client = Client::builder()
        .build().unwrap();

    let mut builder = client
        .request(method, url)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Credentials", "true")
        .fetch_credentials_include(); 

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response { 

        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            
            // log::info!("cookies: {:?}", cookies.into());
            // console::log_1(&format!("reqwest test data: {:?}", data).into());
            if let Ok(data) = data {
                // log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}