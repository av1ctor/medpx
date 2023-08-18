use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::Crud;
use crate::models::user::{User, UserId, UserKind, UserKindResponse};

use super::doctors::DoctorsService;
use super::patients::PatientsService;
use super::staff::StaffService;
use super::thirdparties::ThirdPartiesService;

pub struct UsersService {}

impl UsersService {
    pub fn find_by_id(
        id: &UserId,
        db: &DB,
        caller: &Principal
    ) -> Result<User, String> {
        if *caller == Principal::anonymous() {
            return Err("Forbidden".to_string());
        }
        
        let users = db.users.borrow();

        let user = match users.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        Ok((*user).clone())
    }

    pub fn find_by_kind(
        id: &UserId,
        kind: UserKind,
        db: &DB,
    ) -> UserKindResponse {
        match kind {
            UserKind::Doctor(_) => 
                UserKindResponse::Doctor(DoctorsService::find_by_id(id, db).unwrap().into()),
            UserKind::Patient(_) =>     
                UserKindResponse::Patient(PatientsService::find_by_id(id, db).unwrap().into()),
            UserKind::ThirdParty(_) => 
                UserKindResponse::ThirdParty(ThirdPartiesService::find_by_id(id, db).unwrap().into()),
            UserKind::Staff(_) => 
                UserKindResponse::Staff(StaffService::find_by_id(id, db).unwrap().into()),
        }
    }
}