use candid::CandidType;
use serde::Deserialize;
use crate::{patient::Patient, doctor::Doctor, staff::Staff, thirdparty::ThirdParty};

#[derive(CandidType, Clone, Deserialize)]
pub enum User {
    Doctor(Doctor),
    Patient(Patient),
    ThirdParty(ThirdParty),
    Staff(Staff),
}