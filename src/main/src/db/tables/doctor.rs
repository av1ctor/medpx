use std::{cell::RefCell, rc::Rc};
use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::doctor::{DoctorId, Doctor};

pub type DoctorTable = Table<DoctorId, Doctor>;

impl TableAllocatable<DoctorId, Doctor> for DoctorTable {}
impl TableSerializable<DoctorId, Doctor> for DoctorTable {}
impl TableDeserializable<DoctorId, Doctor> for DoctorTable {}
impl TableSubscribable for DoctorTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<DoctorId, Doctor> for DoctorTable {
    fn insert(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Create, vec![Principal(k.clone())]);
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
            Self::notify(&self.subs.0, TableEventKind::Update, vec![Principal(k.clone())]);
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
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Principal(k.clone())]);
        Ok(())
    }
}