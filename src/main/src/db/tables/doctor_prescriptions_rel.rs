use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned};
use crate::db::traits::crud::Crud;
use crate::models::{doctor::DoctorId, prescription::PrescriptionId};

pub struct DoctorPrescriptionsRelTable {
    pub data: TableData<DoctorId, BTreeSet<PrescriptionId>>,
    pub schema: TableSchema,
}
    
impl Table<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { version: 0.1 },
            data: TableData(BTreeMap::new()),
        }
    }

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

    fn set_data(
        &mut self,
        data: TableData<DoctorId, BTreeSet<PrescriptionId>>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema {
        &self.schema
    }
}

impl TableSerializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionsRelTable {}

impl TableVersioned<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionsRelTable {
    fn migrate(
        &self,
        from_version: f32,
        buf: &[u8]
    ) -> Result<TableData<DoctorId, BTreeSet<PrescriptionId>>, String> {
        panic!("Not supported")
    }
}

impl TableDeserializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionsRelTable {}

impl Crud<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionsRelTable {}

impl TableSubscriber for DoctorPrescriptionsRelTable {
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