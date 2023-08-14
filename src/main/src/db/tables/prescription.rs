use std::{cell::RefCell, rc::Rc, collections::BTreeMap};
use crate::db::traits::{crud::CRUD, table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::{Text, Principal}, TableSubscriber, TableAllocatable, TableData, TableSubs}};
use crate::models::prescription::{PrescriptionId, Prescription};

pub struct PrescriptionTable {
    pub data: TableData<PrescriptionId, Prescription>,
    pub subs: TableSubs,
}

impl TableAllocatable<PrescriptionTable> for PrescriptionTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

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
            Self::notify(&self.subs.0, TableEventKind::Create, vec![Text(k.clone()), Principal(v.doctor.clone())]);
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
            Self::notify(&self.subs.0, TableEventKind::Update, vec![Text(k.clone()), Principal(v.doctor.clone())]);
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
        let v = self.data.0.remove(k).unwrap();
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Text(k.clone()), Principal(v.doctor.clone())]);
        Ok(())
    }
}