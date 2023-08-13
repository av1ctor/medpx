use crate::db::traits::{crud::CRUD, table::{Table, TableAllocatable, TableSerializable, TableSubscribable, TableDeserializable, TableEventKind, TableEventKey::Principal, TableSubscriber}};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub type ThirdPartyTable<'a> = Table<'a, ThirdPartyId, ThirdParty>;

impl TableAllocatable<'_, ThirdPartyId, ThirdParty> for ThirdPartyTable<'_> {}
impl TableSerializable<ThirdPartyId, ThirdParty> for ThirdPartyTable<'_> {}
impl TableDeserializable<ThirdPartyId, ThirdParty> for ThirdPartyTable<'_> {}
impl TableSubscribable<'_, ThirdPartyId, ThirdParty> for ThirdPartyTable<'_> {
    fn subscribe(
        &mut self,
        tb: &'static mut dyn TableSubscriber
    ) {
        self.subs.push(tb);
    }
}

impl CRUD<ThirdPartyId, ThirdParty> for ThirdPartyTable<'_> {
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
            Self::notify(&mut self.subs, TableEventKind::Create, Principal(k.clone()));
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
            Self::notify(&mut self.subs, TableEventKind::Update, Principal(k.clone()));
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
        Self::notify(&mut self.subs, TableEventKind::Delete, Principal(k.clone()));
        Ok(())
    }
}