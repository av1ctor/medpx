use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Principal}};
use crate::models::staff::{StaffId, Staff};

pub type StaffTable = Table<StaffId, Staff>;

impl TableAllocator<StaffId, Staff> for StaffTable {}
impl TableSerializer<StaffId, Staff> for StaffTable {}
impl TableDeserializer<StaffId, Staff> for StaffTable {}
impl TableSubscribed<StaffId, Staff> for StaffTable {}

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
            Self::notify(&self.subs, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&self.subs, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}