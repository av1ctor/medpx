use std::fmt;

use candid::{CandidType, Principal};
use serde::Deserialize;

pub type KeyId = String;

#[derive(CandidType, Clone, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum KeyKind {
    EmailAddress,
    PhoneNumber,
    Random,
}

impl fmt::Display for KeyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyKind::EmailAddress => write!(f, "EAD"),
            KeyKind::PhoneNumber => write!(f, "PON"),
            KeyKind::Random => write!(f, "RND"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Key {
    pub id: KeyId,
    pub kind: KeyKind,
    pub country: Option<String>,
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
    pub country: Option<String>,
    pub value: String,
}

#[derive(CandidType)]
pub struct KeyResponse {
    id: KeyId,
    kind: KeyKind,
    country: Option<String>,
    value: String,
    created_at: u64,
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
        e: &KeyRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: Key::unique_id(&e.kind, &e.country, &e.value),
            kind: e.kind.clone(),
            country: e.country.clone(),
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
        kind: &KeyKind,
        country: &Option<String>,
        value: &String
    ) -> String {
        if let Some(c) = country {
            format!("{}#{}#{}", c, kind, value)
        } else {
            format!("XX#{}#{}", kind, value)
        }
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
            created_at: e.created_at,
        }
    }
}
