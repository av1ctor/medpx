use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubscribable, TableSubs, TableEventKey};
use crate::db::traits::crud::CrudSubscribable;
use crate::models::staff::{StaffId, Staff};

pub struct StaffTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<StaffId, Staff>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, StaffId, Staff> for StaffTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema {
                version: 0.1,
                name: TableName::Staff,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
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
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, StaffId, Staff> for StaffTable {}

impl TableVersioned<TableName, StaffId, Staff> for StaffTable {}

impl TableDeserializable<TableName, StaffId, Staff> for StaffTable {}

impl CrudSubscribable<TableName, StaffId, Staff> for StaffTable {}

impl TableSubscribable<TableName, StaffId, Staff> for StaffTable {
    fn get_subs(
        &self
    ) -> &TableSubs<TableName> {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs<TableName> {
        &mut self.subs
    }

    fn get_pkey(
        k: &StaffId
    ) -> TableEventKey {
        TableEventKey::Principal(k.clone())
    }

    fn get_keys(
        _v: &Staff
    ) -> Vec<TableEventKey> {
        vec![
        ]
    }
}