use std::collections::BTreeMap;
use crate::db::TableName;
use crate::db::traits::table::{TableSerializable, TableDeserializable, TableData, Table, TableSchema, TableVersioned, TableSubscriber, TableEventKind, TableEventKey, TableEvent};
use crate::db::traits::crud::Crud;
use crate::models::user::{UserId, User, UserKind};

pub struct UsersTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<UserId, User>,
}

impl Table<TableName, UserId, User> for UsersTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::Users,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<UserId, User> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<UserId, User> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<UserId, User>
    ) {
        self.data = data;
    }

    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, UserId, User> for UsersTable {}

impl TableVersioned<TableName, UserId, User> for UsersTable {}

impl TableDeserializable<TableName, UserId, User> for UsersTable {}

impl Crud<TableName, UserId, User> for UsersTable {}

impl TableSubscriber<TableName> for UsersTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Doctors => {
                if let TableEventKey::Principal(doctor_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                            self.data.0.insert(
                                doctor_key, 
                                User { 
                                    kind: UserKind::Doctor(doctor_key), 
                                    active: true, 
                                    banned: false,
                                });
                        },
                        TableEventKind::Delete => {
                            self.data.0.remove(&doctor_key);
                        },
                        _ => {}
                    }
                }
            },
            TableName::Patients => {
                if let TableEventKey::Principal(patient_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                            self.data.0.insert(
                                patient_key, 
                                User { 
                                    kind: UserKind::Patient(patient_key), 
                                    active: true, 
                                    banned: false,
                                });
                        },
                        TableEventKind::Delete => {
                            self.data.0.remove(&patient_key);
                        },
                        _ => {}
                    }
                }
            },
            TableName::Staff => {
                if let TableEventKey::Principal(staff_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                            self.data.0.insert(
                                staff_key, 
                                User { 
                                    kind: UserKind::Staff(staff_key), 
                                    active: true, 
                                    banned: false,
                                });
                        },
                        TableEventKind::Delete => {
                            self.data.0.remove(&staff_key);
                        },
                        _ => {}
                    }
                }
            },
            TableName::ThirdParties => {
                if let TableEventKey::Principal(thirdparty_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                            self.data.0.insert(
                                thirdparty_key, 
                                User { 
                                    kind: UserKind::ThirdParty(thirdparty_key), 
                                    active: true, 
                                    banned: false,
                                });
                        },
                        TableEventKind::Delete => {
                            self.data.0.remove(&thirdparty_key);
                        },
                        _ => {}
                    }
                }
            },
            _ => panic!("Unsupported")
        }
    }
}