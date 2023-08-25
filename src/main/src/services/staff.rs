use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::staff::{Staff, StaffId};

pub struct StaffService {}

impl StaffService {
    pub fn create(
        staff: &Staff,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }
        
        db.staff.borrow_mut().insert_and_notify(caller.to_owned(), staff.clone())
    }

    pub fn update(
        id: &StaffId,
        staff: &Staff,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *id != staff.created_by || *caller != staff.created_by {
            return Err("Forbidden".to_string());
        }

        db.staff.borrow_mut().update_and_notify(id.to_owned(), staff.clone())
    }

    pub fn delete(
        id: &StaffId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut staffs = db.staff.borrow_mut();

        let staff = match staffs.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != staff.created_by || *caller != staff.created_by {
            return Err("Forbidden".to_string());
        }
        
        staffs.delete_and_notify(id)
    }

    pub fn find_by_id(
        id: &StaffId,
        db: &DB
    ) -> Result<Staff, String> {
        match db.staff.borrow().find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => Ok(e.clone())
        }
    }
}