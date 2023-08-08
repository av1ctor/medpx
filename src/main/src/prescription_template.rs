use candid::{Principal, CandidType};
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionTemplate {
    pub title: String,
    pub body: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}
