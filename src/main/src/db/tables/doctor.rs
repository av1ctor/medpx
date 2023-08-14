use std::collections::BTreeMap;
use crate::db::traits::{crud::Crud, table::{TableAllocatable, TableSerializable, TableDeserializable, TableData}};
use crate::models::doctor::{DoctorId, Doctor};

pub struct DoctorTable {
    pub data: TableData<DoctorId, Doctor>,
}

impl TableAllocatable<DoctorTable> for DoctorTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<DoctorId, Doctor> for DoctorTable {}

impl TableDeserializable<DoctorId, Doctor> for DoctorTable {}

impl Crud<DoctorId, Doctor> for DoctorTable {
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
}