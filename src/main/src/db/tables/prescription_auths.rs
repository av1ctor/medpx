use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSubs, TableSubscribable, TableEventKey, TableSchema, TableVersioned};
use crate::db::traits::crud::CrudSubscribable;
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth};

pub struct PrescriptionAuthsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PrescriptionAuthId, PrescriptionAuth>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrescriptionAuths, 
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PrescriptionAuthId, PrescriptionAuth> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionAuthId, PrescriptionAuth> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PrescriptionAuthId, PrescriptionAuth>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableVersioned<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableDeserializable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl CrudSubscribable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableSubscribable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
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
        k: &PrescriptionAuthId
    ) -> TableEventKey {
        TableEventKey::Text(k.clone())
    }

    fn get_keys(
        v: &PrescriptionAuth
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Text(v.prescription_id.clone()),
            TableEventKey::Principal(v.to),
        ]
    }
}
