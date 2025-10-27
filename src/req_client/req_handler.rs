use super::req_builder;
use serde_json::json;

pub async fn getAllStudyProgramRegist(session: &String) -> String {
    let url = "https://dangkyapi.hcmute.edu.vn/api/Authen/GetAllStudyProgramRegist";

    let request = req_builder::HttpRequest::new()
        .set_method("GET".to_string())
        .add_header(
            "Authorization".to_string(),
            "Bearer ".to_string() + &session,
        )
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

pub async fn getRegistSemesterCreditQuota(session: &String) -> String {
    let url = "https://dangkyapi.hcmute.edu.vn/api/Regist/GetRegistSemesterCreditQuota?StudyProgramID=24110";

    let request = req_builder::HttpRequest::new()
        .set_method("GET".to_string())
        .add_header(
            "Authorization".to_string(),
            "Bearer ".to_string() + &session,
        )
        .set_url(url.to_string())
        .request_builder();

    let response = req_builder::send(request).await.unwrap();
    if response.status().is_success() {
        let text = response.text().await.unwrap();
        return text;
    } else {
        panic!(
            "Failed to fetch semester credit quota: {}",
            response.status()
        );
    }
}

pub async fn getAllClassRegisted(session: &String) -> String {
    let url = "https://dangkyapi.hcmute.edu.vn/api/Regist/GetAllClassRegisted";
    let body = json!({
        "ReqParam1": "0",
        "ReqParam2": "62"
    });

    let request = req_builder::HttpRequest::new()
        .set_method("POST".to_string())
        .add_header(
            "Authorization".to_string(),
            "Bearer ".to_string() + &session,
        )
        .add_header("Content-Type".to_string(), "application/json".to_string()) // important!
        .set_body("{}".to_string())
        .set_url(url.to_string())
        .request_builder();

    let response = req_builder::send(request).await.unwrap();
    if response.status().is_success() {
        let text = response.text().await.unwrap();
        return text;
    } else {
        println!("Failed to fetch all class registeds: {}", response.status());
        return "".to_string();
    }
}
pub async fn getAllClassAllowRegist(session: &String) -> String {
    let url = "https://dangkyapi.hcmute.edu.vn/api/Regist/GetAllClassAllowRegist";
    let body = json!({
      "ReqParam1": "24110",
      "ReqParam2": "KH",
      "ReqParam3": "2025-2026",
      "ReqParam4": "HK01",
      "ReqParam5": ""
    });

    let request = req_builder::HttpRequest::new()
        .set_method("POST".to_string())
        .add_header(
            "Authorization".to_string(),
            "Bearer ".to_string() + &session,
        )
        .add_header("Content-Type".to_string(), "application/json".to_string()) // important!
        .set_body(body.to_string())
        .set_url(url.to_string())
        .request_builder();
    let response = req_builder::send(request).await.unwrap();
    if response.status().is_success() {
        let text = response.text().await.unwrap();
        return text;
    } else {
        println!(
            "Failed to fetch all class allow registeds: {}",
            response.status()
        );
        return "".to_string();
    }
}
