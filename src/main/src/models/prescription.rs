use candid::{Principal, CandidType};
use serde::Deserialize;

use super::user::UserId;

pub type PrescriptionId = String;

#[derive(CandidType, Clone, Deserialize)]
pub struct Prescription {
    pub id: PrescriptionId,
    pub doctor: UserId,
    pub patient: UserId,
    pub hash: Vec<u8>,
    pub contents: Option<Vec<u8>>,
    pub created_at: u64,
    pub created_by: Principal,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionRequest {
    pub patient: UserId,
    pub hash: Vec<u8>,
    pub contents: Option<Vec<u8>>,
}

#[derive(CandidType, Clone)]
pub struct PrescriptionResponse {
    id: PrescriptionId,
    doctor: UserId,
    patient: UserId,
    hash: Vec<u8>,
    contents: Vec<u8>,
    created_at: u64,
}

impl Prescription {
    pub fn new(
        id: &String,
        e: &PrescriptionRequest,
        caller: &Principal
    ) -> Self {
        Self { 
            id: id.clone(),
            doctor: caller.clone(), 
            patient: e.patient, 
            hash: e.hash.clone(),
            contents: e.contents.clone(), 
            created_at: ic_cdk::api::time(), 
            created_by: caller.clone(),
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
            hash: e.hash,
            contents: e.contents.unwrap_or_default(), 
            created_at: e.created_at,
        }
    }
}