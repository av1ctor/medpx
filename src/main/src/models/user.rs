use candid::{Principal, CandidType};
use serde::Deserialize;
use super::doctor::{DoctorResponse, DoctorRequest, Doctor};
use super::patient::{PatientResponse, PatientRequest, Patient};
use super::thirdparty::{ThirdPartyResponse, ThirdPartyRequest, ThirdParty};
use super::staff::{StaffResponse, StaffRequest, Staff};

pub type UserId = Principal;

#[derive(Clone, CandidType, Deserialize)]
pub enum UserKind {
    Doctor(Doctor),
    Patient(Patient),
    ThirdParty(ThirdParty),
    Staff(Staff),
}

#[derive(Clone, CandidType, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub credits: u128,
    pub active: bool,
    pub banned: bool,
    pub kind: UserKind,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum UserKindRequest {
    Doctor(DoctorRequest),
    Patient(PatientRequest),
    ThirdParty(ThirdPartyRequest),
    Staff(StaffRequest),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserRequest {
    name: String,
    email: String,
    kind: UserKindRequest,
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
    id: UserId,
    name: String,
    email: String,
    active: bool,
    banned: bool,
    kind: UserKindResponse,
    created_at: u64,
    updated_at: Option<u64>,
}

impl User {
    pub fn new(
        e: &UserRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            name: e.name.clone(),
            email: e.email.clone(),
            active: true,
            banned: false,
            credits: 0,
            kind: e.kind.clone().into(),
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

impl From<User> for UserResponse {
    fn from(
        e: User
    ) -> Self {
        Self { 
            id: e.id,
            name: e.name, 
            email: e.email,
            active: e.active,
            banned: e.banned,
            kind: e.kind.into(),
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

impl From<UserKindRequest> for UserKind {
    fn from(
        value: UserKindRequest
    ) -> Self {
        match value {
            UserKindRequest::Doctor(k) => UserKind::Doctor(k.into()),
            UserKindRequest::Patient(k) => UserKind::Patient(k.into()),
            UserKindRequest::ThirdParty(k) => UserKind::ThirdParty(k.into()),
            UserKindRequest::Staff(k) => UserKind::Staff(k.into()),
        }
    }
}

impl From<UserKind> for UserKindResponse {
    fn from(
        value: UserKind
    ) -> Self {
        match value {
            UserKind::Doctor(k) => UserKindResponse::Doctor(k.into()),
            UserKind::Patient(k) => UserKindResponse::Patient(k.into()),
            UserKind::ThirdParty(k) => UserKindResponse::ThirdParty(k.into()),
            UserKind::Staff(k) => UserKindResponse::Staff(k.into()),
        }
    }
}