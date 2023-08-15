use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::Crud;
use crate::models::user::{User, UserId};

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

        let created_by = match user.kind {
            crate::models::user::UserKind::Doctor(p) => p,
            crate::models::user::UserKind::Patient(p) => p,
            crate::models::user::UserKind::ThirdParty(p) => p,
            crate::models::user::UserKind::Staff(p) => p,
        };
        
        if *id != created_by || *caller != created_by {
            return Err("Forbidden".to_string());
        }

        Ok((*user).clone())
    }
}