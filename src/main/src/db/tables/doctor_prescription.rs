use std::collections::BTreeSet;
use crate::{db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::{Principal, self}, TableSubscriber}}, models::prescription::PrescriptionId};
use crate::models::doctor::DoctorId;

pub type DoctorPrescriptionTable<'a> = Table<'a, DoctorId, BTreeSet<PrescriptionId>>;

impl TableAllocatable<'_, DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable<'_> {}
impl TableSerializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable<'_> {}
impl TableDeserializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable<'_> {}
impl TableSubscribable<'_, DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable<'_> {
    fn subscribe(
        &mut self,
        tb: &'static mut dyn TableSubscriber
    ) {
        self.subs.push(tb);
    }
}

impl CRUD<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable<'_> {
    fn insert(
        &mut self,
        k: &DoctorId,
        v: &BTreeSet<PrescriptionId>
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
        v: &BTreeSet<PrescriptionId>
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
    ) -> Option<BTreeSet<PrescriptionId>> {
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
    ) -> &BTreeSet<PrescriptionId> {
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

impl TableSubscriber for DoctorPrescriptionTable<'_> {
    fn on(
        &mut self,
        kind: TableEventKind,
        key: TableEventKey
    ) {
        /*if !self.data.0.contains_key(&doctor) {
            self.data.0.insert(doctor.clone(), BTreeSet::new());
        }
        
        let doc_prescriptions = self.data.0
            .get_mut(&doctor).unwrap();
        doc_prescriptions.insert(k.clone());*/
    }
}