use std::collections::{BTreeSet, BTreeMap};
use candid::Principal;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::key::KeyId;

pub struct PrincipalKeysRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<Principal, BTreeSet<KeyId>>,
}
    
impl Table<TableName, Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrincipalKeysRel,
            },
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
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl TableVersioned<TableName, Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl TableDeserializable<TableName, Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl Crud<TableName, Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

impl TableSubscriber<TableName> for PrincipalKeysRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        if let (
                TableEventKey::Text(key),
                TableEventKey::Principal(principal) 
            ) = (event.pkey.clone(), event.keys[0].clone()) {
            match event.kind {
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