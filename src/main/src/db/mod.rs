pub mod traits;
pub mod tables;
pub mod migrations;

use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::api::stable::{StableWriter, StableReader};
use self::tables::user_prescriptions_rel::UserPrescriptionsRelTable;
use self::tables::groups::GroupsTable;
use self::tables::key_principal_rel::KeyPrincipalRelTable;
use self::tables::prescription_auths::PrescriptionAuthsTable;
use self::tables::prescription_auths_rel::PrescriptionAuthsRelTable;
use self::tables::prescription_templates::PrescriptionTemplatesTable;
use self::tables::keys::KeysTable;
use self::tables::prescriptions::PrescriptionsTable;
use self::tables::principal_groups_rel::PrincipalGroupsRelTable;
use self::tables::principal_keys_rel::PrincipalKeysRelTable;
use self::tables::users::UsersTable;
use self::traits::table::{TableSerializable, TableDeserializable, TableSubscribable, Table};

#[derive(Clone, Debug)]
pub enum TableName {
    Users,
    Prescriptions,
    Keys,
    PrescriptionAuths,
    PrescriptionTemplates,
    UserPrescriptionsRel,
    PrescriptionAuthsRel,
    PrincipalKeysRel,
    KeyPrincipalRel,
    Groups,
    PrincipalGroupsRel,
}

pub struct DB {
    pub users: Rc<RefCell<UsersTable>>,
    pub prescriptions: Rc<RefCell<PrescriptionsTable>>,
    pub prescription_auths: Rc<RefCell<PrescriptionAuthsTable>>,
    pub prescription_templates: Rc<RefCell<PrescriptionTemplatesTable>>,
    pub keys: Rc<RefCell<KeysTable>>,
    pub user_prescriptions_rel: Rc<RefCell<UserPrescriptionsRelTable>>,
    pub prescription_auths_rel: Rc<RefCell<PrescriptionAuthsRelTable>>,
    pub principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
    pub key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
    pub groups: Rc<RefCell<GroupsTable>>,
    pub principal_groups_rel: Rc<RefCell<PrincipalGroupsRelTable>>,
}

impl DB {
    pub fn new(
    ) -> Self {
        let users = Rc::new(RefCell::new(UsersTable::new()));
        let prescriptions = Rc::new(RefCell::new(PrescriptionsTable::new()));
        let keys = Rc::new(RefCell::new(KeysTable::new()));
        let prescription_auths = Rc::new(RefCell::new(PrescriptionAuthsTable::new()));
        let prescription_templates = Rc::new(RefCell::new(PrescriptionTemplatesTable::new()));
        let user_prescriptions_rel = Rc::new(RefCell::new(UserPrescriptionsRelTable::new()));
        let prescription_auths_rel = Rc::new(RefCell::new(PrescriptionAuthsRelTable::new()));
        let principal_keys_rel = Rc::new(RefCell::new(PrincipalKeysRelTable::new()));
        let key_principal_rel = Rc::new(RefCell::new(KeyPrincipalRelTable::new()));
        let groups = Rc::new(RefCell::new(GroupsTable::new()));
        let principal_groups_rel = Rc::new(RefCell::new(PrincipalGroupsRelTable::new()));

        //
        prescriptions.borrow_mut().subscribe(user_prescriptions_rel.clone());
        prescriptions.borrow_mut().subscribe(prescription_auths.clone());
        //
        keys.borrow_mut().subscribe(principal_keys_rel.clone());
        keys.borrow_mut().subscribe(key_principal_rel.clone());
        //
        prescription_auths.borrow_mut().set_aux(prescription_auths_rel.clone());
        prescription_auths.borrow_mut().subscribe(prescription_auths_rel.clone());
        prescription_auths.borrow_mut().subscribe(user_prescriptions_rel.clone());
        //
        groups.borrow_mut().subscribe(principal_groups_rel.clone());
        
        Self {
            users,
            prescriptions,
            keys,
            prescription_auths,
            prescription_templates,
            user_prescriptions_rel,
            prescription_auths_rel,
            principal_keys_rel,
            key_principal_rel,
            groups,
            principal_groups_rel,
        }
    }

    pub fn serialize(
        &self,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        self.users.borrow().serialize(writer)?;
        self.keys.borrow().serialize(writer)?;
        self.prescriptions.borrow().serialize(writer)?;
        self.prescription_auths.borrow().serialize(writer)?;
        self.prescription_templates.borrow().serialize(writer)?;
        self.user_prescriptions_rel.borrow().serialize(writer)?;
        self.prescription_auths_rel.borrow().serialize(writer)?;
        self.principal_keys_rel.borrow().serialize(writer)?;
        self.key_principal_rel.borrow().serialize(writer)?;
        self.groups.borrow().serialize(writer)?;
        self.principal_groups_rel.borrow().serialize(writer)?;
        Ok(())
    }

    pub fn deserialize(
        &mut self,
        reader: &mut StableReader
    ) -> Result<(), String> {
        self.users.borrow_mut().deserialize(reader, true)?;
        self.keys.borrow_mut().deserialize(reader, true)?;
        self.prescriptions.borrow_mut().deserialize(reader, true)?;
        self.prescription_auths.borrow_mut().deserialize(reader, true)?;
        self.prescription_templates.borrow_mut().deserialize(reader, true)?;
        self.user_prescriptions_rel.borrow_mut().deserialize(reader, true)?;
        self.prescription_auths_rel.borrow_mut().deserialize(reader, true)?;
        self.principal_keys_rel.borrow_mut().deserialize(reader, true)?;
        self.key_principal_rel.borrow_mut().deserialize(reader, true)?;
        self.groups.borrow_mut().deserialize(reader, true)?;
        self.principal_groups_rel.borrow_mut().deserialize(reader, true)?;
        Ok(())
    }
}
