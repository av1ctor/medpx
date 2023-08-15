use std::collections::{BTreeSet, BTreeMap};
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::{prescription::PrescriptionId, prescription_auth::PrescriptionAuthId};

pub struct PrescriptionAuthsRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
}
    
impl Table<TableName, PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrescriptionAuthsRel, 
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PrescriptionId, BTreeSet<PrescriptionAuthId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionId, BTreeSet<PrescriptionAuthId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PrescriptionId, BTreeSet<PrescriptionAuthId>>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl TableVersioned<TableName, PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl TableDeserializable<TableName, PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl Crud<TableName, PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl TableSubscriber<TableName> for PrescriptionAuthsRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        if let (
                TableEventKey::Text(prescription_auth_key), 
                TableEventKey::Text(prescription_key)
            ) = (event.pkey.clone(), event.keys[0].clone()) {
            match event.kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&prescription_key) {
                        self.data.0.insert(prescription_key.clone(), BTreeSet::new());
                    }

                    self.data.0.get_mut(&prescription_key).unwrap()
                        .insert(prescription_auth_key.clone());
                },
                TableEventKind::Update => {
                    // assuming doctor_key won't be updated
                },
                TableEventKind::Delete => {
                    self.data.0.get_mut(&prescription_key).unwrap()
                        .remove(&prescription_auth_key);
                },
            }
        }
    }
}