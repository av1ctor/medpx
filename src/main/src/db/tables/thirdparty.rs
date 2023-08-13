use std::{rc::Rc, cell::RefCell};
use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub type ThirdPartyTable = Table<ThirdPartyId, ThirdParty>;

impl TableAllocatable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableSerializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableDeserializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableSubscribable for ThirdPartyTable {
    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.subs.0.push(tb);
    }
}

impl CRUD<ThirdPartyId, ThirdParty> for ThirdPartyTable {
    fn insert(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Create, vec![Principal(k.clone())]);
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Self::notify(&self.subs.0, TableEventKind::Update, vec![Principal(k.clone())]);
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &ThirdPartyId
    ) -> Option<ThirdParty> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &ThirdPartyId
    ) -> &ThirdParty {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &ThirdPartyId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Self::notify(&self.subs.0, TableEventKind::Delete, vec![Principal(k.clone())]);
        Ok(())
    }
}