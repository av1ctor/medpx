use std::collections::BTreeMap;
use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema}};
use crate::models::doctor::{DoctorId, Doctor};

pub struct DoctorsTable {
    pub schema: TableSchema,
    pub data: TableData<DoctorId, Doctor>,
}

impl Table<DoctorId, Doctor> for DoctorsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<DoctorId, Doctor> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<DoctorId, Doctor> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<DoctorId, Doctor>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<DoctorId, Doctor> for DoctorsTable {}

impl TableDeserializable<DoctorId, Doctor> for DoctorsTable {}

impl Crud<DoctorId, Doctor> for DoctorsTable {}