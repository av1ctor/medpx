use std::collections::{BTreeSet, BTreeMap};
use candid::Principal;
use crate::db::TableName;
use crate::sdb::table::{TableSerializable, TableDeserializable, TableEventKind, TableEventKey, TableSubscriber, TableData, Table, TableSchema, TableVersioned, TableEvent};
use crate::sdb::crud::Crud;
use crate::models::group::GroupId;

pub struct PrincipalGroupsRelTable {
    pub schema: TableSchema<TableName>,
    pub data: TableData<Principal, BTreeSet<GroupId>>,
}
    
impl Table<TableName, Principal, BTreeSet<GroupId>> for PrincipalGroupsRelTable {
    fn new(
    ) -> Self {
        Self {
            schema: TableSchema { 
                version: 0.1,
                name: TableName::PrincipalGroupsRel,
            },
            data: TableData(BTreeMap::new()),
        }
    }

    fn get_data(
        &self
    ) -> &TableData<Principal, BTreeSet<GroupId>> {
        &self.data
    }

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<Principal, BTreeSet<GroupId>> {
        &mut self.data
    }

    fn set_data(
        &mut self,
        data: TableData<Principal, BTreeSet<GroupId>>
    ) {
        self.data = data;
    }
    
    fn get_schema(
        &self
    ) -> &TableSchema<TableName> {
        &self.schema
    }
}

impl TableSerializable<TableName, Principal, BTreeSet<GroupId>> for PrincipalGroupsRelTable {}

impl TableVersioned<TableName, Principal, BTreeSet<GroupId>> for PrincipalGroupsRelTable {}

impl TableDeserializable<TableName, Principal, BTreeSet<GroupId>> for PrincipalGroupsRelTable {}

impl Crud<TableName, Principal, BTreeSet<GroupId>> for PrincipalGroupsRelTable {}

impl TableSubscriber<TableName> for PrincipalGroupsRelTable {
    fn on(
        &mut self,
        event: &TableEvent<TableName>
    ) {
        match event.table_name {
            TableName::Groups => {
                if let TableEventKey::Text(group_key) = event.pkey.clone() {
                    match event.kind {
                        TableEventKind::Create => {
                            event.keys.iter().for_each(|key| {
                                if let TableEventKey::Principal(principal) = key {
                                    if !self.data.0.contains_key(&principal) {
                                        self.data.0.insert(principal.clone(), BTreeSet::new());
                                    }
        
                                    self.data.0.get_mut(&principal).unwrap()
                                        .insert(group_key.clone());
                                }
                            });
                        },
                        TableEventKind::Update => {
                            event.keys.iter().for_each(|key| {
                                if let TableEventKey::Principal(principal) = key {
                                    self.data.0.clear();
        
                                    self.data.0.get_mut(&principal).unwrap()
                                        .insert(group_key.clone());
                                }
                            });
                        },
                        TableEventKind::Delete => {
                            event.keys.iter().for_each(|key| {
                                if let TableEventKey::Principal(principal) = key {
                                    self.data.0.get_mut(&principal).unwrap()
                                        .remove(&group_key);
                                }
                            });
                        },
                    }
                }
            },
            _ => panic!("Unsupported")
        }
    }
}