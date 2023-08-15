pub mod traits;
pub mod tables;
pub mod migrations;

use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::api::stable::{StableWriter, StableReader};
use self::tables::doctor_prescriptions_rel::DoctorPrescriptionsRelTable;
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
use self::tables::users::UsersTable;
use self::traits::table::{TableSerializable, TableDeserializable, TableSubscribable};

#[derive(Clone)]
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
    PrescriptionAuthsRel,
    PrincipalKeysRel,
    KeyPrincipalRel,
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
    pub prescription_auths_rel: Rc<RefCell<PrescriptionAuthsRelTable>>,
    pub principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
    pub key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
}

impl DB {
    pub fn new(
        doctors: Rc<RefCell<DoctorsTable>>,
        patients: Rc<RefCell<PatientsTable>>,
        staff: Rc<RefCell<StaffTable>>,
        thirdparties: Rc<RefCell<ThirdPartiesTable>>,
        users: Rc<RefCell<UsersTable>>,
        prescriptions: Rc<RefCell<PrescriptionsTable>>,
        keys: Rc<RefCell<KeysTable>>,
        prescription_auths: Rc<RefCell<PrescriptionAuthsTable>>,
        prescription_templates: Rc<RefCell<PrescriptionTemplatesTable>>,
        doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionsRelTable>>,
        patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionsRelTable>>,
        prescription_auths_rel: Rc<RefCell<PrescriptionAuthsRelTable>>,
        principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
        key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
    ) -> Self {

        //
        prescriptions.borrow_mut().subscribe(doctor_prescriptions_rel.clone());
        prescriptions.borrow_mut().subscribe(patient_prescriptions_rel.clone());
        //
        keys.borrow_mut().subscribe(principal_keys_rel.clone());
        keys.borrow_mut().subscribe(key_principal_rel.clone());
        //
        prescription_auths.borrow_mut().subscribe(prescription_auths_rel.clone());
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
            prescription_auths_rel,
            principal_keys_rel,
            key_principal_rel,
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
        self.prescription_auths_rel.borrow().serialize(writer)?;
        self.principal_keys_rel.borrow().serialize(writer)?;
        self.key_principal_rel.borrow().serialize(writer)?;
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
        self.prescription_auths_rel.borrow_mut().deserialize(reader)?;
        self.principal_keys_rel.borrow_mut().deserialize(reader)?;
        self.key_principal_rel.borrow_mut().deserialize(reader)?;
        Ok(())
    }
}
