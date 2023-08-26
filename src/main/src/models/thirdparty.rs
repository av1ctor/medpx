use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub enum ThirdPartyKind {
    Hospital,
    DrugStore,
    Other,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdParty {
    pub kind: ThirdPartyKind,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyRequest {
    kind: ThirdPartyKind,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ThirdPartyResponse {
    kind: ThirdPartyKind,
}

impl ThirdParty {
    pub fn new(
        e: &ThirdPartyRequest
    ) -> Self {
        Self {
            kind: e.kind.clone(),
        }
    }

    pub fn update(
        &self
    ) -> Self {
        self.clone()
    }
}

impl From<ThirdParty> for ThirdPartyResponse {
    fn from(
        e: ThirdParty
    ) -> Self {
        Self { 
            kind: e.kind,
        }
    }
}

impl From<ThirdPartyRequest> for ThirdParty {
    fn from(
        e: ThirdPartyRequest
    ) -> Self {
        Self {
            kind: e.kind,
        }
    }
}
