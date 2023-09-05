use candid::CandidType;
use serde::Deserialize;

use crate::models::prescription_template::PrescriptionTemplateId;

#[derive(CandidType, Clone, Deserialize)]
pub struct Doctor {
    pub license_num: String,
    pub prescription_template: Option<String>,
    pub cert: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct DoctorRequest {
    license_num: String,
    prescription_template: Option<PrescriptionTemplateId>,
    cert: String,
}

#[derive(CandidType)]
pub struct DoctorResponse {
    license_num: String,
    prescription_template: Option<PrescriptionTemplateId>,
}

impl Doctor {
    pub fn new(
        e: &DoctorRequest
    ) -> Self {
        Self {
            license_num: e.license_num.clone(),
            prescription_template: None,
            cert: e.cert.clone(),
        }
    }

    pub fn update(
        &self
    ) -> Self {
        self.clone()
    }
}

impl From<Doctor> for DoctorResponse {
    fn from(
        e: Doctor
    ) -> Self {
        Self { 
            license_num: e.license_num,
            prescription_template: e.prescription_template,
        }
    }
}

impl From<DoctorRequest> for Doctor {
    fn from(
        e: DoctorRequest
    ) -> Self {
        Self {
            license_num: e.license_num,
            prescription_template: e.prescription_template,
            cert: e.cert,
        }
    }
}
