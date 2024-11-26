use serde_json::Value;
use jsonschema::ErrorIterator;
use std::fmt::{self, Debug};
use derive_more::{derive::Error, Display, Into};
use super::super::super::types::error::LoLSanError;

pub async fn fetch (url: &str,danger_accept_invalid_cert: bool) -> Result<Value,reqwest::Error> {
    let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(danger_accept_invalid_cert).build()?;
    let response = client.get(url).send().await?;
    match response.error_for_status() {
        Ok(response) => {
            Ok(response.json().await?)
        }
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug,thiserror::Error)]
pub enum ValidationError {
    #[error("JSONSchemaError: {0}")]
    JSONSchema(String),
}

impl From<jsonschema::ValidationError<'static>> for ValidationError {
    fn from(error: jsonschema::ValidationError) -> Self {
        ValidationError::JSONSchema(error.to_string())
    }
}

//エラーに謎のイテレータが返ってくるので、とりあえずstringに変換しておく
impl From<jsonschema::ErrorIterator<'_>> for ValidationError{
    fn from(iter_errors: jsonschema::ErrorIterator) -> Self {
        let s = iter_errors.map(|e| e.to_string()).fold(String::new(), |acc, e| acc + &e + "\n");
        ValidationError::JSONSchema(s)
    }
}

pub async fn validate (schema: &Value, body: &Value) -> Result<(),ValidationError> {
    let validator = jsonschema::validator_for(schema)?;
    if validator.is_valid(body) {
        Ok(())
    } else {
        Err(validator.iter_errors(body).into())
    }
}

// This function fetches a JSON response from the given URL and validates it against the given JSON schema.
async fn fetch_and_validate(url: &str,danger_accept_invalid_cert: bool, schema: &Value) -> Result<Value,LoLSanError> {
    let responce = fetch(url,danger_accept_invalid_cert).await?;
    validate(schema,&responce).await?;
    Ok(responce)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_fetch_and_validate() {
        let mut server = mockito::Server::new_async().await;
        let path = "/lolsan";
        let json_body = r#"{"userId": 1, "id": 1}"#;
        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .expect_at_least(1)
            .create_async()
            .await;
        let url = server.url() + path;

        {
            let schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "userId": {"type": "integer"},
                    "id": {"type": "integer"}
                },
                "required": ["userId", "id"]
            });
            let result = fetch_and_validate(&url,true, &schema).await;
            // The test expects a successful result because the JSON response matches the schema.
            assert!(result.is_ok_and(|v| v == serde_json::json!({"userId": 1, "id": 1})));
        }
        {
            let schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "userId": {"type": "integer"},
                    "fakeId": {"type": "string"},
                    "id": {"type": "integer"}
                },
                "required": ["userId", "id", "fakeId"]
            });
            let result = fetch_and_validate(&url, true,&schema).await;

            // The test expects an error because the schema requires a "fakeId" field which is not present in the JSON response.
            assert!(matches!(result, Err(LoLSanError::Validation(_))));
        }
        
        let fake_url = server.url() + "/fake";
        {
            let schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "userId": {"type": "integer"},
                    "fakeId": {"type": "string"},
                    "id": {"type": "integer"}
                },
                "required": ["userId", "id", "fakeId"]
            });
            let result = fetch_and_validate(&fake_url, true,&schema).await;

            // The test expects an error because the URL is invalid.
            assert!(matches!(result, Err(LoLSanError::Reqwest(_))));
        }
        mock.assert_async().await;
    }
}
