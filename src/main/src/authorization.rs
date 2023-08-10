use candid::{Principal, CandidType};
use serde::Deserialize;

pub type AuthorizationId = String;

#[derive(CandidType, Clone, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum AuthorizantionKind {
    Read,
    Write,
    ReadWrite,
    All,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Authorization {
    pub id: String,
    pub kind: AuthorizantionKind,
    pub from: Principal,
    pub to: Principal,
    pub expires_at: Option<u64>,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct AuthorizationRequest {
    pub kind: AuthorizantionKind,
    pub to: Principal,
    pub expires_at: Option<u64>,
}

#[derive(CandidType)]
pub struct AuthorizationResponse {
    pub id: String,
    pub kind: AuthorizantionKind,
    pub from: Principal,
    pub to: Principal,
    pub expires_at: Option<u64>,
}

impl Eq for Authorization {
}

impl PartialEq for Authorization {
    fn eq(
        &self, 
        other: &Self
    ) -> bool {
        self.kind == other.kind && self.from == other.from && self.to == other.to
    }
}

impl PartialOrd for Authorization {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.from.partial_cmp(&other.from) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.to.partial_cmp(&other.to)
    }
}

impl Ord for Authorization {
    fn cmp(
        &self, 
        other: &Self
    ) -> std::cmp::Ordering {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord.unwrap(),
        }
        match self.from.cmp(&other.from) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.to.cmp(&other.to)
    }
}

impl Authorization {
    pub fn new(
        id: &String,
        e: &AuthorizationRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: id.clone(),
            kind: e.kind.clone(),
            from: caller.clone(),
            to: e.to.clone(),
            expires_at: e.expires_at,
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

impl From<Authorization> for AuthorizationResponse {
    fn from(
        e: Authorization
    ) -> Self {
        Self { 
            id: e.id,
            kind: e.kind,
            from: e.from, 
            to: e.to, 
            expires_at: e.expires_at,
        }
    }
}