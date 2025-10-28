use serde::Deserialize;
use serde::de::DeserializeOwned; 
use serde_derive::Deserialize; 
use crate::req_client::req_handler;

pub struct App {
    session: String,
    client_metadata: ClientMetadata,
}

#[derive(Debug, Deserialize)]
pub struct ClientMetadata {
    ID: u64,
    IdDot: u64,
    YearStudy: String,
    TermID: String,
    BeginDate: String,
    EndDate: String,
    #[serde(default)]
    study_programs: Vec<StudyProgram>,
}

#[derive(Debug, Deserialize)]
pub struct StudyProgram {
    StudyProgramID: String,
    StudyProgramName: String,
}

impl App {
    pub fn new(session: &String) -> Self {
        Self {
            session: session.to_string(),
            client_metadata: ClientMetadata {
                ID: 0,
                IdDot: 0,
                YearStudy: "".to_string(),
                TermID: "".to_string(),
                BeginDate: "".to_string(),
                EndDate: "".to_string(),
                study_programs: vec![],
            },
        }
    }

    pub async fn set_client_metadata(&mut self) {
        let study_programs_json = req_handler::getAllStudyProgramRegist(&self.session).await;
        let study_programs: Vec<StudyProgram> =
            serde_json::from_str(&study_programs_json).unwrap_or_default();
        println!("Study Programs: {:?}", study_programs);

        let quota_json = req_handler::getRegistSemesterCreditQuota(&self.session).await;
        let mut quota: ClientMetadata = serde_json::from_str(&quota_json).unwrap();

        quota.study_programs = study_programs;

        println!("Client Metadata: {:?}", quota);
        self.client_metadata = quota;
    }
}

