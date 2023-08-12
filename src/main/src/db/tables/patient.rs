use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Principal}};
use crate::models::patient::{PatientId, Patient};

pub type PatientTable = Table<PatientId, Patient>;

impl TableAllocator<PatientId, Patient> for PatientTable {}
impl TableSerializer<PatientId, Patient> for PatientTable {}
impl TableDeserializer<PatientId, Patient> for PatientTable {}
impl TableSubscribed<PatientId, Patient> for PatientTable {}

impl CRUD<PatientId, Patient> for PatientTable {
    fn insert(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs, TableEventKind::Create, Principal(k.clone()));
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs, TableEventKind::Update, Principal(k.clone()));
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &PatientId
    ) -> Option<Patient> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &PatientId
    ) -> &Patient {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PatientId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}