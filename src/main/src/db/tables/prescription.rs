use std::{cell::RefCell, rc::Rc};
use crate::db::traits::{crud::CRUD, table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber, TableData, TableSubs, Table, TableAllocatable}};
use crate::models::prescription::{PrescriptionId, Prescription};

pub type PrescriptionTable = Table<PrescriptionId, Prescription>;

impl TableAllocatable<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableSerializable<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableDeserializable<PrescriptionId, Prescription> for PrescriptionTable {}
impl TableSubscribable for PrescriptionTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

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
            Self::notify(&self.subs.0, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&self.subs.0, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&self.subs.0, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}