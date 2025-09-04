use std::env;
use super::req_builder;

pub async fn getAllStudyProgramRegist() -> String {
    
    let session = env::var("SESSION").expect("SESSION environment variable not set");
    let url = "https://dangkyapi.hcmute.edu.vn/api/Authen/GetAllStudyProgramRegist";

    let request = req_builder::HttpRequest::new()
        .set_method("GET".to_string())
        .add_header("Authorization".to_string(), "Bearer ".to_string() + &session)
        .set_url(url.to_string())
        .request_builder();
    
    let response = req_builder::send(request).await.unwrap(); 
    if response.status().is_success() {
        let text = response.text().await.unwrap();
        return text;
    } else {
        panic!("Failed to fetch study programs: {}", response.status());
    }
}
