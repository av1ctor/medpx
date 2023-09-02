use std::collections::BTreeMap;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey, TableData, TableSubs, Table, TableSchema, TableVersioned};
use crate::sdb::crud::{CrudSubscribable, Crud};
use crate::models::group::{GroupId, Group};

pub struct GroupsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<GroupId, Group>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, GroupId, Group> for GroupsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::Groups, 
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<GroupId, Group> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<GroupId, Group> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<GroupId, Group>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, GroupId, Group> for GroupsTable {}

impl TableVersioned<TableName, GroupId, Group> for GroupsTable {}

impl TableDeserializable<TableName, GroupId, Group> for GroupsTable {}

impl Crud<TableName, GroupId, Group> for GroupsTable {}

impl CrudSubscribable<TableName, GroupId, Group> for GroupsTable {}

impl TableSubscribable<TableName, GroupId, Group> for GroupsTable {
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
        k: &GroupId
    ) -> TableEventKey {
        TableEventKey::Text(k.clone())
    }

    fn get_keys(
        v: &Group
    ) -> Vec<TableEventKey> {
        // group creator must be included
        let mut res = vec![
            TableEventKey::Principal(v.created_by.clone())
        ];
        res.append(&mut v.members.iter().map(|m| TableEventKey::Principal(m.clone())).collect());

        res
    }
}