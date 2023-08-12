use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Text}};
use crate::models::key::{KeyId, Key};

pub type KeyTable = Table<KeyId, Key>;

impl TableAllocator<KeyId, Key> for KeyTable {}
impl TableSerializer<KeyId, Key> for KeyTable {}
impl TableDeserializer<KeyId, Key> for KeyTable {}
impl TableSubscribed<KeyId, Key> for KeyTable {}

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
            Self::notify(&self.subs, TableEventKind::Create, Text(k.clone()));
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
            Self::notify(&self.subs, TableEventKind::Update, Text(k.clone()));
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
        Self::notify(&self.subs, TableEventKind::Delete, Text(k.clone()));
        Ok(())
    }
}