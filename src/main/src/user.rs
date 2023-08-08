use candid::CandidType;
use serde::Deserialize;
use crate::{patient::Patient, doctor::Doctor, staff::Staff};

#[derive(CandidType, Clone, Deserialize)]
pub enum User {
    Doctor(Doctor),
    Patient(Patient),
    Staff(Staff),
}