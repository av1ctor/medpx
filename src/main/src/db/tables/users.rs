use std::collections::BTreeMap;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubscribable, TableSubs, TableEventKey};
use crate::sdb::crud::{CrudSubscribable, Crud};
use crate::models::user::{UserId, User};

pub struct UsersTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<UserId, User>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, UserId, User> for UsersTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::Users,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<UserId, User> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<UserId, User> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<UserId, User>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, UserId, User> for UsersTable {}

impl TableVersioned<TableName, UserId, User> for UsersTable {}

impl TableDeserializable<TableName, UserId, User> for UsersTable {}

impl Crud<TableName, UserId, User> for UsersTable {}

impl CrudSubscribable<TableName, UserId, User> for UsersTable {}

impl TableSubscribable<TableName, UserId, User> for UsersTable {
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
        k: &UserId
    ) -> TableEventKey {
        TableEventKey::Principal(k.clone())
    }

    fn get_keys(
        _v: &User
    ) -> Vec<TableEventKey> {
        vec![
        ]
    }
}