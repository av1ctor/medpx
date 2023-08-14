use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable, TableDataAccessible}};
use crate::models::prescription_template::{PrescriptionTemplateId, PrescriptionTemplate};

pub struct PrescriptionTemplatesTable {
    pub data: TableData<PrescriptionTemplateId, PrescriptionTemplate>,
}

impl TableAllocatable<PrescriptionTemplatesTable> for PrescriptionTemplatesTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {
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
}

impl TableSerializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl TableDeserializable<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}

impl Crud<PrescriptionTemplateId, PrescriptionTemplate> for PrescriptionTemplatesTable {}