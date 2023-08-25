use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription::{Prescription, PrescriptionId};

pub struct PrescriptionsService {}

impl PrescriptionsService {
    pub fn create(
        prescription: &Prescription,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }

        // validations
        if db.doctors.borrow().find_by_id(&caller).is_none() {
            return Err("Doctor not found".to_string());
        }
    
        if db.patients.borrow().find_by_id(&prescription.patient).is_none() {
            return Err("Patient not found".to_string());
        }
        
        db.prescriptions.borrow_mut().insert(prescription.id.clone(), prescription.clone())
    }

    pub fn update(
        id: &PrescriptionId,
        prescription: &Prescription,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller != prescription.created_by {
            return Err("Forbidden".to_string());
        }

        db.prescriptions.borrow_mut().update(id.to_owned(), prescription.clone())
    }

    pub fn delete(
        id: &PrescriptionId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut prescriptions = db.prescriptions.borrow_mut();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != prescription.created_by {
            return Err("Forbidden".to_string());
        }
        
        prescriptions.delete(id)
    }

    pub fn find_by_id(
        id: &PrescriptionId,
        db: &DB,
        caller: &Principal
    ) -> Result<Prescription, String> {
        let prescriptions = db.prescriptions.borrow();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if prescription.doctor != *caller && prescription.patient != *caller {
            if !Self::has_access(db, &prescription.id, caller) {
                return Err("Forbidden".to_string());
            }
        }

        Ok(prescription.clone())
    }

    pub fn has_access(
        db: &DB, 
        prescription_id: &PrescriptionId, 
        user: &Principal
    ) -> bool {
        let now = ic_cdk::api::time();
        match db.prescription_auths_rel.borrow().find_by_id(prescription_id) {
            None => return false,
            Some(ids) => {
                if !ids.iter().map(|id| 
                    db.prescription_auths.borrow().get(id).clone()
                    ).any(|e| e.to == *user && (e.expires_at.is_none() || now <= e.expires_at.unwrap())) {
                        return false;
                }
            }
        }
        true
    }
}

