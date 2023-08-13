use std::{rc::Rc, cell::RefCell};
use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub type PrescriptionTemplateTable = Table<PrescriptionTemplateId, PrescriptionTemplate>;

impl TableAllocatable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}
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
            Self::notify(&self.subs.0, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&self.subs.0, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&self.subs.0, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}