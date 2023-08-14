use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable, TableDataAccessible}};
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth};

pub struct PrescriptionAuthTable {
    pub data: TableData<PrescriptionAuthId, PrescriptionAuth>,
}

impl TableAllocatable<PrescriptionAuthTable> for PrescriptionAuthTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {
    fn get_data(
        &self
    ) -> &TableData<PrescriptionAuthId, PrescriptionAuth> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionAuthId, PrescriptionAuth> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PrescriptionAuthId, PrescriptionAuth>
    ) {
        self.data = data;
    }
}

impl TableSerializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}

impl TableDeserializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}

impl Crud<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthTable {}