use std::{rc::Rc, cell::RefCell, collections::BTreeMap};
use crate::db::traits::{crud::CRUD, table::{TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber, TableData, TableSubs}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub struct PrescriptionTemplateTable {
    pub data: TableData<PrescriptionTemplateId, PrescriptionTemplate>,
    pub subs: TableSubs,
}

impl TableAllocatable<PrescriptionTemplateTable> for PrescriptionTemplateTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

impl TableSerializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}

impl TableDeserializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}

impl TableSubscribable for PrescriptionTemplateTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {
    fn insert(
        &mut self,
        k: &PrescriptionTemplateId,
        v: &PrescriptionTemplate
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Create, vec![Text(k.clone())]);
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &PrescriptionTemplateId,
        v: &PrescriptionTemplate
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Update, vec![Text(k.clone())]);
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &PrescriptionTemplateId
    ) -> Option<PrescriptionTemplate> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &PrescriptionTemplateId
    ) -> &PrescriptionTemplate {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &PrescriptionTemplateId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Text(k.clone())]);
        Ok(())
    }
}