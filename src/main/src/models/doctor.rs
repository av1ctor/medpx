use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::models::prescription_template::PrescriptionTemplateId;

pub type DoctorId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub struct Doctor {
    pub id: DoctorId,
    pub license_num: String,
    pub name: String,
    pub email: String,
    pub prescription_template: Option<String>,
    pub credits: u128,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct DoctorRequest {
    license_num: String,
    name: String,
    email: String,
    prescription_template: Option<PrescriptionTemplateId>,
}

#[derive(CandidType)]
pub struct DoctorResponse {
    id: DoctorId,
    license_num: String,
    name: String,
    email: String,
    prescription_template: Option<PrescriptionTemplateId>,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Doctor {
    pub fn new(
        e: &DoctorRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            license_num: e.license_num.clone(),
            name: e.name.clone(),
            email: e.email.clone(),
            prescription_template: None,
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

impl From<Doctor> for DoctorResponse {
    fn from(
        e: Doctor
    ) -> Self {
        Self { 
            id: e.id,
            license_num: e.license_num,
            name: e.name, 
            email: e.email,
            prescription_template: e.prescription_template,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
