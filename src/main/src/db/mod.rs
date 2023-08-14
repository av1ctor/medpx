pub mod traits;
pub mod tables;

use std::cell::RefCell;
use std::rc::Rc;
use candid::Principal;
use ic_cdk::api::stable::{StableWriter, StableReader};
use crate::models::key::Key;
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId};
use self::tables::doctor_prescription::DoctorPrescriptionTable;
use self::tables::patient_prescription::PatientPrescriptionTable;
use self::tables::prescription_auth::PrescriptionAuthTable;
use self::tables::prescription_template::PrescriptionTemplateTable;
use self::tables::doctor::DoctorTable;
use self::tables::key::KeyTable;
use self::tables::patient::PatientTable;
use self::tables::prescription::PrescriptionTable;
use self::tables::staff::StaffTable;
use self::tables::thirdparty::ThirdPartyTable;
use self::traits::table::{TableSerializable, TableDeserializable, TableSubscribable};

pub struct DB {
    pub doctors: Rc<RefCell<DoctorTable>>,
    pub patients: Rc<RefCell<PatientTable>>,
    pub staff: Rc<RefCell<StaffTable>>,
    pub thirdparties: Rc<RefCell<ThirdPartyTable>>,
    pub prescriptions: Rc<RefCell<PrescriptionTable>>,
    pub prescription_auths: Rc<RefCell<PrescriptionAuthTable>>,
    pub prescription_templates: Rc<RefCell<PrescriptionTemplateTable>>,
    pub keys: Rc<RefCell<KeyTable>>,
    pub doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionTable>>,
    pub patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionTable>>,
    //pub prescription_auths_rel: BTreeMap<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
    //pub principal_keys_rel: BTreeMap<Principal, BTreeSet<KeyId>>,
    //pub key_principal: BTreeMap<String, Principal>,
}

impl DB {
    pub fn new(
        doctors: Rc<RefCell<DoctorTable>>,
        patients: Rc<RefCell<PatientTable>>,
        staff: Rc<RefCell<StaffTable>>,
        thirdparties: Rc<RefCell<ThirdPartyTable>>,
        prescriptions: Rc<RefCell<PrescriptionTable>>,
        keys: Rc<RefCell<KeyTable>>,
        prescrition_auths: Rc<RefCell<PrescriptionAuthTable>>,
        prescription_templates: Rc<RefCell<PrescriptionTemplateTable>>,
        doctor_prescriptions_rel: Rc<RefCell<DoctorPrescriptionTable>>,
        patient_prescriptions_rel: Rc<RefCell<PatientPrescriptionTable>>,
    ) -> Self {

        prescriptions.borrow_mut().subscribe(doctor_prescriptions_rel.clone());
        prescriptions.borrow_mut().subscribe(patient_prescriptions_rel.clone());
        
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
            //principal_keys_rel: todo!(),
            //key_principal: todo!(),
        }
    }

    pub fn serialize(
        &self,
        writter: &mut StableWriter
    ) -> Result<(), String> {
        DoctorTable::serialize(&self.doctors.borrow().data, writter)?;
        PatientTable::serialize(&self.patients.borrow().data, writter)?;
        StaffTable::serialize(&self.staff.borrow().data, writter)?;
        ThirdPartyTable::serialize(&self.thirdparties.borrow().data, writter)?;
        KeyTable::serialize(&self.keys.borrow().data, writter)?;
        PrescriptionTable::serialize(&self.prescriptions.borrow().data, writter)?;
        PrescriptionAuthTable::serialize(&self.prescription_auths.borrow().data, writter)?;
        PrescriptionTemplateTable::serialize(&self.prescription_templates.borrow().data, writter)?;
        DoctorPrescriptionTable::serialize(&self.doctor_prescriptions_rel.borrow().data, writter)?;
        PatientPrescriptionTable::serialize(&self.patient_prescriptions_rel.borrow().data, writter)?;
        Ok(())
    }

    pub fn deserialize(
        &mut self,
        reader: &mut StableReader
    ) -> Result<(), String> {
        self.doctors.borrow_mut().data = DoctorTable::deserialize(reader)?;
        self.patients.borrow_mut().data = PatientTable::deserialize(reader)?;
        self.staff.borrow_mut().data = StaffTable::deserialize(reader)?;
        self.thirdparties.borrow_mut().data = ThirdPartyTable::deserialize(reader)?;
        self.keys.borrow_mut().data = KeyTable::deserialize(reader)?;
        self.prescriptions.borrow_mut().data = PrescriptionTable::deserialize(reader)?;
        self.prescription_auths.borrow_mut().data = PrescriptionAuthTable::deserialize(reader)?;
        self.prescription_templates.borrow_mut().data = PrescriptionTemplateTable::deserialize(reader)?;
        self.doctor_prescriptions_rel.borrow_mut().data = DoctorPrescriptionTable::deserialize(reader)?;
        self.patient_prescriptions_rel.borrow_mut().data = PatientPrescriptionTable::deserialize(reader)?;
        Ok(())
    }
    
    /**
     * keys table
     */
    pub fn key_insert(
        &mut self,
        k: &Principal,
        v: &Key
    ) -> Result<(), String> {
        /*if !self.principal_keys_rel.contains_key(k) {
            self.principal_keys_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let keys = self.principal_keys_rel.get_mut(k).unwrap();
        if keys.iter().any(|e| self.keys.get(e).cmp(v) == Ordering::Equal) {
            Err("Key already exists".to_string())
        }
        else {
            self.keys.insert(&v.id, v)?;
            keys.insert(v.id.clone());
            self.key_principal.insert(v.id.clone(), k.clone());*/
            Ok(())
        //}
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
