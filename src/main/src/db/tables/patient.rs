use std::{rc::Rc, cell::RefCell, collections::BTreeMap};
use crate::db::traits::{crud::CRUD, table::{TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber, TableData, TableSubs}};
use crate::models::patient::{PatientId, Patient};

pub struct PatientTable {
    pub data: TableData<PatientId, Patient>,
    pub subs: TableSubs,
}

impl TableAllocatable<PatientTable> for PatientTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

impl TableSerializable<PatientId, Patient> for PatientTable {}

impl TableDeserializable<PatientId, Patient> for PatientTable {}

impl TableSubscribable for PatientTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<PatientId, Patient> for PatientTable {
    fn insert(
        &mut self,
        k: &PatientId,
        v: &Patient
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
        k: &PatientId,
        v: &Patient
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
        k: &PatientId
    ) -> Option<Patient> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &PatientId
    ) -> &Patient {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PatientId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Principal(k.clone())]);
        Ok(())
    }
}