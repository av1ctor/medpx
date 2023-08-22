use std::collections::{BTreeSet, BTreeMap};
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::{thirdparty::ThirdPartyId, prescription::PrescriptionId};

pub struct ThirdPartyPrescriptionsRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<ThirdPartyId, BTreeSet<PrescriptionId>>,
}

impl Table<TableName, ThirdPartyId, BTreeSet<PrescriptionId>> for ThirdPartyPrescriptionsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::ThirdPartyPrescriptionsRel,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<ThirdPartyId, BTreeSet<PrescriptionId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<ThirdPartyId, BTreeSet<PrescriptionId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<ThirdPartyId, BTreeSet<PrescriptionId>>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, ThirdPartyId, BTreeSet<PrescriptionId>> for ThirdPartyPrescriptionsRelTable {}

impl TableVersioned<TableName, ThirdPartyId, BTreeSet<PrescriptionId>> for ThirdPartyPrescriptionsRelTable {}

impl TableDeserializable<TableName, ThirdPartyId, BTreeSet<PrescriptionId>> for ThirdPartyPrescriptionsRelTable {}

impl Crud<TableName, ThirdPartyId, BTreeSet<PrescriptionId>> for ThirdPartyPrescriptionsRelTable {}

impl TableSubscriber<TableName> for ThirdPartyPrescriptionsRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        if let (
                TableEventKey::Text(prescription_key), 
                TableEventKey::Principal(thirdparty_key)
            ) = (event.keys[0].clone(), event.keys[1].clone()) {
            match event.kind {
                TableEventKind::Create => {
                    if !self.data.0.contains_key(&thirdparty_key) {
                        self.data.0.insert(thirdparty_key.clone(), BTreeSet::new());
                    }

                    self.data.0.get_mut(&thirdparty_key).unwrap()
                        .insert(prescription_key.clone());
                },
                TableEventKind::Update => {
                    // assuming thirdparty_key won't be updated
                },
                TableEventKind::Delete => {
                    self.data.0.get_mut(&thirdparty_key).unwrap()
                        .remove(&prescription_key);
                },
            }
        }
    }
}