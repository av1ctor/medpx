use std::collections::BTreeMap;

use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableData, Table, TableSubs, TableSubscribable, TableEventKey}};
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth};

pub struct PrescriptionAuthsTable {
    pub data: TableData<PrescriptionAuthId, PrescriptionAuth>,
    pub subs: TableSubs,
}

impl Table<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
    fn new(
    ) -> Self {
        Self {
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
}

impl TableSerializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableDeserializable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl Crud<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableSubscribable<PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
    fn get_subs(
        &self
    ) -> &TableSubs {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs {
        &mut self.subs
    }

    fn get_keys(
        k: &PrescriptionAuthId,
        v: &PrescriptionAuth
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Text(k.clone()), 
            TableEventKey::Text(v.prescription_id.clone())
        ]
    }
}
