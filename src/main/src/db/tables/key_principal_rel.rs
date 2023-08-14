use std::collections::BTreeMap;
use candid::Principal;
use crate::{db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema}}, models::key::KeyId};

pub struct KeyPrincipalRelTable {
    pub schema: TableSchema,
    pub data: TableData<KeyId, Principal>,
}
    
impl Table<KeyId, Principal> for KeyPrincipalRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
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
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<KeyId, Principal> for KeyPrincipalRelTable {}

impl TableDeserializable<KeyId, Principal> for KeyPrincipalRelTable {}

impl Crud<KeyId, Principal> for KeyPrincipalRelTable {}

impl TableSubscriber for KeyPrincipalRelTable {
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
}