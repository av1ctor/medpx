use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct Doctor {
    pub id: String,
    pub name: String,
    pub num_prescriptions: u32,
    pub credits: u128,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DoctorRequest {
    id: String,
    name: String,
}

#[derive(CandidType)]
pub struct DoctorResponse {
    id: String,
    name: String,
}

impl Doctor {
    pub fn new(
        e: &DoctorRequest
    ) -> Self {
        Self {
            id: e.id.clone(),
            name: e.name.clone(),
            num_prescriptions: 0,
            credits: 0,
            created_at: ic_cdk::api::time(),
        }
    }
}

impl From<Doctor> for DoctorResponse {
    fn from(
        e: Doctor
    ) -> Self {
        Self { 
            id: e.id,
            name: e.name, 
        }
    }
}
