use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription::PrescriptionId;
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId, PrescriptionAuthTarget};

pub struct PrescriptionAuthsService {}

impl PrescriptionAuthsService {
    pub fn create(
        auth: &PrescriptionAuth,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if db.users.borrow().find_by_id(caller).is_none() {
            return Err("User not found".to_string());
        }

        match &auth.to {
            PrescriptionAuthTarget::User(to) => 
                if db.users.borrow().find_by_id(&to).is_none() {
                    return Err("Target user not found".to_string());
                },
            PrescriptionAuthTarget::Group(to) => 
                if db.groups.borrow().find_by_id(&to).is_none() {
                    return Err("Target group not found".to_string());
                },
        };
        

        let prescriptions = db.prescriptions.borrow();
        let prescription = match prescriptions.find_by_id(&auth.prescription_id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != prescription.patient {
            return Err("Forbidden".to_string());
        }
        
        db.prescription_auths.borrow_mut().insert_and_notify(auth.id.clone(), auth.clone())
    }

    pub fn update(
        id: &PrescriptionAuthId,
        req: &PrescriptionAuth,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut auths = db.prescription_auths.borrow_mut();

        let auth = match auths.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != auth.created_by {
            return Err("Forbidden".to_string());
        }

        auths.update_and_notify(id.to_owned(), req.clone())
    }

    pub fn delete(
        id: &PrescriptionAuthId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut auths = db.prescription_auths.borrow_mut();

        let auth = match auths.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != auth.created_by {
            return Err("Forbidden".to_string());
        }
        
        auths.delete_and_notify(id)
    }

    pub fn find_by_id(
        id: &PrescriptionAuthId,
        db: &DB,
        caller: &Principal
    ) -> Result<PrescriptionAuth, String> {
        let auths = db.prescription_auths.borrow();

        let auth = match auths.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if auth.created_by != *caller {
            return Err("Forbidden".to_string());
        }

        Ok(auth.clone())
    }

    pub fn find_by_prescription(
        id: &PrescriptionId,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<PrescriptionAuth>, String> {
        let rel = db.prescription_auths_rel.borrow();

        let ids = match rel.find_by_id(id) {
            None => return Ok(vec![]),
            Some(e) => e
        };

        let auths: Vec<PrescriptionAuth> = ids.iter().map(|id| 
            db.prescription_auths.borrow().get(id).clone()
        ).collect();
        
        if let Some(first) = auths.first() {
            if first.created_by != *caller {
                return Err("Forbidden".to_string());
            }
        }

        Ok(auths)
    }
}