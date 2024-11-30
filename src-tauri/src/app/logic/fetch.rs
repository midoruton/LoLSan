use derive_more::Into;
use serde_json::Value;
use std::fmt::Debug;

pub async fn fetch(url: &str, danger_accept_invalid_cert: bool) -> Result<Value, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(danger_accept_invalid_cert)
        .build()?;
    let response = client.get(url).send().await?;
    match response.error_for_status() {
        Ok(response) => Ok(response.json().await?),
        Err(e) => Err(e),
    }
}

#[derive(Debug, thiserror::Error)]
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
impl From<jsonschema::ErrorIterator<'_>> for ValidationError {
    fn from(iter_errors: jsonschema::ErrorIterator) -> Self {
        let s = iter_errors
            .map(|e| e.to_string())
            .fold(String::new(), |acc, e| acc + &e + "\n");
        ValidationError::JSONSchema(s)
    }
}

pub async fn validate(schema: &Value, body: &Value) -> Result<(), ValidationError> {
    let validator = jsonschema::draft7::options()
        .build(schema)?;
    if validator.is_valid(body) {
        Ok(())
    } else {
        Err(validator.iter_errors(body).into())
    }
}



#[cfg(test)]
mod tests {
    use crate::types::error::LoLSanError;
    // This function fetches a JSON response from the given URL and validates it against the given JSON schema.
    async fn fetch_and_validate(
        url: &str,
        danger_accept_invalid_cert: bool,
        schema: &Value,
    ) -> Result<Value, LoLSanError> {
        let responce = fetch(url, danger_accept_invalid_cert).await?;
        validate(schema, &responce).await?;
        Ok(responce)
    }
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
            let result = fetch_and_validate(&url, true, &schema).await;
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
            let result = fetch_and_validate(&url, true, &schema).await;

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
            let result = fetch_and_validate(&fake_url, true, &schema).await;

            // The test expects an error because the URL is invalid.
            assert!(matches!(result, Err(LoLSanError::Reqwest(_))));
        }
        mock.assert_async().await;
    }

    const ALL_GAME_DATA_SCHEMA_STR: &str = std::include_str!("../../../../src/schema/AllGameData.json");
    const VALID_GAME_DATA_JSON_STR: &str = std::include_str!("../../../../src/schema/ValidAllGameData1.json");
    const INVALID_GAME_DATA_JSON_STR: &str = std::include_str!("../../../../src/schema/InvalidAllGameData1.json");
    #[tokio::test]
    async fn test_validate(){
        let schema = serde_json::from_str::<serde_json::Value>(ALL_GAME_DATA_SCHEMA_STR).unwrap();
        println!("Schema loaded: {}", schema);

        let valid_body = serde_json::from_str::<serde_json::Value>(VALID_GAME_DATA_JSON_STR).unwrap();
        let result = validate(&schema, &valid_body).await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());

        let invalid_body = serde_json::from_str::<serde_json::Value>(INVALID_GAME_DATA_JSON_STR).unwrap();
        let result = validate(&schema, &invalid_body).await;
        println!("Result: {:?}", result);
        assert!(result.is_err());
        //let body = serde_json::json!({"userId": 1});
        //let result = validate(&schema, &body);
        //assert!(matches!(result, Err(ValidationError::JSONSchema(_))));
    }
}
