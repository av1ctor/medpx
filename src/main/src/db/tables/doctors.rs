use std::collections::BTreeMap;
use crate::db::traits::{crud::Crud, table::{TableAllocatable, TableSerializable, TableDeserializable, TableData, TableDataAccessible}};
use crate::models::doctor::{DoctorId, Doctor};

pub struct DoctorsTable {
    pub data: TableData<DoctorId, Doctor>,
}

impl TableAllocatable<DoctorsTable> for DoctorsTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<DoctorId, Doctor> for DoctorsTable {
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

    
}

impl TableSerializable<DoctorId, Doctor> for DoctorsTable {}

impl TableDeserializable<DoctorId, Doctor> for DoctorsTable {}

impl Crud<DoctorId, Doctor> for DoctorsTable {}