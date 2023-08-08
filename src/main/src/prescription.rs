use candid::{Principal, CandidType};
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct Prescription {
    pub id: String,
    pub doctor: Principal,
    pub patient: Principal,
    pub contents: Vec<u8>,
    pub created_at: u64,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionRequest {
    pub doctor: Principal,
    pub patient: Principal,
    pub contents: Vec<u8>,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Clone)]
pub struct PrescriptionResponse {
    doctor: Principal,
    patient: Principal,
    contents: Vec<u8>,
    expires_at: Option<u64>,
}

impl Prescription {
    pub fn new(
        id: &String,
        e: &PrescriptionRequest
    ) -> Self {
        Self { 
            id: id.clone(),
            doctor: e.doctor, 
            patient: e.patient, 
            contents: e.contents.clone(), 
            created_at: ic_cdk::api::time(), 
            deleted_at: None,
            deleted_by: None,
            expires_at: e.expires_at, 
        }
    }
}

impl From<Prescription> for PrescriptionResponse {
    fn from(
        e: Prescription
    ) -> Self {
        Self { 
            doctor: e.doctor, 
            patient: e.patient, 
            contents: e.contents, 
            expires_at: e.expires_at, 
        }
    }
}