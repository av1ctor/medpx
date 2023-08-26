use std::collections::{BTreeSet, BTreeMap};
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::{user::UserId, prescription::PrescriptionId};

pub struct UserPrescriptionsRelTable {
    pub data: TableData<UserId, BTreeSet<PrescriptionId>>,
    pub schema: TableSchema<TableName>,
}
    
impl Table<TableName, UserId, BTreeSet<PrescriptionId>> for UserPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::UserPrescriptionsRel,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<UserId, BTreeSet<PrescriptionId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<UserId, BTreeSet<PrescriptionId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<UserId, BTreeSet<PrescriptionId>>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, UserId, BTreeSet<PrescriptionId>> for UserPrescriptionsRelTable {}

impl TableVersioned<TableName, UserId, BTreeSet<PrescriptionId>> for UserPrescriptionsRelTable {}

impl TableDeserializable<TableName, UserId, BTreeSet<PrescriptionId>> for UserPrescriptionsRelTable {}

impl Crud<TableName, UserId, BTreeSet<PrescriptionId>> for UserPrescriptionsRelTable {}

impl TableSubscriber<TableName> for UserPrescriptionsRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Prescriptions => {
                if let (
                        TableEventKey::Text(prescription_key), 
                        TableEventKey::Principal(doctor_key),
                        TableEventKey::Principal(patient_key)
                    ) = (event.pkey.clone(), event.keys[0].clone(), event.keys[1].clone()) {
                    match event.kind {
                        TableEventKind::Create => {
                            // doctor
                            if !self.data.0.contains_key(&doctor_key) {
                                self.data.0.insert(doctor_key.clone(), BTreeSet::new());
                            }

                            self.data.0.get_mut(&doctor_key).unwrap()
                                .insert(prescription_key.clone());

                            // patient
                            if !self.data.0.contains_key(&patient_key) {
                                self.data.0.insert(patient_key.clone(), BTreeSet::new());
                            }

                            self.data.0.get_mut(&patient_key).unwrap()
                                .insert(prescription_key.clone());
                        },
                        TableEventKind::Update => {
                            // assuming user_key won't be updated
                        },
                        TableEventKind::Delete => {
                            self.data.0.get_mut(&doctor_key).unwrap()
                                .remove(&prescription_key);
                            self.data.0.get_mut(&patient_key).unwrap()
                                .remove(&prescription_key);
                        },
                    }
                }
            },
            TableName::PrescriptionAuths => {
                if let (
                        TableEventKey::Text(prescription_key), 
                        TableEventKey::Principal(to_key),
                    ) = (event.keys[0].clone(), event.keys[1].clone()) {
                    match event.kind {
                        TableEventKind::Create => {
                            // to
                            if !self.data.0.contains_key(&to_key) {
                                self.data.0.insert(to_key.clone(), BTreeSet::new());
                            }

                            self.data.0.get_mut(&to_key).unwrap()
                                .insert(prescription_key.clone());
                        },
                        TableEventKind::Update => {
                            // assuming user_key won't be updated
                        },
                        TableEventKind::Delete => {
                            self.data.0.get_mut(&to_key).unwrap()
                                .remove(&prescription_key);
                        },
                    }
                }
            },
            _ => panic!("Unsupported")
        }
    }
}