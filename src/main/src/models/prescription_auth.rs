use candid::{Principal, CandidType};
use serde::Deserialize;

use crate::models::prescription::PrescriptionId;

pub type PrescriptionAuthId = String;

#[derive(CandidType, Clone, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum PrescriptionAuthKind {
    Read,
    Write,
    ReadWrite,
    All,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct PrescriptionAuth {
    pub id: PrescriptionAuthId,
    pub prescription_id: PrescriptionId,
    pub kind: PrescriptionAuthKind,
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
pub struct PrescritipionAuthRequest {
    pub prescription_id: PrescriptionId,
    pub kind: PrescriptionAuthKind,
    pub to: Principal,
    pub expires_at: Option<u64>,
}

#[derive(CandidType)]
pub struct PrescriptionAuthResponse {
    id: PrescriptionAuthId,
    prescription_id: PrescriptionId,
    kind: PrescriptionAuthKind,
    from: Principal,
    to: Principal,
    expires_at: Option<u64>,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Eq for PrescriptionAuth {
}

impl PartialEq for PrescriptionAuth {
    fn eq(
        &self, 
        other: &Self
    ) -> bool {
        self.kind == other.kind && self.from == other.from && self.to == other.to
    }
}

impl PartialOrd for PrescriptionAuth {
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

impl Ord for PrescriptionAuth {
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

impl PrescriptionAuth {
    pub fn new(
        id: &String,
        e: &PrescritipionAuthRequest,
        caller: &Principal
    ) -> Self {
        Self {
            id: id.clone(),
            prescription_id: e.prescription_id.clone(),
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

impl From<PrescriptionAuth> for PrescriptionAuthResponse {
    fn from(
        e: PrescriptionAuth
    ) -> Self {
        Self { 
            id: e.id,
            prescription_id: e.prescription_id,
            kind: e.kind,
            from: e.from, 
            to: e.to, 
            expires_at: e.expires_at,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}