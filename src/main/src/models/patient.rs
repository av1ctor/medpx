use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct Patient {
    pub birth_date: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct PatientRequest {
    birth_date: u64,
}

#[derive(CandidType)]
pub struct PatientResponse {
    birth_date: u64,
}

impl Patient {
    pub fn new(
        e: &PatientRequest
    ) -> Self {
        Self {
            birth_date: e.birth_date,
        }
    }

    pub fn update(
        &self
    ) -> Self {
        self.clone()
    }
}

impl From<Patient> for PatientResponse {
    fn from(
        e: Patient
    ) -> Self {
        Self { 
            birth_date: e.birth_date,
        }
    }
}

impl From<PatientRequest> for Patient {
    fn from(
        e: PatientRequest
    ) -> Self {
        Self {
            birth_date: e.birth_date,
        }
    }
}
