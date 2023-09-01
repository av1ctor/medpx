use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud, Pagination};
use crate::models::key::{Key, KeyId, KeyKind};
use crate::models::user::UserId;

pub struct KeysService {}

impl KeysService {
    pub fn create(
        key: &Key,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }

        db.keys.borrow_mut().insert_and_notify(key.id.clone(), key.clone())
    }

    pub fn update(
        id: &KeyId,
        req: &Key,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut keys = db.keys.borrow_mut();

        let key = match keys.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != key.created_by {
            return Err("Forbidden".to_string());
        }

        keys.update_and_notify(id.to_owned(), req.clone())
    }

    pub fn delete(
        id: &KeyId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut keys = db.keys.borrow_mut();

        let key = match keys.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != key.created_by {
            return Err("Forbidden".to_string());
        }
        
        keys.delete_and_notify(id)
    }

    pub fn find_by_id(
        id: &KeyId,
        db: &DB,
        _caller: &Principal
    ) -> Result<Key, String> {
        let keys = db.keys.borrow();

        let key = match keys.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        Ok(key.clone())
    }

    pub fn find_by_value(
        kind: &KeyKind,
        country: &Option<String>,
        value: &String,
        db: &DB,
        _caller: &Principal
    ) -> Result<Key, String> {
        let keys = db.keys.borrow();

        let id = Key::unique_id(kind, country, value);

        let key = match keys.find_by_id(&id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        Ok(key.clone())
    }

    pub fn find_all_by_user(
        id: &UserId,
        pag: Pagination,
        db: &DB,
        caller: &Principal
    ) -> Result<Vec<Key>, String> {
        let keys_rel = db.principal_keys_rel.borrow();

        let list: Vec<_> = match keys_rel.find_by_id(id) {
            None => return Ok(vec![]),
            Some(ids) => {
                let keys = db.keys.borrow();
                let mut arr: Vec<String> = ids.iter().cloned().collect();
                arr.sort_by(|a, b| b.cmp(a));
                arr.iter()
                    .map(|e| keys.find_by_id(e).unwrap())
                    .skip(pag.offset as usize)
                    .take(pag.limit as usize)
                    .cloned()
                    .collect()
            }
        };

        if list.len() > 0 && list[0].created_by != *caller {
            return Err("Forbidden".to_string());
        }

        Ok(list)
    }
}