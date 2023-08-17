use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{Crud, CrudSubscribable, Pagination};
use crate::models::patient::{Patient, PatientId};
use crate::models::prescription::PrescriptionId;

pub struct PatientsService {}

impl PatientsService {
    pub fn create(
        patient: &Patient,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }
        
        db.patients.borrow_mut().insert(caller.to_owned(), patient.clone())
    }

    pub fn update(
        id: &PatientId,
        patient: &Patient,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *id != patient.created_by || *caller != patient.created_by {
            return Err("Forbidden".to_string());
        }

        db.patients.borrow_mut().update(id.to_owned(), patient.clone())
    }

    pub fn delete(
        id: &PatientId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut patients = db.patients.borrow_mut();

        let patient = match patients.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != patient.created_by || *caller != patient.created_by {
            return Err("Forbidden".to_string());
        }
        
        patients.delete(id)
    }

    pub fn find_by_id(
        id: &PatientId,
        db: &DB
    ) -> Result<Patient, String> {
        match db.patients.borrow().find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => Ok(e.clone())
        }
    }

    pub fn find_prescriptions(
        id: &PatientId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<PrescriptionId>, String> {
        let patients = db.patients.borrow();

        let patient = match patients.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != patient.created_by || *caller != patient.created_by {
            return Err("Forbidden".to_string());
        }

        Ok(match db.patient_prescriptions_rel.borrow().find_by_id(id) {
            None => vec![],
            Some(list) =>
                list.iter()
                    .skip(pag.offset as usize)
                    .take(pag.limit as usize)
                    .map(|e| e.clone())
                    .collect()
        })
    }
}