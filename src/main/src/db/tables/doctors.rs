use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubs, TableSubscribable, TableEventKey};
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::doctor::{DoctorId, Doctor};

pub struct DoctorsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<DoctorId, Doctor>,
    pub subs: TableSubs<TableName>,
}

impl Table<TableName, DoctorId, Doctor> for DoctorsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.2,
                name: TableName::Doctors,
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<DoctorId, Doctor> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<DoctorId, Doctor> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<DoctorId, Doctor>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, DoctorId, Doctor> for DoctorsTable {}

impl TableVersioned<TableName, DoctorId, Doctor> for DoctorsTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<DoctorId, Doctor>, String> {
        crate::db::migrations::doctors::migrate(from_version, buf)
    }
}

impl TableDeserializable<TableName, DoctorId, Doctor> for DoctorsTable {}

impl Crud<TableName, DoctorId, Doctor> for DoctorsTable {}

impl CrudSubscribable<TableName, DoctorId, Doctor> for DoctorsTable {}

impl TableSubscribable<TableName, DoctorId, Doctor> for DoctorsTable {
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
        k: &DoctorId
    ) -> TableEventKey {
        TableEventKey::Principal(k.clone())
    }

    fn get_keys(
        _v: &Doctor
    ) -> Vec<TableEventKey> {
        vec![
        ]
    }
}