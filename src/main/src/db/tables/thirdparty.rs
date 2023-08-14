use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable}};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub struct ThirdPartyTable {
    pub data: TableData<ThirdPartyId, ThirdParty>,
}

impl TableAllocatable<ThirdPartyTable> for ThirdPartyTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}

impl TableDeserializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}

impl Crud<ThirdPartyId, ThirdParty> for ThirdPartyTable {
    fn get_data(
        &self
    ) -> &TableData<ThirdPartyId, ThirdParty> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<ThirdPartyId, ThirdParty> {
        &mut self.data
    }
}