use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubscribable, TableSubs, TableEventKey};
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

pub struct ThirdPartiesTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<ThirdPartyId, ThirdParty>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.3,
                name: TableName::ThirdParties,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
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
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl TableVersioned<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<ThirdPartyId, ThirdParty>, String> {
        crate::db::migrations::thirdparties::migrate(from_version, buf)
    }
}

impl TableDeserializable<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl Crud<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl CrudSubscribable<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {}

impl TableSubscribable<TableName, ThirdPartyId, ThirdParty> for ThirdPartiesTable {
    fn get_subs(
        &self
    ) -> &TableSubs<TableName> {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs<TableName> {
        &mut self.subs
    }

    fn get_pkey(
        k: &ThirdPartyId
    ) -> TableEventKey {
        TableEventKey::Principal(k.clone())
    }

    fn get_keys(
        _v: &ThirdParty
    ) -> Vec<TableEventKey> {
        vec![
        ]
    }
}