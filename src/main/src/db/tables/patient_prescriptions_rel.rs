use std::collections::{BTreeSet, BTreeMap};
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::{patient::PatientId, prescription::PrescriptionId};

pub struct PatientPrescriptionsRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PatientId, BTreeSet<PrescriptionId>>,
}

impl Table<TableName, PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PatientPrescriptionsRel,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PatientId, BTreeSet<PrescriptionId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PatientId, BTreeSet<PrescriptionId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PatientId, BTreeSet<PrescriptionId>>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl TableVersioned<TableName, PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl TableDeserializable<TableName, PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl Crud<TableName, PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl TableSubscriber<TableName> for PatientPrescriptionsRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Prescriptions => {
                if let (
                        TableEventKey::Text(prescription_key), 
                        TableEventKey::Principal(patient_key)
                    ) = (event.pkey.clone(), event.keys[1].clone()) {
                    match event.kind {
                        TableEventKind::Create => {
                            if !self.data.0.contains_key(&patient_key) {
                                self.data.0.insert(patient_key.clone(), BTreeSet::new());
                            }

                            self.data.0.get_mut(&patient_key).unwrap()
                                .insert(prescription_key.clone());
                        },
                        TableEventKind::Update => {
                            // assuming patient_key won't be updated
                        },
                        TableEventKind::Delete => {
                            self.data.0.get_mut(&patient_key).unwrap()
                                .remove(&prescription_key);
                        },
                    }
                }
            },
            _ => panic!("Unsupported")
        }
    }
}