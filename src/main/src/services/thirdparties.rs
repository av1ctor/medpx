use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::CrudSubscribable;
use crate::models::thirdparty::{ThirdParty, ThirdPartyId};

pub struct ThirdPartiesService {}

impl ThirdPartiesService {
    pub fn create(
        thirdparty: &ThirdParty,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }
        
        db.thirdparties.borrow_mut().insert(caller.to_owned(), thirdparty.clone())
    }

    pub fn update(
        id: &ThirdPartyId,
        thirdparty: &ThirdParty,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *id != thirdparty.created_by || *caller != thirdparty.created_by {
            return Err("Forbidden".to_string());
        }

        db.thirdparties.borrow_mut().update(id.to_owned(), thirdparty.clone())
    }

    pub fn delete(
        id: &ThirdPartyId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut thirdparties = db.thirdparties.borrow_mut();

        let thirdparty = match thirdparties.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != thirdparty.created_by || *caller != thirdparty.created_by {
            return Err("Forbidden".to_string());
        }
        
        thirdparties.delete(id)
    }

    pub fn find_by_id(
        id: &ThirdPartyId,
        db: &DB
    ) -> Result<ThirdParty, String> {
        match db.thirdparties.borrow().find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => Ok(e.clone())
        }
    }
}