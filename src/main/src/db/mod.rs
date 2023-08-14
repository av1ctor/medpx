pub mod traits;
pub mod tables;

use std::cell::RefCell;
use std::rc::Rc;
use ic_cdk::api::stable::{StableWriter, StableReader};
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId};
use self::tables::doctor_prescriptions_rel::DoctorPrescriptionsRelTable;
use self::tables::key_principal_rel::KeyPrincipalRelTable;
use self::tables::patient_prescriptions_rel::PatientPrescriptionsRelTable;
use self::tables::prescription_auths::PrescriptionAuthsTable;
use self::tables::prescription_templates::PrescriptionTemplatesTable;
use self::tables::doctors::DoctorsTable;
use self::tables::keys::KeysTable;
use self::tables::patients::PatientsTable;
use self::tables::prescriptions::PrescriptionsTable;
use self::tables::principal_keys_rel::PrincipalKeysRelTable;
use self::tables::staff::StaffTable;
use self::tables::thirdparties::ThirdPartiesTable;
use self::traits::table::{TableSerializable, TableDeserializable, TableSubscribable};

pub struct DB {
    pub doctors: Rc<RefCell<DoctorsTable>>,
    pub patients: Rc<RefCell<PatientsTable>>,
    pub staff: Rc<RefCell<StaffTable>>,
    pub thirdparties: Rc<RefCell<ThirdPartiesTable>>,
    pub prescriptions: Rc<RefCell<PrescriptionsTable>>,
    pub prescription_auths: Rc<RefCell<PrescriptionAuthsTable>>,
    pub prescription_templates: Rc<RefCell<PrescriptionTemplatesTable>>,
    pub keys: Rc<RefCell<KeysTable>>,
    pub doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionsRelTable>>,
    pub patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionsRelTable>>,
    //pub prescription_auths_rel: BTreeMap<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
    pub principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
    pub key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
}

impl DB {
    pub fn new(
        doctors: Rc<RefCell<DoctorsTable>>,
        patients: Rc<RefCell<PatientsTable>>,
        staff: Rc<RefCell<StaffTable>>,
        thirdparties: Rc<RefCell<ThirdPartiesTable>>,
        prescriptions: Rc<RefCell<PrescriptionsTable>>,
        keys: Rc<RefCell<KeysTable>>,
        prescrition_auths: Rc<RefCell<PrescriptionAuthsTable>>,
        prescription_templates: Rc<RefCell<PrescriptionTemplatesTable>>,
        doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionsRelTable>>,
        patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionsRelTable>>,
        principal_keys_rel: Rc<RefCell<PrincipalKeysRelTable>>,
        key_principal_rel: Rc<RefCell<KeyPrincipalRelTable>>,
    ) -> Self {

        prescriptions.borrow_mut().subscribe(doctor_prescriptions_rel.clone());
        prescriptions.borrow_mut().subscribe(patient_prescriptions_rel.clone());
        keys.borrow_mut().subscribe(principal_keys_rel.clone());
        keys.borrow_mut().subscribe(key_principal_rel.clone());
        
        Self {
            doctors,
            patients,
            staff,
            thirdparties,
            prescriptions,
            keys,
            prescription_auths: prescrition_auths,
            prescription_templates,
            doctor_prescriptions_rel,
            patient_prescriptions_rel,
            //prescription_auths_rel: todo!(),
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
        self.keys.borrow().serialize(writer)?;
        self.prescriptions.borrow().serialize(writer)?;
        self.prescription_auths.borrow().serialize(writer)?;
        self.prescription_templates.borrow().serialize(writer)?;
        self.doctor_prescriptions_rel.borrow().serialize(writer)?;
        self.patient_prescriptions_rel.borrow().serialize(writer)?;
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
        self.keys.borrow_mut().deserialize(reader)?;
        self.prescriptions.borrow_mut().deserialize(reader)?;
        self.prescription_auths.borrow_mut().deserialize(reader)?;
        self.prescription_templates.borrow_mut().deserialize(reader)?;
        self.doctor_prescriptions_rel.borrow_mut().deserialize(reader)?;
        self.patient_prescriptions_rel.borrow_mut().deserialize(reader)?;
        self.principal_keys_rel.borrow_mut().deserialize(reader)?;
        self.key_principal_rel.borrow_mut().deserialize(reader)?;
        Ok(())
    }
    
    /**
     * authorizations table
     */
    pub fn prescription_auth_insert(
        &mut self,
        k: &PrescriptionAuthId,
        v: &PrescriptionAuth
    ) -> Result<(), String> {
        /*if !self.prescription_auths_rel.contains_key(k) {
            self.prescription_auths_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let auths = self.prescription_auths_rel.get_mut(k).unwrap();
        if auths.iter().any(|e| self.prescrition_auths.get(e).cmp(v) == Ordering::Equal) {
            Err("Authorization already exists".to_string())
        }
        else {
            self.prescrition_auths.insert(k, v)?;
            auths.insert(k.clone());*/
            Ok(())
        //}
    }
    
}
