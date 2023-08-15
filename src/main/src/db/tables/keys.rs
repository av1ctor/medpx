use std::collections::BTreeMap;
use crate::db::traits::table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey, TableSubs, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::CrudSubscribable;
use crate::models::key::{KeyId, Key};

pub struct KeysTable {
    pub schema: TableSchema,
    pub data: TableData<KeyId, Key>,
    pub subs: TableSubs,
}

impl Table<KeyId, Key> for KeysTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
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
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<KeyId, Key> for KeysTable {}

impl TableDeserializable<KeyId, Key> for KeysTable {}

impl TableVersioned<KeyId, Key> for KeysTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<KeyId, Key>, String> {
        panic!("Not supported")
    }
}

impl CrudSubscribable<KeyId, Key> for KeysTable {}

impl TableSubscribable<KeyId, Key> for KeysTable {
    fn get_subs(
        &self
    ) -> &TableSubs {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs {
        &mut self.subs
    }

    fn get_keys(
        k: &KeyId,
        v: &Key
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Text(k.clone()), 
            TableEventKey::Principal(v.created_by.clone())
        ]
    }
}