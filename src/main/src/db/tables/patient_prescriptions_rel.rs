use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableAllocatable, TableData, TableDataAccessible}};
use crate::models::{patient::PatientId, prescription::PrescriptionId};

pub struct PatientPrescriptionsRelTable {
    pub data: TableData<PatientId, BTreeSet<PrescriptionId>>,
}

impl TableAllocatable<PatientPrescriptionsRelTable> for PatientPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {
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
}

impl TableSerializable<PatientId, BTreeSet<PrescriptionId>> for PatientPrescriptionsRelTable {}

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