use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::doctor::{DoctorId, Doctor};

pub type DoctorTable<'a> = Table<'a, DoctorId, Doctor>;

impl TableAllocatable<'_, DoctorId, Doctor> for DoctorTable<'_> {}
impl TableSerializable<DoctorId, Doctor> for DoctorTable<'_> {}
impl TableDeserializable<DoctorId, Doctor> for DoctorTable<'_> {}
impl TableSubscribable<'_, DoctorId, Doctor> for DoctorTable<'_> {
    fn subscribe(
        &mut self,
        tb: &'static mut dyn TableSubscriber
    ) {
        self.subs.push(tb);
    }
}

impl CRUD<DoctorId, Doctor> for DoctorTable<'_> {
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
            Self::notify(&mut self.subs, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&mut self.subs, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&mut self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}