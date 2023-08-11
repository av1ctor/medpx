use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Principal}};
use crate::models::doctor::{DoctorId, Doctor};

pub type DoctorTable = Table<DoctorId, Doctor>;

impl TableAllocator<DoctorId, Doctor> for DoctorTable {}
impl TableSerializer<DoctorId, Doctor> for DoctorTable {}
impl TableDeserializer<DoctorId, Doctor> for DoctorTable {}
impl TableSubscribed<DoctorId, Doctor> for DoctorTable {}

impl CRUD<DoctorId, Doctor> for DoctorTable {
    fn insert(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated definition".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::alert(&self.subs, TableEventKind::Create, Principal(k.clone()));
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::alert(&self.subs, TableEventKind::Update, Principal(k.clone()));
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &DoctorId
    ) -> Option<Doctor> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &DoctorId
    ) -> &Doctor {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &DoctorId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::alert(&self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}