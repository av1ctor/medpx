use std::collections::BTreeMap;
use candid::Principal;
use crate::{db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableAllocatable, TableData}}, models::key::KeyId};

pub struct KeyPrincipalTable {
    pub data: TableData<KeyId, Principal>,
}
    
impl TableAllocatable<KeyPrincipalTable> for KeyPrincipalTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<KeyId, Principal> for KeyPrincipalTable {}

impl TableDeserializable<KeyId, Principal> for KeyPrincipalTable {}

impl Crud<KeyId, Principal> for KeyPrincipalTable {
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
}

impl TableSubscriber for KeyPrincipalTable {
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