pub mod traits;
pub mod tables;
pub mod migrations;

use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::api::stable::{StableWriter, StableReader};
use self::tables::doctor_prescriptions_rel::DoctorPrescriptionsRelTable;
use self::tables::groups::GroupsTable;
use self::tables::key_principal_rel::KeyPrincipalRelTable;
use self::tables::patient_prescriptions_rel::PatientPrescriptionsRelTable;
use self::tables::prescription_auths::PrescriptionAuthsTable;
use self::tables::prescription_auths_rel::PrescriptionAuthsRelTable;
use self::tables::prescription_templates::PrescriptionTemplatesTable;
use self::tables::doctors::DoctorsTable;
use self::tables::keys::KeysTable;
use self::tables::patients::PatientsTable;
use self::tables::prescriptions::PrescriptionsTable;
use self::tables::principal_keys_rel::PrincipalKeysRelTable;
use self::tables::staff::StaffTable;
use self::tables::thirdparties::ThirdPartiesTable;
use self::tables::thirdparty_prescriptions_rel::ThirdPartyPrescriptionsRelTable;
use self::tables::users::UsersTable;
use self::traits::table::{TableSerializable, TableDeserializable, TableSubscribable, Table};

#[derive(Clone, Debug)]
pub enum TableName {
    Doctors,
    Patients,
    Staff,
    ThirdParties,
    Users,
    Prescriptions,
    Keys,
    PrescriptionAuths,
    PrescriptionTemplates,
    DoctorPrescriptionsRel,
    PatientPrescriptionsRel,
    ThirdPartyPrescriptionsRel,
    PrescriptionAuthsRel,
    PrincipalKeysRel,
    KeyPrincipalRel,
    Groups,
}

pub struct DB {
    pub doctors: Rc<RefCell<DoctorsTable>>,
    pub patients: Rc<RefCell<PatientsTable>>,
    pub staff: Rc<RefCell<StaffTable>>,
    pub thirdparties: Rc<RefCell<ThirdPartiesTable>>,
    pub users: Rc<RefCell<UsersTable>>,
    pub prescriptions: Rc<RefCell<PrescriptionsTable>>,
    pub prescription_auths: Rc<RefCell<PrescriptionAuthsTable>>,
    pub prescription_templates: Rc<RefCell<PrescriptionTemplatesTable>>,
    pub keys: Rc<RefCell<KeysTable>>,
    pub doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionsRelTable>>,
    pub patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionsRelTable>>,
    pub thirdparty_prescriptions_rel: Rc<RefCell<ThirdPartyPrescriptionsRelTable>>,
    pub prescription_auths_rel: Rc<RefCell<PrescriptionAuthsRelTable>>,
    pub principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
    pub key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
    pub groups: Rc<RefCell<GroupsTable>>,
}

