use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct Patient {
    pub id: String,
    pub name: String,
    pub birth_date: u64,
    pub num_prescriptions: u32,
    pub credits: u128,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct PatientRequest {
    id: String,
    name: String,
    birth_date: u64,
}

#[derive(CandidType)]
pub struct PatientResponse {
    id: String,
    name: String,
    birth_date: u64,
}

impl Patient {
    pub fn new(
        e: &PatientRequest
    ) -> Self {
        Self {
            id: e.id.clone(),
            name: e.name.clone(),
            birth_date: e.birth_date,
            num_prescriptions: 0,
            credits: 0,
            created_at: ic_cdk::api::time(),
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
            birth_date: e.birth_date,
        }
    }
}
