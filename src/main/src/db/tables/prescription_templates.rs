use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub struct PrescriptionTemplatesTable {
    pub schema: TableSchema,
    pub data: TableData<PrescriptionTemplateId, PrescriptionTemplate>,
}

impl Table<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

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

    fn set_data(
        &mut self,
        data: TableData<PrescriptionTemplateId, PrescriptionTemplate>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl TableVersioned<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<PrescriptionTemplateId, PrescriptionTemplate>, String> {
        panic!("Not supported")
    }
}

impl TableDeserializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl Crud<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}