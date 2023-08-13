use std::{rc::Rc, cell::RefCell};
use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber}};
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth};

pub type PrescriptionAuthTable = Table<PrescriptionAuthId, PrescriptionAuth>;

impl TableAllocatable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableSerializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableDeserializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}
impl TableSubscribable for PrescriptionAuthTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

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
            Self::notify(&self.subs.0, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&self.subs.0, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&self.subs.0, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}