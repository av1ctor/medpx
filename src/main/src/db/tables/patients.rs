use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubscribable, TableSubs, TableEventKey};
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::patient::{PatientId, Patient};

pub struct PatientsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PatientId, Patient>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, PatientId, Patient> for PatientsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.2,
                name: TableName::Patients,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PatientId, Patient> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PatientId, Patient> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PatientId, Patient>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PatientId, Patient> for PatientsTable {}

impl TableVersioned<TableName, PatientId, Patient> for PatientsTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<PatientId, Patient>, String> {
        crate::db::migrations::patients::migrate(from_version, buf)
    }
}

impl TableDeserializable<TableName, PatientId, Patient> for PatientsTable {}

impl Crud<TableName, PatientId, Patient> for PatientsTable {}

impl CrudSubscribable<TableName, PatientId, Patient> for PatientsTable {}

impl TableSubscribable<TableName, PatientId, Patient> for PatientsTable {
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
        k: &PatientId
    ) -> TableEventKey {
        TableEventKey::Principal(k.clone())
    }

    fn get_keys(
        _v: &Patient
    ) -> Vec<TableEventKey> {
        vec![
        ]
    }
}