impl DB {
    pub fn new(
    ) -> Self {
        let doctors = Rc::new(RefCell::new(DoctorsTable::new()));
        let patients = Rc::new(RefCell::new(PatientsTable::new()));
        let staff = Rc::new(RefCell::new(StaffTable::new()));
        let thirdparties = Rc::new(RefCell::new(ThirdPartiesTable::new()));
        let users = Rc::new(RefCell::new(UsersTable::new()));
        let prescriptions = Rc::new(RefCell::new(PrescriptionsTable::new()));
        let keys = Rc::new(RefCell::new(KeysTable::new()));
        let prescription_auths = Rc::new(RefCell::new(PrescriptionAuthsTable::new()));
        let prescription_templates = Rc::new(RefCell::new(PrescriptionTemplatesTable::new()));
        let doctor_prescriptions_rel = Rc::new(RefCell::new(DoctorPrescriptionsRelTable::new()));
        let patient_prescriptions_rel = Rc::new(RefCell::new(PatientPrescriptionsRelTable::new()));
        let thirdparty_prescriptions_rel = Rc::new(RefCell::new(ThirdPartyPrescriptionsRelTable::new()));
        let prescription_auths_rel = Rc::new(RefCell::new(PrescriptionAuthsRelTable::new()));
        let principal_keys_rel = Rc::new(RefCell::new(PrincipalKeysRelTable::new()));
        let key_principal_rel = Rc::new(RefCell::new(KeyPrincipalRelTable::new()));
        let groups = Rc::new(RefCell::new(GroupsTable::new()));

        //
        prescriptions.borrow_mut().subscribe(doctor_prescriptions_rel.clone());
        prescriptions.borrow_mut().subscribe(patient_prescriptions_rel.clone());
        //
        keys.borrow_mut().subscribe(principal_keys_rel.clone());
        keys.borrow_mut().subscribe(key_principal_rel.clone());
        //
        prescription_auths.borrow_mut().subscribe(prescription_auths_rel.clone());
        prescription_auths.borrow_mut().subscribe(thirdparty_prescriptions_rel.clone());
        //
        doctors.borrow_mut().subscribe(users.clone());
        patients.borrow_mut().subscribe(users.clone());
        staff.borrow_mut().subscribe(users.clone());
        thirdparties.borrow_mut().subscribe(users.clone());
        
        Self {
            doctors,
            patients,
            staff,
            thirdparties,
            users,
            prescriptions,
            keys,
            prescription_auths,
            prescription_templates,
            doctor_prescriptions_rel,
            patient_prescriptions_rel,
            thirdparty_prescriptions_rel,
            prescription_auths_rel,
            principal_keys_rel,
            key_principal_rel,
            groups,
        }
    }

    pub fn serialize(
        &self,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        self.doctors.borrow().serialize(writer)?;
        self.patients.borrow().serialize(writer)?;
        self.staff.borrow().serialize(writer)?;
        self.thirdparties.borrow().serialize(writer)?;
        self.users.borrow().serialize(writer)?;
        self.keys.borrow().serialize(writer)?;
        self.prescriptions.borrow().serialize(writer)?;
        self.prescription_auths.borrow().serialize(writer)?;
        self.prescription_templates.borrow().serialize(writer)?;
        self.doctor_prescriptions_rel.borrow().serialize(writer)?;
        self.patient_prescriptions_rel.borrow().serialize(writer)?;
        self.thirdparty_prescriptions_rel.borrow().serialize(writer)?;
        self.prescription_auths_rel.borrow().serialize(writer)?;
        self.principal_keys_rel.borrow().serialize(writer)?;
        self.key_principal_rel.borrow().serialize(writer)?;
        self.groups.borrow().serialize(writer)?;
        Ok(())
    }

    pub fn deserialize(
        &mut self,
        reader: &mut StableReader
    ) -> Result<(), String> {
        self.doctors.borrow_mut().deserialize(reader)?;
        self.patients.borrow_mut().deserialize(reader)?;
        self.staff.borrow_mut().deserialize(reader)?;
        self.thirdparties.borrow_mut().deserialize(reader)?;
        self.users.borrow_mut().deserialize(reader)?;
        self.keys.borrow_mut().deserialize(reader)?;
        self.prescriptions.borrow_mut().deserialize(reader)?;
        self.prescription_auths.borrow_mut().deserialize(reader)?;
        self.prescription_templates.borrow_mut().deserialize(reader)?;
        self.doctor_prescriptions_rel.borrow_mut().deserialize(reader)?;
        self.patient_prescriptions_rel.borrow_mut().deserialize(reader)?;
        self.thirdparty_prescriptions_rel.borrow_mut().deserialize(reader)?;
        self.prescription_auths_rel.borrow_mut().deserialize(reader)?;
        self.principal_keys_rel.borrow_mut().deserialize(reader)?;
        self.key_principal_rel.borrow_mut().deserialize(reader)?;
        self.groups.borrow_mut().deserialize(reader)?;
        Ok(())
    }
}
