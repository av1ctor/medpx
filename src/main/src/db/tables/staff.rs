use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable}};
use crate::models::staff::{StaffId, Staff};

pub struct StaffTable {
    pub data: TableData<StaffId, Staff>,
}

impl TableAllocatable<StaffTable> for StaffTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<StaffId, Staff> for StaffTable {}

impl TableDeserializable<StaffId, Staff> for StaffTable {}

impl Crud<StaffId, Staff> for StaffTable {
    fn get_data(
        &self
    ) -> &TableData<StaffId, Staff> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<StaffId, Staff> {
        &mut self.data
    }
}