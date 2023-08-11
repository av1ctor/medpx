use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::staff::{StaffId, Staff}, db::traits::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct StaffTable {
    data: BTreeMap<StaffId, Staff>,
}

impl CRUD<StaffId, Staff> for StaffTable {
    fn insert(
        &mut self,
        k: &StaffId,
        v: &Staff
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Staff member already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &StaffId,
        v: &Staff
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Staff member not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &StaffId
    ) -> Option<Staff> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &StaffId
    ) -> &Staff {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &StaffId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}