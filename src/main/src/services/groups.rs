use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud, Pagination};
use crate::models::group::{Group, GroupId};
use crate::models::user::UserId;

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

    pub fn find_all_by_user(
        id: &UserId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<Group>, String> {
        let groups_rel = db.principal_groups_rel.borrow();

        let group_ids = match groups_rel.find_by_id(id) {
            None => return Ok(vec![]),
            Some(e) => e
        };

        let groups = db.groups.borrow();
        let list: Vec<Group> = group_ids.iter()
            .map(|e| groups.find_by_id(e).unwrap())
            .skip(pag.offset as usize)
            .take(pag.limit as usize)
            .cloned()
            .collect();

        if list.len() > 0 && list[0].created_by != *caller {
            return Err("Forbidden".to_string());
        }

        Ok(list)
    }
}