use reqwest::Client;
use reqwest::RequestBuilder;

pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest {
            method: "".to_string(),
            url: "".to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn add_header(&mut self, key: String, value: String) -> &mut Self {
        if key.is_empty() || value.is_empty() {
            panic!("Header key and value cannot be empty");
        }
        self.headers.push((key, value));
        return self;
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        if url.is_empty() {
            panic!("Url cannot be empty");
        }
        self.url = url;
        return self;
    }
    pub fn set_body(&mut self, body: String) -> &mut Self {
        if body.is_empty() {
            panic!("Body cannot be empty");
        }
        self.body = Some(body);
        return self;
    }
    pub fn set_method(&mut self, method: String) -> &mut Self {
        if !["GET", "POST", "PUT", "DELETE", "PATCH"].contains(&method.as_str()) {
            panic!(
                "Invalid HTTP method: {}, method should be: GET, POST, PUT, DELETE.",
                method
            );
        }
        self.method = method;
        return self;
    }

    pub fn request_builder(&self) -> reqwest::RequestBuilder {
        if self.method.is_empty() {
            panic!("HTTP method cannot be empty");
        }
        if self.url.is_empty() {
            panic!("URL cannot be empty");
        }
        if self.headers.is_empty() {
            panic!("Headers cannot be empty");
        }

        let client = Client::new();
        let mut request_builder = client.request(
            reqwest::Method::from_bytes(self.method.as_bytes()).unwrap(),
            &self.url,
        );
        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }
        if let Some(body) = &self.body {
            request_builder = request_builder.body(body.clone());
        }
        return request_builder;
    }
}

pub async fn send(request: RequestBuilder) -> Result<reqwest::Response, String> {
    let response = request.send().await.map_err(|e| e.to_string())?;
    Ok(response)
}
