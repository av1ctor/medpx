use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, Table}};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub struct ThirdPartiesTable {
    pub data: TableData<ThirdPartyId, ThirdParty>,
}

impl Table<ThirdPartyId, ThirdParty> for ThirdPartiesTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }

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

impl TableSerializable<ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl TableDeserializable<ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl Crud<ThirdPartyId, ThirdParty> for ThirdPartiesTable {}