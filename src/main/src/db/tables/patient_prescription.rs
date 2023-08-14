use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey::{Principal, Text, self}, TableSubscriber, TableAllocatable, TableData, TableSubs}};
use crate::models::{patient::PatientId, prescription::PrescriptionId};

pub struct PatientPrescriptionTable {
    pub data: TableData<PatientId, BTreeSet<PrescriptionId>>,
    pub subs: TableSubs,
}

impl TableAllocatable<PatientPrescriptionTable> for PatientPrescriptionTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
        }
    }
}

impl TableSerializable<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionTable {}

impl TableDeserializable<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionTable {}

impl Crud<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionTable {
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
}

impl TableSubscriber for PatientPrescriptionTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                Text(prescription_key), 
                Principal(patient_key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&patient_key) {
                        self.data.0.insert(patient_key.clone(), BTreeSet::new());
                    }

                    let doc_prescriptions = self.data.0
                        .get_mut(&patient_key).unwrap();
                    doc_prescriptions.insert(prescription_key.clone());
                },
                TableEventKind::Update => {
                    // assuming patient_key won't be updated
                },
                TableEventKind::Delete => {
                    let doc_prescriptions = self.data.0
                        .get_mut(&patient_key).unwrap();
                    doc_prescriptions.remove(&prescription_key);
                },
            }
        }
    }
}