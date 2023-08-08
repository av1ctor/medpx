use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct Doctor {
    pub id: String,
    pub name: String,
    pub num_prescriptions: u32,
    pub prescription_template: Option<String>,
    pub credits: u128,
    pub created_at: u64,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct DoctorRequest {
    id: String,
    name: String,
    prescription_template: Option<String>,
}

#[derive(CandidType)]
pub struct DoctorResponse {
    id: String,
    name: String,
    prescription_template: Option<String>,
}

impl Doctor {
    pub fn new(
        e: &DoctorRequest
    ) -> Self {
        Self {
            id: e.id.clone(),
            name: e.name.clone(),
            prescription_template: None,
            num_prescriptions: 0,
            credits: 0,
            created_at: ic_cdk::api::time(),
            updated_at: None,
            updated_by: None,
            deleted_at: None,
            deleted_by: None,
        }
    }
}

impl From<Doctor> for DoctorResponse {
    fn from(
        e: Doctor
    ) -> Self {
        Self { 
            id: e.id,
            name: e.name, 
            prescription_template: e.prescription_template,
        }
    }
}
