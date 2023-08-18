use candid::{CandidType, Principal};
use serde::Deserialize;

pub type PatientId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub struct Patient {
    pub id: PatientId,
    pub name: String,
    pub email: String,
    pub birth_date: u64,
    pub credits: u128,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct PatientRequest {
    name: String,
    email: String,
    birth_date: u64,
}

#[derive(CandidType)]
pub struct PatientResponse {
    id: PatientId,
    name: String,
    email: String,
    birth_date: u64,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Patient {
    pub fn new(
        e: &PatientRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            name: e.name.clone(),
            email: e.email.clone(),
            birth_date: e.birth_date,
            credits: 0,
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

impl From<Patient> for PatientResponse {
    fn from(
        e: Patient
    ) -> Self {
        Self { 
            id: e.id,
            name: e.name, 
            email: e.email,
            birth_date: e.birth_date,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
