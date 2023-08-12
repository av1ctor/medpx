pub mod traits;
pub mod tables;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use candid::Principal;
use ic_cdk::api::stable::{StableWriter, StableReader};
use crate::models::doctor::DoctorId;
use crate::models::patient::PatientId;
use crate::models::prescription::{Prescription, PrescriptionId};
use crate::models::key::{Key, KeyId};
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId};
use self::tables::prescription_auth::PrescriptionAuthTable;
use self::tables::prescription_template::PrescriptionTemplateTable;
use self::traits::crud::CRUD;
use self::tables::doctor::DoctorTable;
use self::tables::key::KeyTable;
use self::tables::patient::PatientTable;
use self::tables::prescription::PrescriptionTable;
use self::tables::staff::StaffTable;
use self::tables::thirdparty::ThirdPartyTable;
use self::traits::table::{TableAllocator, TableSerializer, TableDeserializer};

pub struct DB {
    pub staff: StaffTable,
    pub doctors: DoctorTable,
    pub patients: PatientTable,
    pub thirdparties: ThirdPartyTable,
    pub prescriptions: PrescriptionTable,
    pub prescrition_auths: PrescriptionAuthTable,
    pub prescription_templates: PrescriptionTemplateTable,
    pub keys: KeyTable,
    pub doctor_prescriptions_rel: BTreeMap<DoctorId, BTreeSet<PrescriptionId>>,
    pub patient_prescriptions_rel: BTreeMap<PatientId, BTreeSet<PrescriptionId>>,
    pub prescription_auths_rel: BTreeMap<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
    pub principal_keys_rel: BTreeMap<Principal, BTreeSet<KeyId>>,
    pub key_principal: BTreeMap<String, Principal>,
}

impl DB {
    pub fn new(
    ) -> Self {
        let doctors = DoctorTable::new();
        let staff = StaffTable::new();
        let patients = PatientTable::new();
        let thirdparties = ThirdPartyTable::new();
        let prescriptions = PrescriptionTable::new();
        let keys = KeyTable::new();
        let prescrition_auths = PrescriptionAuthTable::new();
        let prescription_templates = PrescriptionTemplateTable::new();
        
        Self {
            doctors,
            staff,
            patients,
            thirdparties,
            prescriptions,
            keys,
            prescrition_auths,
            prescription_templates,
            doctor_prescriptions_rel: todo!(),
            patient_prescriptions_rel: todo!(),
            prescription_auths_rel: todo!(),
            principal_keys_rel: todo!(),
            key_principal: todo!(),
        }
    }

    pub fn serialize(
        &self,
        writter: &mut StableWriter
    ) -> Result<(), String> {
        DoctorTable::serialize(&self.doctors, writter)?;
        Ok(())
    }

    pub fn deserialize(
        &mut self,
        reader: &mut StableReader
    ) -> Result<(), String> {
        self.doctors.data = DoctorTable::deserialize(reader)?;
        Ok(())
    }
    
    /**
     * prescriptions table
     */
    pub fn prescription_insert(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        self.prescriptions.insert(k, v)?;
        
        if !self.doctor_prescriptions_rel.contains_key(&v.doctor) {
            self.doctor_prescriptions_rel.insert(v.doctor.clone(), BTreeSet::new());
        }
        
        let doc_prescriptions = self.doctor_prescriptions_rel
            .get_mut(&v.doctor).unwrap();
        doc_prescriptions.insert(k.clone());

        if !self.patient_prescriptions_rel.contains_key(&v.patient) {
            self.patient_prescriptions_rel.insert(v.patient.clone(), BTreeSet::new());
        }

        let pat_prescriptions = self.patient_prescriptions_rel
            .get_mut(&v.patient).unwrap();
        pat_prescriptions.insert(k.clone());

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
        if !self.principal_keys_rel.contains_key(k) {
            self.principal_keys_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let keys = self.principal_keys_rel.get_mut(k).unwrap();
        if keys.iter().any(|e| self.keys.get(e).cmp(v) == Ordering::Equal) {
            Err("Key already exists".to_string())
        }
        else {
            self.keys.insert(&v.id, v)?;
            keys.insert(v.id.clone());
            self.key_principal.insert(v.id.clone(), k.clone());
            Ok(())
        }
    }

    /**
     * authorizations table
     */
    pub fn prescription_auth_insert(
        &mut self,
        k: &PrescriptionAuthId,
        v: &PrescriptionAuth
    ) -> Result<(), String> {
        if !self.prescription_auths_rel.contains_key(k) {
            self.prescription_auths_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let auths = self.prescription_auths_rel.get_mut(k).unwrap();
        if auths.iter().any(|e| self.prescrition_auths.get(e).cmp(v) == Ordering::Equal) {
            Err("Authorization already exists".to_string())
        }
        else {
            self.prescrition_auths.insert(k, v)?;
            auths.insert(k.clone());
            Ok(())
        }
    }
    
}
