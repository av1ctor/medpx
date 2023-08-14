use std::{cell::RefCell, rc::Rc, collections::BTreeMap};
use crate::db::traits::{crud::CRUD, table::{TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Text, TableSubscriber, TableSubs, TableData}};
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
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<KeyId, Key> for KeyTable {
    fn insert(
        &mut self,
        k: &KeyId,
        v: &Key
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Create, vec![Text(k.clone())]);
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &KeyId,
        v: &Key
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Update, vec![Text(k.clone())]);
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &KeyId
    ) -> Option<Key> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &KeyId
    ) -> &Key {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &KeyId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Text(k.clone())]);
        Ok(())
    }
}