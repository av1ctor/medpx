use candid::{CandidType, Principal};
use serde::Deserialize;

pub type ThirdPartyId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdParty {
    pub id: ThirdPartyId,
    pub name: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyRequest {
    name: String,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyResponse {
    id: ThirdPartyId,
    name: String,
}

impl ThirdParty {
    pub fn new(
        e: &ThirdPartyRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: caller.clone(),
            name: e.name.clone(),
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
            name: e.name, 
        }
    }
}

