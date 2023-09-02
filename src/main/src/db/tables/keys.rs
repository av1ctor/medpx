use std::collections::BTreeMap;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey, TableSubs, TableData, Table, TableSchema, TableVersioned};
use crate::sdb::crud::{CrudSubscribable, Crud};
use crate::models::key::{KeyId, Key};

pub struct KeysTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<KeyId, Key>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, KeyId, Key> for KeysTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::Keys,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<KeyId, Key> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<KeyId, Key> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<KeyId, Key>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, KeyId, Key> for KeysTable {}

impl TableDeserializable<TableName, KeyId, Key> for KeysTable {}

impl TableVersioned<TableName, KeyId, Key> for KeysTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<KeyId, Key>, String> {
        crate::db::migrations::keys::migrate(from_version, buf)
    }
}

impl Crud<TableName, KeyId, Key> for KeysTable {}

impl CrudSubscribable<TableName, KeyId, Key> for KeysTable {}

impl TableSubscribable<TableName, KeyId, Key> for KeysTable {
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
        k: &KeyId
    ) -> TableEventKey {
        TableEventKey::Text(k.clone())
    }

    fn get_keys(
        v: &Key
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Principal(v.created_by.clone())
        ]
    }
}