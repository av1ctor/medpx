use std::collections::BTreeMap;
use candid::CandidType;
use serde::Deserialize;
use crate::{models::thirdparty::{ThirdPartyId, ThirdParty}, db::traits::crud::CRUD};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct ThirdPartyTable {
    data: BTreeMap<ThirdPartyId, ThirdParty>,
}

impl CRUD<ThirdPartyId, ThirdParty> for ThirdPartyTable {
    fn insert(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if self.data.contains_key(k) {
            Err("Third party already exists".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if !self.data.contains_key(k) {
            Err("Third party not found".to_string())
        }
        else {
            self.data.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &ThirdPartyId
    ) -> Option<ThirdParty> {
        if !self.data.contains_key(k) {
            None
        }
        else {
            Some(self.data[k].clone())
        }
    }

    fn get(
        &self,
        k: &ThirdPartyId
    ) -> &ThirdParty {
        self.data.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &ThirdPartyId
    ) -> Result<(), String> {
        _ = self.data.remove(k);
        Ok(())
    }
}