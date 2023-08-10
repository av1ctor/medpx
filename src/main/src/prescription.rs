use candid::{Principal, CandidType};
use serde::Deserialize;

pub type PrescriptionId = String;

#[derive(CandidType, Clone, Deserialize)]
pub struct Prescription {
    pub id: PrescriptionId,
    pub doctor: Principal,
    pub patient: Principal,
    pub contents: Vec<u8>,
    pub created_at: u64,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionRequest {
    pub doctor: Principal,
    pub patient: Principal,
    pub contents: Vec<u8>,
}

#[derive(CandidType, Clone)]
pub struct PrescriptionResponse {
    id: PrescriptionId,
    doctor: Principal,
    patient: Principal,
    contents: Vec<u8>,
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
        }
    }
}

impl From<Prescription> for PrescriptionResponse {
    fn from(
        e: Prescription
    ) -> Self {
        Self { 
            id: e.id,
            doctor: e.doctor, 
            patient: e.patient, 
            contents: e.contents, 
        }
    }
}