use std::{cell::RefCell, rc::Rc};
use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::staff::{StaffId, Staff};

pub type StaffTable = Table<StaffId, Staff>;

impl TableAllocatable<StaffId, Staff> for StaffTable {}
impl TableSerializable<StaffId, Staff> for StaffTable {}
impl TableDeserializable<StaffId, Staff> for StaffTable {}
impl TableSubscribable for StaffTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<StaffId, Staff> for StaffTable {
    fn insert(
        &mut self,
        k: &StaffId,
        v: &Staff
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
        k: &StaffId,
        v: &Staff
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
        k: &StaffId
    ) -> Option<Staff> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &StaffId
    ) -> &Staff {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &StaffId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Principal(k.clone())]);
        Ok(())
    }
}