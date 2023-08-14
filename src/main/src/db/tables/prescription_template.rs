use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub struct PrescriptionTemplateTable {
    pub data: TableData<PrescriptionTemplateId, PrescriptionTemplate>,
}

impl TableAllocatable<PrescriptionTemplateTable> for PrescriptionTemplateTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}

impl TableDeserializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {}

impl Crud<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplateTable {
    fn get_data(
        &self
    ) -> &TableData<PrescriptionTemplateId, PrescriptionTemplate> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionTemplateId, PrescriptionTemplate> {
        &mut self.data
    }
}