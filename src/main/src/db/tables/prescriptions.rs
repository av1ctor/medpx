use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableSubscribable, TableDeserializable, TableEventKey, TableData, TableSubs, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription::{PrescriptionId, Prescription};

pub struct PrescriptionsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PrescriptionId, Prescription>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, PrescriptionId, Prescription> for PrescriptionsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.2,
                name: TableName::Prescriptions, 
            },
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
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PrescriptionId, Prescription> for PrescriptionsTable {}

impl TableVersioned<TableName, PrescriptionId, Prescription> for PrescriptionsTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<PrescriptionId, Prescription>, String> {
        crate::db::migrations::prescriptions::migrate(from_version, buf)
    }
}

impl TableDeserializable<TableName, PrescriptionId, Prescription> for PrescriptionsTable {}

impl Crud<TableName, PrescriptionId, Prescription> for PrescriptionsTable {}

impl CrudSubscribable<TableName, PrescriptionId, Prescription> for PrescriptionsTable {}

impl TableSubscribable<TableName, PrescriptionId, Prescription> for PrescriptionsTable {
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
        k: &PrescriptionId
    ) -> TableEventKey {
        TableEventKey::Text(k.clone())
    }

    fn get_keys(
        v: &Prescription
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Principal(v.doctor.clone()),
            TableEventKey::Principal(v.patient.clone())
        ]
    }
}