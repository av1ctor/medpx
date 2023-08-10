use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::doctor::{DoctorId, Doctor}, db::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct DoctorTable {
    data: BTreeMap<DoctorId, Doctor>
}

impl CRUD<DoctorId, Doctor> for DoctorTable {
    fn insert(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Doctor already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Doctor not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &DoctorId
    ) -> Option<Doctor> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &DoctorId
    ) -> &Doctor {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &DoctorId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}