use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{Crud, Pagination, CrudSubscribable};
use crate::models::prescription::{Prescription, PrescriptionState};
use crate::models::user::{User, UserId, UserKind};

use super::doctors::DoctorsService;

pub struct UsersService {}

impl UsersService {
    pub fn create(
        user: &User,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }

        match user.kind.clone() {
            UserKind::Doctor(doctor) => {
                if let Err(err) = DoctorsService::validate_cert(&doctor.cert.as_bytes().to_vec(), &doctor) {
                    return Err(err);
                }
            },
            _ => ()
        }
        
        db.users.borrow_mut().insert_and_notify(caller.to_owned(), user.clone())
    }

    pub fn update(
        id: &UserId,
        req: &User,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut users = db.users.borrow_mut();

        let user = match users.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != user.created_by || *caller != user.created_by {
            return Err("Forbidden".to_string());
        }

        users.update_and_notify(id.to_owned(), req.clone())
    }

    pub fn delete(
        id: &UserId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut users = db.users.borrow_mut();

        let user = match users.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != user.created_by || *caller != user.created_by {
            return Err("Forbidden".to_string());
        }
        
        users.delete_and_notify(id)
    }

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

    pub fn find_prescriptions(
        id: &UserId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<Prescription>, String> {
        let users = db.users.borrow();

        let user = match users.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != user.created_by || *caller != user.created_by {
            return Err("Forbidden".to_string());
        }

        let ids = match db.user_prescriptions_rel.borrow().find_by_id(id) {
            None => vec![],
            Some(set) =>  {
                let mut arr: Vec<String> = set.iter().cloned().collect();
                arr.sort_by(|a, b| b.cmp(a));
                arr.iter()
                    .skip(pag.offset as usize)
                    .take(pag.limit as usize)
                    .cloned()
                    .collect()
            }
        };

        let prescriptions = db.prescriptions.borrow();
        let user_prescriptions = ids.iter().map(|id| prescriptions.get(id).clone())
            .filter(|p| p.state == PrescriptionState::Signed)
            .collect();
        
        Ok(user_prescriptions)
    }
}