use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Text}};
use crate::models::prescription::{PrescriptionId, Prescription};

pub type PrescriptionTable = Table<PrescriptionId, Prescription>;

impl TableAllocator<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableSerializer<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableDeserializer<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableSubscribed<PrescriptionId, Prescription> for PrescriptionTable {}

impl CRUD<PrescriptionId, Prescription> for PrescriptionTable {
    fn insert(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs, TableEventKind::Create, Text(k.clone()));
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs, TableEventKind::Update, Text(k.clone()));
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &PrescriptionId
    ) -> Option<Prescription> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &PrescriptionId
    ) -> &Prescription {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PrescriptionId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}