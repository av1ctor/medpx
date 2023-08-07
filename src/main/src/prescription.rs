use candid::{Principal, CandidType};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Prescription {
    id: String,
    doctor: Principal,
    patient: Principal,
    contents: Vec<u8>,
    created_at: u64,
    expires_at: Option<u64>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionRequest {
    pub doctor: Principal,
    pub patient: Principal,
    pub contents: Vec<u8>,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Clone, Serialize)]
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