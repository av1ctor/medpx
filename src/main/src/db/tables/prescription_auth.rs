use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Text}};
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth};

pub type PrescriptionAuthTable = Table<PrescriptionAuthId, PrescriptionAuth>;

impl TableAllocator<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableSerializer<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableDeserializer<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableSubscribed<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}

impl CRUD<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {
    fn insert(
        &mut self,
        k: &PrescriptionAuthId,
        v: &PrescriptionAuth
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
        k: &PrescriptionAuthId,
        v: &PrescriptionAuth
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
        k: &PrescriptionAuthId
    ) -> Option<PrescriptionAuth> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &PrescriptionAuthId
    ) -> &PrescriptionAuth {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PrescriptionAuthId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}