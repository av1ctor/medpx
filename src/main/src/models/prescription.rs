use candid::{Principal, CandidType};
use serde::Deserialize;

use super::user::UserId;

pub type PrescriptionId = String;

#[derive(CandidType, Clone, Deserialize, PartialEq)]
pub enum PrescriptionState {
    Created,
    Signed,
    Deleted,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Prescription {
    pub id: PrescriptionId,
    pub state: PrescriptionState,
    pub doctor: UserId,
    pub patient: UserId,
    pub plain_text_hash: Vec<u8>,
    pub cipher_text_hash: Option<Vec<u8>>,
    pub cipher_text: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub cert: Option<String>,
    pub created_at: u64,
    pub created_by: Principal,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionPreRequest {
    pub patient: UserId,
    pub plain_text_hash: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionPostRequest {
    pub cipher_text_hash: Vec<u8>,
    pub cipher_text: Vec<u8>,
    pub signature: Vec<u8>,
    pub cert: String,
}

#[derive(CandidType, Clone)]
pub struct PrescriptionResponse {
    id: PrescriptionId,
    doctor: UserId,
    patient: UserId,
    plain_text_hash: Vec<u8>,
    cipher_text: Vec<u8>,
    created_at: u64,
}

impl Prescription {
    pub fn new(
        id: &String,
        e: &PrescriptionPreRequest,
        caller: &Principal
    ) -> Self {
        Self { 
            id: id.clone(),
            state: PrescriptionState::Created,
            doctor: caller.clone(), 
            patient: e.patient, 
            plain_text_hash: e.plain_text_hash.clone(),
            signature: None,
            cert: None,
            cipher_text_hash: None,
            cipher_text: None, 
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
            plain_text_hash: e.plain_text_hash,
            cipher_text: e.cipher_text.unwrap_or_default(), 
            created_at: e.created_at,
        }
    }
}