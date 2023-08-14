use std::collections::{BTreeSet, BTreeMap};
use crate::db::traits::{crud::Crud, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableAllocatable, TableData, TableDataAccessible}};
use crate::models::{prescription::PrescriptionId, prescription_auth::PrescriptionAuthId};

pub struct PrescriptionAuthsRelTable {
    pub data: TableData<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
}
    
impl TableAllocatable<PrescriptionAuthsRelTable> for PrescriptionAuthsRelTable {
    fn new(
    ) -> Self {
        Self {
            data: TableData(BTreeMap::new()),
        }
    }
}

impl TableDataAccessible<PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {
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

    
}

impl TableSerializable<PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl TableDeserializable<PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl Crud<PrescriptionId, BTreeSet<PrescriptionAuthId>> for PrescriptionAuthsRelTable {}

impl TableSubscriber for PrescriptionAuthsRelTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                TableEventKey::Text(prescription_auth_key), 
                TableEventKey::Text(prescription_key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
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