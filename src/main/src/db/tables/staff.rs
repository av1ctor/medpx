use std::collections::BTreeMap;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::staff::{StaffId, Staff};

pub struct StaffTable {
    pub schema: TableSchema,
    pub data: TableData<StaffId, Staff>,
}

impl Table<StaffId, Staff> for StaffTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

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

    fn set_data(
        &mut self,
        data: TableData<StaffId, Staff>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<StaffId, Staff> for StaffTable {}

impl TableVersioned<StaffId, Staff> for StaffTable {}

impl TableDeserializable<StaffId, Staff> for StaffTable {}

impl Crud<StaffId, Staff> for StaffTable {}