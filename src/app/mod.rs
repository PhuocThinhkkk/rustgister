use crate::req_client::req_handler;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct App {
    session: String,
    client_metadata: ClientMetadata,
    currently_registed_classes: Vec<ClassRow>,
    classes_allow_regist: Vec<req_handler::ClassAllowRegistResponse>,
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

#[derive(Debug, Deserialize)]
pub struct ClassRow {
    pub ScheduleStudyUnitID: String,
    pub ScheduleStudyUnitAlias: String,
    pub StudyUnitID: String,
    pub StudyUnitTypeID: u8,
    pub StudyUnitTypeName: String,
    pub CurriculumID: String,
    pub CurriculumName: String,
    pub Credits: f32,
    pub ProfessorName: String,
    pub Schedules: String,
    pub Status: u8,
    pub IsTranfer: bool,
    pub IsDelete: bool,
    pub BeginDate: String,
    pub EndDate: String,
    pub TrungLich: bool,
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
            currently_registed_classes: vec![],
            classes_allow_regist: vec![],
        }
    }

    pub async fn set_client_metadata(&mut self) -> &Self {
        let study_programs_json = req_handler::getAllStudyProgramRegist(&self.session).await;
        let study_programs: Vec<StudyProgram> =
            serde_json::from_str(&study_programs_json).unwrap_or_default();

        let quota_json = req_handler::getRegistSemesterCreditQuota(&self.session).await;
        let mut quota: ClientMetadata = serde_json::from_str(&quota_json).unwrap();

        quota.study_programs = study_programs;

        self.client_metadata = quota;
        return self;
    }
    pub async fn set_all_class_registed(&mut self) -> &Self {
        let class_registed_json = req_handler::getAllClassRegisted(&self.session)
            .await
            .unwrap();
        let class_registed = class_registed_json.Rows;
        return self;
    }

    pub async fn set_all_class_allow_regist(&mut self) -> &Self {
        let class_allow_regist_json = req_handler::getAllClassAllowRegist(&self.session)
            .await
            .unwrap();
        self.classes_allow_regist = class_allow_regist_json;
        return self;
    }
}
