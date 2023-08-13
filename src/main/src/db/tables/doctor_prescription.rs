use std::{collections::BTreeSet, cell::RefCell, rc::Rc};
use crate::{db::traits::{crud::CRUD, table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::{Principal, self}, TableSubscriber, Table, TableAllocatable}}, models::prescription::PrescriptionId};
use crate::models::doctor::DoctorId;

pub type DoctorPrescriptionTable = Table<DoctorId, BTreeSet<PrescriptionId>>;
    
impl TableAllocatable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}
impl TableSerializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}
impl TableDeserializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}
impl TableSubscribable for DoctorPrescriptionTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {
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
            Self::notify(&self.subs.0, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&self.subs.0, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&self.subs.0, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}

impl TableSubscriber for DoctorPrescriptionTable {
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