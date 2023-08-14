use std::collections::BTreeMap;
use crate::db::traits::{crud::CrudSubscribable, table::{TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKey::Text, TableSubs, TableData}};
use crate::models::key::{KeyId, Key};

pub struct KeyTable {
    pub data: TableData<KeyId, Key>,
    pub subs: TableSubs,
}

impl TableAllocatable<KeyTable> for KeyTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

impl TableSerializable<KeyId, Key> for KeyTable {}

impl TableDeserializable<KeyId, Key> for KeyTable {}

impl TableSubscribable for KeyTable {
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
}

impl CrudSubscribable<KeyId, Key> for KeyTable {
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

    fn get_subs(
        &self
    ) -> &TableSubs {
        &self.subs
    }

    fn get_keys(
        k: &KeyId,
        _v: &Key
    ) -> Vec<crate::db::traits::table::TableEventKey> {
        vec![Text(k.clone())]
    }
}