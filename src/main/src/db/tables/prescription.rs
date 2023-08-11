use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::prescription::{PrescriptionId, Prescription}, db::traits::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct PrescriptionTable {
    data: BTreeMap<PrescriptionId, Prescription>,
}

impl CRUD<PrescriptionId, Prescription> for PrescriptionTable {
    fn insert(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Prescription already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Prescription not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &PrescriptionId
    ) -> Option<Prescription> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &PrescriptionId
    ) -> &Prescription {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PrescriptionId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}