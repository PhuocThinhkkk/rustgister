use super::req_builder;
use crate::app::ClassRow;
use serde::Deserialize;
use serde_derive::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct ClassRegistedResponse {
    pub Rows: Vec<ClassRow>,
    Reval: String,
}

pub async fn getAllClassRegisted(
    session: &String,
) -> Result<ClassRegistedResponse, Box<dyn std::error::Error>> {
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
        .add_header("Content-Type".to_string(), "application/json".to_string())
        .set_body(body.to_string())
        .set_url(url.to_string())
        .request_builder();

    let response = req_builder::send(request).await.unwrap();
    if response.status().is_success() {
        let text = response.text().await.unwrap();
        let class_registed_response: ClassRegistedResponse = serde_json::from_str(&text).unwrap();
        Ok(class_registed_response)
    } else {
        println!("Failed to fetch all class registed: {}", response.status());
        Err("Failed to fetch all class registed".into())
    }
}

#[derive(Debug, Deserialize)]
pub struct ClassAllowRegistResponse {
    pub CurriculumTypeGroupName: String,
    pub classStudyUnits: Vec<classStudyUnit>,
}
#[derive(Debug, Deserialize)]
struct classStudyUnit {
    SelectionName: Option<String>,
    Selections: Vec<Selection>,
}
#[derive(Debug, Deserialize)]
struct Selection {
    StudyUnitID: String,
    CurriculumID: String,
    CurriculumName: String,
    CurriculumType: String,
    NumberOfScheduleStudyUnit: u8,
    Credits: f32,
    CurriculumTypeGroupName: String,
    IsInsert: bool,
    SelectionID: String,
    SelectionName: Option<String>,
}
pub async fn getAllClassAllowRegist(session: &String) -> Result<Vec<ClassAllowRegistResponse> , Box<dyn std::error::Error>> {
    let url = "https://dangkyapi.hcmute.edu.vn/api/Regist/GetAllClassAllowRegist";
    let body = json!({
      "ReqParam1": "24110",
      "ReqParam2": "NKH",
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
        let class_allow_regist_response: Vec<ClassAllowRegistResponse> = serde_json::from_str(&text).unwrap();
        Ok(class_allow_regist_response)
    } else {
        println!("Failed to fetch all class allow regist: {}", response.status());
        Err("Failed to fetch all class allow regist".into())
    }
}
