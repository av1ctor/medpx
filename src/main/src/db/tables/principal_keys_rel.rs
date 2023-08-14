use std::collections::{BTreeSet, BTreeMap};
use candid::Principal;
use crate::{db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableAllocatable, TableData, TableDataAccessible}}, models::key::KeyId};

pub struct PrincipalKeysRelTable {
    pub data: TableData<Principal, BTreeSet<KeyId>>,
}
    
impl TableAllocatable<PrincipalKeysRelTable> for PrincipalKeysRelTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {
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
}

impl TableSerializable<Principal, BTreeSet<KeyId>> for PrincipalKeysRelTable {}

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