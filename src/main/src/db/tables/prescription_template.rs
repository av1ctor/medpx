use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Text}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub type PrescriptionTemplateTable = Table<PrescriptionTemplateId, PrescriptionTemplate>;

impl TableAllocator<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}
impl TableSerializer<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}
impl TableDeserializer<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}
impl TableSubscribed<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}

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
            Self::notify(&self.subs, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&self.subs, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&self.subs, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}