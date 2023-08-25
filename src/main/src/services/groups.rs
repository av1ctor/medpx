use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::group::{Group, GroupId};

pub struct GroupsService {}

impl GroupsService {
    pub fn create(
        group: &Group,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }
        
        db.groups.borrow_mut().insert_and_notify(group.id.to_owned(), group.clone())
    }

    pub fn update(
        id: &GroupId,
        group: &Group,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller != group.created_by {
            return Err("Forbidden".to_string());
        }

        db.groups.borrow_mut().update_and_notify(id.to_owned(), group.clone())
    }

    pub fn delete(
        id: &GroupId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut groups = db.groups.borrow_mut();

        let group = match groups.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != group.created_by {
            return Err("Forbidden".to_string());
        }
        
        groups.delete_and_notify(id)
    }

    pub fn find_by_id(
        id: &GroupId,
        db: &DB
    ) -> Result<Group, String> {
        match db.groups.borrow().find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => Ok(e.clone())
        }
    }
}