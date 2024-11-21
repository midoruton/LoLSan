use anyhow::{Error, Result,bail};
use serde_json::Value;
use jsonschema::ErrorIterator;
use std::fmt::{self, Debug};
use derive_more::{derive::Error, Display, Into};
#[derive(Error,Debug)]
struct ValidationError {
    error : String
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}






async fn fetch_and_validate(url: &str, schema: &Value) -> Result<Value> {
    let response = reqwest::get(url).await?;
    let validator = jsonschema::validator_for(schema)?;

    match response.error_for_status() {
        Ok(response) => {
            let body = response.json().await?;
            if validator.is_valid(&body) {
                Ok(body)
            } else {
                //エラーに謎のイテレータが返ってくるので、とりあえずstringに変換しておく
                let error_strings = validator
                .iter_errors(&body)
                .map(|e| e.to_string())
                .fold(String::new(), |acc, e| acc + &e + "\n");
                Err(anyhow::anyhow!("Validation error: {:?}", error_strings))
            }
        }
        Err(e) => Err(e.into()),
    }
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
            let result = fetch_and_validate(&url, &schema).await;
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
            let result = fetch_and_validate(&url, &schema).await;

            // The test expects an error because the schema requires a "fakeId" field which is not present in the JSON response.
            assert!(result.is_err_and(|e| e.to_string().contains("Validation error:")));
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
            let result = fetch_and_validate(&fake_url, &schema).await;

            // The test expects an error because the URL is invalid.
            assert!(!result.is_err_and(|e| e.downcast_ref::<reqwest::Error>().is_some()));
        }
        mock.assert_async().await;
    }
}