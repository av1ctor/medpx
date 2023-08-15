use std::collections::BTreeMap;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::doctor::{DoctorId, Doctor};

pub struct DoctorsTable {
    pub schema: TableSchema,
    pub data: TableData<DoctorId, Doctor>,
}

impl Table<DoctorId, Doctor> for DoctorsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.2 },
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

impl TableVersioned<DoctorId, Doctor> for DoctorsTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<DoctorId, Doctor>, String> {
        crate::db::migrations::doctors::migrate(from_version, buf)
    }
}

impl TableDeserializable<DoctorId, Doctor> for DoctorsTable {}

impl Crud<DoctorId, Doctor> for DoctorsTable {}