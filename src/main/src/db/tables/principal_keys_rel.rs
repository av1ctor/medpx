use std::collections::{BTreeSet, BTreeMap};
use candid::Principal;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::key::KeyId;

pub struct PrincipalKeysRelTable {
    pub schema: TableSchema,
    pub data: TableData<Principal, BTreeSet<KeyId>>,
}
    
impl Table<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<Principal, BTreeSet<KeyId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<Principal, BTreeSet<KeyId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<Principal, BTreeSet<KeyId>>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl TableVersioned<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<Principal, BTreeSet<KeyId>>, String> {
        panic!("Not supported")
    }
}

impl TableDeserializable<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl Crud<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl TableSubscriber for PrincipalKeysRelTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                TableEventKey::Principal(principal), 
                TableEventKey::Text(key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&principal) {
                        self.data.0.insert(principal.clone(), BTreeSet::new());
                    }

                    self.data.0.get_mut(&principal).unwrap()
                        .insert(key.clone());
                },
                TableEventKind::Update => {
                    // assuming principal won't be updated
                },
                TableEventKind::Delete => {
                    self.data.0.get_mut(&principal).unwrap()
                        .remove(&key);
                },
            }
        }
    }
}