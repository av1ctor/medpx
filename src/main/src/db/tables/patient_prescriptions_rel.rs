use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::{patient::PatientId, prescription::PrescriptionId};

pub struct PatientPrescriptionsRelTable {
    pub schema: TableSchema,
    pub data: TableData<PatientId, BTreeSet<PrescriptionId>>,
}

impl Table<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
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
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl TableVersioned<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<PatientId, BTreeSet<PrescriptionId>>, String> {
        panic!("Not supported")
    }
}

impl TableDeserializable<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl Crud<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

impl TableSubscriber for PatientPrescriptionsRelTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                TableEventKey::Text(prescription_key), 
                TableEventKey::Principal(patient_key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
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
    }
}