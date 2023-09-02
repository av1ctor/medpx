use std::collections::BTreeMap;
use candid::Principal;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::sdb::crud::Crud;
use crate::models::key::KeyId;

pub struct KeyPrincipalRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<KeyId, Principal>,
}
    
impl Table<TableName, KeyId, Principal> for KeyPrincipalRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::KeyPrincipalRel,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<KeyId, Principal> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<KeyId, Principal> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<KeyId, Principal>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, KeyId, Principal> for KeyPrincipalRelTable {}

impl TableVersioned<TableName, KeyId, Principal> for KeyPrincipalRelTable {}

impl TableDeserializable<TableName, KeyId, Principal> for KeyPrincipalRelTable {}

impl Crud<TableName, KeyId, Principal> for KeyPrincipalRelTable {}

impl TableSubscriber<TableName> for KeyPrincipalRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Keys => {
                if let (
                        TableEventKey::Text(key),
                        TableEventKey::Principal(principal)
                    ) = (event.pkey.clone(), event.keys[0].clone()) {
                    match event.kind {
                        TableEventKind::Create => {
                            self.data.0
                                .insert(key.clone(), principal.clone());
                        },
                        TableEventKind::Update => {
                            // assuming key won't be updated
                        },
                        TableEventKind::Delete => {
                            self.data.0.remove(&key);
                        },
                    }
                }
            }
            _ => panic!("Unsupported")
        }
    }
}