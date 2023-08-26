use candid::{Principal, CandidType};
use serde::Deserialize;
use super::user::UserId;

pub type GroupId = String;

#[derive(CandidType, Clone, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub members: Vec<UserId>,
    pub created_at: u64,
    pub created_by: UserId,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct GroupRequest {
    pub members: Vec<UserId>,
}

#[derive(CandidType, Clone)]
pub struct GroupResponse {
    id: GroupId,
    members: Vec<UserId>,
    created_by: UserId,
    created_at: u64,
}

impl Group {
    pub fn new(
        id: &String,
        e: &GroupRequest,
        caller: &Principal
    ) -> Self {
        Self { 
            id: id.clone(),
            members: e.members.clone(), 
            created_at: ic_cdk::api::time(), 
            created_by: caller.clone(),
            updated_at: None,
            updated_by: None,
        }
    }
}

impl From<Group> for GroupResponse {
    fn from(
        e: Group
    ) -> Self {
        Self { 
            id: e.id,
            members: e.members, 
            created_by: e.created_by,
            created_at: e.created_at,
        }
    }
}