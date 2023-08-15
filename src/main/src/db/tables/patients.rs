use std::collections::BTreeMap;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::patient::{PatientId, Patient};

pub struct PatientsTable {
    pub schema: TableSchema,
    pub data: TableData<PatientId, Patient>,
}

impl Table<PatientId, Patient> for PatientsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

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

    fn set_data(
        &mut self,
        data: TableData<PatientId, Patient>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<PatientId, Patient> for PatientsTable {}

impl TableVersioned<PatientId, Patient> for PatientsTable {}

impl TableDeserializable<PatientId, Patient> for PatientsTable {}

impl Crud<PatientId, Patient> for PatientsTable {}