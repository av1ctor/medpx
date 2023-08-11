use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::key::{KeyId, Key}, db::traits::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct KeyTable {
    data: BTreeMap<KeyId, Key>,
}

impl CRUD<KeyId, Key> for KeyTable {
    fn insert(
        &mut self,
        k: &KeyId,
        v: &Key
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Key already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &KeyId,
        v: &Key
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Key not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &KeyId
    ) -> Option<Key> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &KeyId
    ) -> &Key {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &KeyId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}