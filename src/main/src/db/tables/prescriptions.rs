use std::collections::BTreeMap;
use crate::db::traits::table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey, TableData, TableSubs, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::CrudSubscribable;
use crate::models::prescription::{PrescriptionId, Prescription};

pub struct PrescriptionsTable {
    pub schema: TableSchema,
    pub data: TableData<PrescriptionId, Prescription>,
    pub subs: TableSubs,
}

impl Table<PrescriptionId, Prescription> for PrescriptionsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PrescriptionId, Prescription> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionId, Prescription> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PrescriptionId, Prescription>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<PrescriptionId, Prescription> for PrescriptionsTable {}

impl TableVersioned<PrescriptionId, Prescription> for PrescriptionsTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<PrescriptionId, Prescription>, String> {
        panic!("Not supported")
    }
}

impl TableDeserializable<PrescriptionId, Prescription> for PrescriptionsTable {}

impl CrudSubscribable<PrescriptionId, Prescription> for PrescriptionsTable {}

impl TableSubscribable<PrescriptionId, Prescription> for PrescriptionsTable {
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
        k: &PrescriptionId,
        v: &Prescription
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Text(k.clone()), 
            TableEventKey::Principal(v.doctor.clone())
        ]
    }
}