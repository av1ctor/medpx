use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, TableAllocatable, TableDataAccessible}};
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

impl TableDataAccessible<ThirdPartyId, ThirdParty> for ThirdPartyTable {
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

    fn set_data(
        &mut self,
        data: TableData<ThirdPartyId, ThirdParty>
    ) {
        self.data = data;
    }
}

impl TableSerializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}

impl TableDeserializable<ThirdPartyId, ThirdParty> for ThirdPartyTable {}

impl Crud<ThirdPartyId, ThirdParty> for ThirdPartyTable {}