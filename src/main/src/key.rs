use std::fmt;

use candid::{CandidType, Principal};
use serde::Deserialize;

pub type KeyId = String;

#[derive(CandidType, Clone, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum KeyKind {
    // unique worldwide 
    EmailAddress,
    PassportNumber,
    Random,
    // unique countrywide
    PhoneNumber,
    IdCardNumber,
    DriverLicenseNumber,
    // FIXME: unique statewide?
    DoctorLicenseNumber,
}

impl fmt::Display for KeyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyKind::EmailAddress => write!(f, "EAD"),
            KeyKind::PassportNumber => write!(f, "PPN"),
            KeyKind::Random => write!(f, "RND"),
            KeyKind::PhoneNumber => write!(f, "PON"),
            KeyKind::IdCardNumber => write!(f, "ICN"),
            KeyKind::DriverLicenseNumber => write!(f, "DLN"),
            KeyKind::DoctorLicenseNumber => write!(f, "DOC"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Key {
    pub id: KeyId,
    pub country: String,
    pub kind: KeyKind,
    pub value: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct KeyRequest {
    pub country: String,
    pub kind: KeyKind,
    pub value: String,
}

#[derive(CandidType)]
pub struct KeyResponse {
    id: KeyId,
    country: String,
    kind: KeyKind,
    value: String,
}

impl Eq for Key {
}

impl PartialEq for Key {
    fn eq(
        &self, 
        other: &Self
    ) -> bool {
        self.kind == other.kind && 
            self.country == other.country &&
                self.value == other.value
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.country.partial_cmp(&other.country) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Key {
    fn cmp(
        &self, 
        other: &Self
    ) -> std::cmp::Ordering {
        match self.country.cmp(&other.country) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord.unwrap(),
        }
        self.value.cmp(&other.value)
    }
}

impl Key {
    pub fn new(
        id: &String,
        e: &KeyRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: id.clone(),
            country: e.country.clone(),
            kind: e.kind.clone(),
            value: e.value.clone(),
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

    pub fn unique_id(
        country: &String,
        kind: &KeyKind,
        value: &String
    ) -> String {
        format!("{}#{}#{}", country, kind, value)
    }
}

impl From<Key> for KeyResponse {
    fn from(
        e: Key
    ) -> Self {
        Self { 
            id: e.id,
            kind: e.kind,
            country: e.country,
            value: e.value, 
        }
    }
}
