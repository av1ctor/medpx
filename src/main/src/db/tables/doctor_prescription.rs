use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableAllocatable, TableData}};
use crate::models::{doctor::DoctorId, prescription::PrescriptionId};

pub struct DoctorPrescriptionTable {
    pub data: TableData<DoctorId, BTreeSet<PrescriptionId>>,
}
    
impl TableAllocatable<DoctorPrescriptionTable> for DoctorPrescriptionTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableSerializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}

impl TableDeserializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}

impl Crud<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {
    fn get_data(
        &self
    ) -> &TableData<DoctorId, BTreeSet<PrescriptionId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<DoctorId, BTreeSet<PrescriptionId>> {
        &mut self.data
    }
}

impl TableSubscriber for DoctorPrescriptionTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                TableEventKey::Text(prescription_key), 
                TableEventKey::Principal(doctor_key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&doctor_key) {
                        self.data.0.insert(doctor_key.clone(), BTreeSet::new());
                    }

                    self.data.0.get_mut(&doctor_key).unwrap()
                        .insert(prescription_key.clone());
                },
                TableEventKind::Update => {
                    // assuming doctor_key won't be updated
                },
                TableEventKind::Delete => {
                    self.data.0.get_mut(&doctor_key).unwrap()
                        .remove(&prescription_key);
                },
            }
        }
    }
}