use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum KeyKind {
    PhoneNumber,
    EmailAddress,
    IdCardNumber,
    DriverLicenseNumber,
    PassportNumber,
    DoctorLicenseNumber,
    Random,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Key {
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
    pub kind: KeyKind,
    pub value: String,
}

#[derive(CandidType)]
pub struct KeyResponse {
    pub kind: KeyKind,
    pub value: String,
}

impl Eq for Key {
}

impl PartialEq for Key {
    fn eq(
        &self, 
        other: &Self
    ) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord.unwrap(),
        }
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Key {
    pub fn new(
        e: &KeyRequest,
        caller: &Principal
    ) -> Self {
        Self {
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
}

impl From<Key> for KeyResponse {
    fn from(
        e: Key
    ) -> Self {
        Self { 
            kind: e.kind,
            value: e.value, 
        }
    }
}
