use candid::{CandidType, Principal};
use serde::Deserialize;

pub type StaffId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub enum StaffRole {
    Admin,
    Contributor,
    Member,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Staff {
    pub id: StaffId,
    pub name: String,
    pub role: StaffRole,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct StaffRequest {
    name: String,
    role: StaffRole,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct StaffResponse {
    id: StaffId,
    name: String,
    role: StaffRole,
}

impl Staff {
    pub fn new(
        e: &StaffRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            name: e.name.clone(),
            role: e.role.clone(),
            created_at: ic_cdk::api::time(),
            created_by: caller.clone(),
            updated_at: None,
            updated_by: None,
            deleted_at: None,
            deleted_by: None,
        }
    }

    pub fn update(
        &self,
        caller: &Principal
    ) -> Self {
        Self {
            updated_at: Some(ic_cdk::api::time()),
            updated_by: Some(caller.clone()),
            ..self.clone()
        }
    }
}

impl From<Staff> for StaffResponse {
    fn from(
        e: Staff
    ) -> Self {
        Self { 
            id: e.id,
            name: e.name, 
            role: e.role,
        }
    }
}

