use candid::{CandidType, Principal};
use serde::Deserialize;

pub type ThirdPartyId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub enum ThirdPartyKind {
    Hospital,
    DrugStore,
    Other,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdParty {
    pub id: ThirdPartyId,
    pub kind: ThirdPartyKind,
    pub name: String,
    pub email: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyRequest {
    kind: ThirdPartyKind,
    name: String,
    email: String,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyResponse {
    id: ThirdPartyId,
    kind: ThirdPartyKind,
    name: String,
    email: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl ThirdParty {
    pub fn new(
        e: &ThirdPartyRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            kind: e.kind.clone(),
            name: e.name.clone(),
            email: e.email.clone(),
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

impl From<ThirdParty> for ThirdPartyResponse {
    fn from(
        e: ThirdParty
    ) -> Self {
        Self { 
            id: e.id,
            kind: e.kind,
            name: e.name, 
            email: e.email,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

