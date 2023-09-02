use std::collections::BTreeMap;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned};
use crate::sdb::crud::Crud;
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub struct PrescriptionTemplatesTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PrescriptionTemplateId, PrescriptionTemplate>,
}

impl Table<TableName, PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrescriptionTemplates, 
            },
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
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl TableVersioned<TableName, PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl TableDeserializable<TableName, PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl Crud<TableName, PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}