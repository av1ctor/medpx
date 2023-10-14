use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSubs, TableSubscribable, TableEventKey, TableSchema, TableVersioned, TableSubscriber, TableEvent, TableEventKind};
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription_auth::{PrescriptionAuthId, PrescriptionAuth, PrescriptionAuthSubject};
use super::prescription_auths_rel::PrescriptionAuthsRelTable;

pub struct PrescriptionAuthsTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<PrescriptionAuthId, PrescriptionAuth>,
    pub subs: TableSubs<TableName>,
    pub aux: Option<Rc<RefCell<PrescriptionAuthsRelTable>>>,
}

impl PrescriptionAuthsTable {
    pub fn set_aux(
        &mut self,
        aux: Rc<RefCell<PrescriptionAuthsRelTable>>
    ) {
        self.aux = Some(aux);
    }
}

impl Table<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrescriptionAuths, 
            },
            data: TableData(BTreeMap::new()),
            subs: TableSubs(Vec::new()),
            aux: None,
        }
    }

    fn get_data(
        &self
    ) -> &TableData<PrescriptionAuthId, PrescriptionAuth> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<PrescriptionAuthId, PrescriptionAuth> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<PrescriptionAuthId, PrescriptionAuth>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableVersioned<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableDeserializable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl Crud<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl CrudSubscribable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {}

impl TableSubscribable<TableName, PrescriptionAuthId, PrescriptionAuth> for PrescriptionAuthsTable {
    fn get_subs(
        &self
    ) -> &TableSubs<TableName> {
        &self.subs
    }

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs<TableName> {
        &mut self.subs
    }

    fn get_pkey(
        k: &PrescriptionAuthId
    ) -> TableEventKey {
        TableEventKey::Text(k.clone())
    }

    fn get_keys(
        v: &PrescriptionAuth
    ) -> Vec<TableEventKey> {
        vec![
            TableEventKey::Text(v.prescription_id.clone()),
            match &v.to {
                PrescriptionAuthSubject::User(to) => TableEventKey::Principal(to.clone()),
                PrescriptionAuthSubject::Group(to) => TableEventKey::Text(to.clone()),
            },
        ]
    }
}

impl TableSubscriber<TableName> for PrescriptionAuthsTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Prescriptions => {
                if let TableEventKey::Text(prescription_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                        },
                        TableEventKind::Update => {
                        },
                        TableEventKind::Delete => {
                            let ids: Vec<_> = match self.aux.as_deref().unwrap().borrow().find_by_id(&prescription_key) {
                                Some(ids) => ids.iter().cloned().collect(),
                                None => return
                            };
                            ids.iter().cloned().for_each(|id| {
                                _ = self.delete_and_notify(&id);
                            })
                        },
                    }
                }
            },
            _ => panic!("Unsupported")
        }
    }
}