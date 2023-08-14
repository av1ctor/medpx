use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable}};
use crate::models::patient::{PatientId, Patient};

pub struct PatientTable {
    pub data: TableData<PatientId, Patient>,
}

impl TableAllocatable<PatientTable> for PatientTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<PatientId, Patient> for PatientTable {}

impl TableDeserializable<PatientId, Patient> for PatientTable {}

impl Crud<PatientId, Patient> for PatientTable {
    fn get_data(
        &self
    ) -> &TableData<PatientId, Patient> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PatientId, Patient> {
        &mut self.data
    }
}