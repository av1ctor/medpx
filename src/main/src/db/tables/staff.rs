use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::staff::{StaffId, Staff};

pub type StaffTable<'a> = Table<'a, StaffId, Staff>;

impl TableAllocatable<'_, StaffId, Staff> for StaffTable<'_> {}
impl TableSerializable<StaffId, Staff> for StaffTable<'_> {}
impl TableDeserializable<StaffId, Staff> for StaffTable<'_> {}
impl TableSubscribable<'_, StaffId, Staff> for StaffTable<'_> {
    fn subscribe(
        &mut self,
        tb: &'static mut dyn TableSubscriber
    ) {
        self.subs.push(tb);
    }
}

impl CRUD<StaffId, Staff> for StaffTable<'_> {
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
            Self::notify(&mut self.subs, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&mut self.subs, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&mut self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}