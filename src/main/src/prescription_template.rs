use candid::{Principal, CandidType};
use serde::Deserialize;

pub type PrescriptionTemplateId = String;

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionTemplate {
    pub id: PrescriptionTemplateId,
    pub title: String,
    pub body: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}
