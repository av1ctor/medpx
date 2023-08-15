
use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{Crud, Pagination};
use crate::models::doctor::{Doctor, DoctorId};
use crate::models::prescription::PrescriptionId;

pub struct DoctorService {}

impl DoctorService {
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

    pub fn find_prescriptions(
        id: &DoctorId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<PrescriptionId>, String> {
        let doctors = db.doctors.borrow();

        let doctor = match doctors.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *id != doctor.created_by || *caller != doctor.created_by {
            return Err("Forbidden".to_string());
        }

        Ok(db.doctor_prescriptions_rel.borrow().get(id)
            .iter()
            .skip(pag.offset as usize)
            .take(pag.limit as usize)
            .map(|e| e.clone())
            .collect()
        )
    }
}