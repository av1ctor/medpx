use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber}};
use crate::models::prescription::{PrescriptionId, Prescription};

pub type PrescriptionTable<'a> = Table<'a, PrescriptionId, Prescription>;

impl TableAllocatable<'_, PrescriptionId, Prescription> for PrescriptionTable<'_> {}
impl TableSerializable<PrescriptionId, Prescription> for PrescriptionTable<'_> {}
impl TableDeserializable<PrescriptionId, Prescription> for PrescriptionTable<'_> {}
impl TableSubscribable<'_, PrescriptionId, Prescription> for PrescriptionTable<'_> {
    fn subscribe(
        &mut self,
        tb: &'static mut dyn TableSubscriber
    ) {
        self.subs.push(tb);
    }
}

impl CRUD<PrescriptionId, Prescription> for PrescriptionTable<'_> {
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
            Self::notify(&mut self.subs, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&mut self.subs, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&mut self.subs, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}