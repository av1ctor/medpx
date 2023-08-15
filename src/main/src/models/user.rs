use candid::{Principal, CandidType};
use serde::Deserialize;
use super::{doctor::{DoctorResponse, DoctorId}, patient::{PatientResponse, PatientId}, thirdparty::{ThirdPartyResponse, ThirdPartyId}, staff::{StaffResponse, StaffId}};

pub type UserId = Principal;

#[derive(CandidType, Deserialize)]
pub enum UserKind {
    Doctor(DoctorId),
    Patient(PatientId),
    ThirdParty(ThirdPartyId),
    Staff(StaffId),
}

#[derive(CandidType, Deserialize)]
pub struct User {
    pub kind: UserKind,
    pub active: bool,
    pub banned: bool,
}

#[derive(CandidType)]
pub enum UserKindResponse {
    Doctor(DoctorResponse),
    Patient(PatientResponse),
    ThirdParty(ThirdPartyResponse),
    Staff(StaffResponse),
}

#[derive(CandidType)]
pub struct UserResponse {
    pub kind: UserKindResponse,
    pub active: bool,
    pub banned: bool,
}