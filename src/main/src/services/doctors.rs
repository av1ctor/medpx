use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{Crud, CrudSubscribable, Pagination};
use crate::models::doctor::{Doctor, DoctorId};
use crate::models::prescription::Prescription;

pub struct DoctorsService {}

impl DoctorsService {
    pub fn create(
        doctor: &Doctor,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }
        
        db.doctors.borrow_mut().insert(caller.to_owned(), doctor.clone())
    }

    pub fn update(
        id: &DoctorId,
        doctor: &Doctor,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *id != doctor.created_by || *caller != doctor.created_by {
            return Err("Forbidden".to_string());
        }

        db.doctors.borrow_mut().update(id.to_owned(), doctor.clone())
    }

    pub fn delete(
        id: &DoctorId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut doctors = db.doctors.borrow_mut();

        let doctor = match doctors.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != doctor.created_by || *caller != doctor.created_by {
            return Err("Forbidden".to_string());
        }
        
        doctors.delete(id)
    }

    pub fn find_by_id(
        id: &DoctorId,
        db: &DB
    ) -> Result<Doctor, String> {
        match db.doctors.borrow().find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => Ok(e.clone())
        }
    }

    pub fn find_prescriptions(
        id: &DoctorId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<Prescription>, String> {
        let doctors = db.doctors.borrow();

        let doctor = match doctors.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != doctor.created_by || *caller != doctor.created_by {
            return Err("Forbidden".to_string());
        }

        let ids = match db.doctor_prescriptions_rel.borrow().find_by_id(id) {
            None => vec![],
            Some(list) => 
                list.iter()
                    .rev()
                    .skip(pag.offset as usize)
                    .take(pag.limit as usize)
                    .cloned()
                    .collect()
        };
        
        Ok(ids.iter().map(|id| 
            db.prescriptions.borrow().get(id).clone()
        ).collect())
    }
}