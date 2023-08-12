use crate::db::traits::{crud::CRUD, table::{Table, TableAllocator, TableSerializer, TableSubscribed, TableDeserializer, TableEventKind, TableEventKey::Principal}};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub type ThirdPartyTable = Table<ThirdPartyId, ThirdParty>;

impl TableAllocator<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableSerializer<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableDeserializer<ThirdPartyId, ThirdParty> for ThirdPartyTable {}
impl TableSubscribed<ThirdPartyId, ThirdParty> for ThirdPartyTable {}

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
            Self::notify(&self.subs, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&self.subs, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}