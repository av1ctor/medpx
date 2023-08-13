use std::collections::BTreeSet;
use crate::{db::traits::{crud::CRUD, table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey::{Principal, Text, self}, TableSubscriber, Table, TableAllocatable}}, models::prescription::PrescriptionId};
use crate::models::doctor::DoctorId;

pub type DoctorPrescriptionTable = Table<DoctorId, BTreeSet<PrescriptionId>>;
    
impl TableAllocatable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}
impl TableSerializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}
impl TableDeserializable<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {}

impl CRUD<DoctorId, BTreeSet<PrescriptionId>> for DoctorPrescriptionTable {
    fn insert(
        &mut self,
        k: &DoctorId,
        v: &BTreeSet<PrescriptionId>
    ) -> Result<(), String> {
        if self.data.0.contains_key(k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn update(
        &mut self,
        k: &DoctorId,
        v: &BTreeSet<PrescriptionId>
    ) -> Result<(), String> {
        if !self.data.0.contains_key(k) {
            Err("Not found".to_string())
        }
        else {
            self.data.0.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    fn find_by_id(
        &self,
        k: &DoctorId
    ) -> Option<BTreeSet<PrescriptionId>> {
        if !self.data.0.contains_key(k) {
            None
        }
        else {
            Some(self.data.0[k].clone())
        }
    }

    fn get(
        &self,
        k: &DoctorId
    ) -> &BTreeSet<PrescriptionId> {
        self.data.0.get(k).unwrap()
    }

    fn delete(
        &mut self,
        k: &DoctorId
    ) -> Result<(), String> {
        _ = self.data.0.remove(k);
        Ok(())
    }
}

impl TableSubscriber for DoctorPrescriptionTable {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        if let (
                Text(prescription_key), 
                Principal(doctor_key)
            ) = (keys[0].clone(), keys[1].clone()) {
            match kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&doctor_key) {
                        self.data.0.insert(doctor_key.clone(), BTreeSet::new());
                    }

                    let doc_prescriptions = self.data.0
                        .get_mut(&doctor_key).unwrap();
                    doc_prescriptions.insert(prescription_key.clone());
                },
                TableEventKind::Update => {
                    // assuming doctor_key won't be updated
                },
                TableEventKind::Delete => {
                    let doc_prescriptions = self.data.0
                        .get_mut(&doctor_key).unwrap();
                    doc_prescriptions.remove(&prescription_key);
                },
            }
        }
        
        

        /*
        
        */
    }
}