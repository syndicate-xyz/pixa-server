use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder,
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::sync::LazyLock;
use std::time::Duration;

// Create a global HTTP client
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .use_native_tls()
        .build()
        .expect("Failed to build HTTP client")
});

pub struct HttpClient {
    client: Client,
}

#[derive(Debug)]
pub enum HttpError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError { status: u16, body: String },
}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        HttpError::RequestError(err)
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(err: serde_json::Error) -> Self {
        HttpError::JsonError(err)
    }
}

impl HttpClient {
    pub fn new() -> Result<Self, HttpError> {
        // Use a reference to the global client
        let client = HTTP_CLIENT.clone();
        Ok(Self { client })
    }

    pub fn with_timeout(timeout_secs: u64) -> Result<Self, HttpError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .pool_max_idle_per_host(32)
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .build()?;
        Ok(Self { client })
    }

    // Helper method to handle response
    pub async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let data = response.json::<T>().await?;
            Ok(data)
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(HttpError::ApiError {
                status: status.as_u16(),
                body,
            })
        }
    }

    // Modified GET request
    pub async fn get<T>(&self, url: &str, headers: Option<HeaderMap>) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let mut request = self.client.get(url);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;

        tracing::debug!("URL: {}", url);

        self.handle_response(response).await
    }

    // Modified POST request
    pub async fn post<T, B>(
        &self,
        url: &str,
        body: &B,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut request = self.client.post(url).json(body);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // Modified PUT request
    pub async fn put<T, B>(
        &self,
        url: &str,
        body: &B,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut request = self.client.put(url).json(body);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // Modified DELETE request
    pub async fn delete<T>(&self, url: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let response = self.client.delete(url).send().await?;
        self.handle_response(response).await
    }

    // Helper to build headers
    pub fn build_headers(headers: Vec<(&str, &str)>) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                header_map.insert(header_name, header_value);
            }
        }
        header_map
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Serialize, Deserialize)]
    struct Todo {
        id: i32,
        title: String,
        completed: bool,
    }

    #[tokio::test]
    async fn test_get_request() {
        let client = HttpClient::new().unwrap();
        let todo: Todo = client
            .get("https://jsonplaceholder.typicode.com/todos/1", None)
            .await
            .unwrap();
        assert_eq!(todo.id, 1);
        println!("todo: {:?}", todo);
    }

    #[tokio::test]
    async fn test_post_request() {
        // let headers = HttpClient::build_headers(vec![
        //     ("Authorization", "Bearer your-token"),
        //     ("Content-Type", "application/json"),
        // ]);

        let client = HttpClient::new().unwrap();
        let new_todo = Todo {
            id: 1,
            title: String::from("Test todo"),
            completed: false,
        };

        let response: Todo = client
            .post(
                "https://jsonplaceholder.typicode.com/todos",
                &new_todo,
                None,
            )
            .await
            .unwrap();

        println!("response: {:?}", response);
        assert_eq!(response.title, "Test todo");
    }

    #[tokio::test]
    async fn test_error_handling() {
        let client = HttpClient::new().unwrap();

        // Test 404 Not Found error
        match client
            .get::<Todo>("https://jsonplaceholder.typicode.com/todos/999999", None)
            .await
        {
            Ok(_) => panic!("Expected an error for non-existent resource"),
            Err(HttpError::ApiError { status, body }) => {
                assert_eq!(status, 404);
                assert!(!body.is_empty(), "Error body should not be empty");
                println!(
                    "Successfully handled 404 error - Status: {}, Body: {}",
                    status, body
                );
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }

        // Test malformed URL error
        match client.get::<Todo>("invalid-url", None).await {
            Ok(_) => panic!("Expected an error for invalid URL"),
            Err(HttpError::RequestError(e)) => {
                println!("Successfully handled invalid URL error: {}", e);
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }

        // Test JSON deserialization error (trying to parse wrong response type)
        match client
            .get::<Todo>("https://jsonplaceholder.typicode.com/users/1", None)
            .await
        {
            Ok(_) => panic!("Expected a JSON error for incompatible types"),
            Err(HttpError::JsonError(_)) | Err(HttpError::ApiError { .. }) => {
                println!("Successfully handled JSON parsing error");
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
