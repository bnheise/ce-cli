use batch_api::{
    models::{import_task::ImportStrategy, ImportTask},
    reqwest::{self, header::CONTENT_TYPE, Url},
};
use object_admin::{apis::ResponseContent, models::CreateStrategy};

use crate::liferay_client::liferay_client_error::LiferayClientError;

pub struct CustomObjectEndpoints<'a> {
    base_path: &'a Url,
    username: &'a str,
    password: &'a str,
}

impl<'a> CustomObjectEndpoints<'a> {
    pub fn new(base_path: &'a Url, username: &'a str, password: &'a str) -> Self {
        Self {
            base_path,
            username,
            password,
        }
    }

    pub fn post_object_data_batch(
        &self,
        body: &serde_json::Value,
        batch_path: &str,
        create_strategy: Option<object_admin::models::CreateStrategy>,
        import_strategy: Option<batch_api::models::import_task::ImportStrategy>,
    ) -> Result<ImportTask, LiferayClientError<serde_json::Value>> {
        let client = Self::get_object_client()?;

        let body = serde_json::to_string(&body)?;

        let resp = client
            .post(self.base_path.join(batch_path).unwrap())
            .header(CONTENT_TYPE, "application/json")
            .query(&[
                (
                    "importStrategy",
                    import_strategy
                        .unwrap_or(ImportStrategy::Continue)
                        .to_string(),
                ),
                (
                    "createStrategy",
                    create_strategy
                        .unwrap_or(CreateStrategy::Upsert)
                        .to_string(),
                ),
            ])
            .body(body)
            .basic_auth(self.username, Some(self.password))
            .send()?;

        if resp.status().is_success() {
            let status = resp.status();
            if let Ok(resp) = resp.json::<ImportTask>() {
                Ok(resp)
            } else {
                let resp_cont = ResponseContent {
                    status,
                    content: String::new(),
                    entity: None,
                };
                Err(LiferayClientError::Response { origin: resp_cont })
            }
        } else {
            let resp_cont = ResponseContent {
                status: resp.status(),
                content: resp.text().unwrap(),
                entity: None,
            };
            Err(LiferayClientError::Response { origin: resp_cont })
        }
    }

    fn get_object_client<T>() -> Result<reqwest::blocking::Client, LiferayClientError<'a, T>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );

        Ok(reqwest::blocking::ClientBuilder::new()
            .default_headers(headers)
            .build()?)
    }
}
