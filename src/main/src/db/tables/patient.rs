use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::patient::{PatientId, Patient}, db::traits::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct PatientTable {
    pub data: BTreeMap<PatientId, Patient>,
}

impl CRUD<PatientId, Patient> for PatientTable {
    fn insert(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Patient already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Patient not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &PatientId
    ) -> Option<Patient> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &PatientId
    ) -> &Patient {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PatientId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}