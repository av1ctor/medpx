pub mod crud;
pub mod tables;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::models::doctor::DoctorId;
use crate::models::patient::PatientId;
use crate::models::prescription::{Prescription, PrescriptionId};
use crate::models::prescription_template::PrescriptionTemplate;
use crate::models::key::{Key, KeyId};
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId};
use self::crud::CRUD;
use self::tables::doctor::DoctorTable;
use self::tables::key::KeyTable;
use self::tables::patient::PatientTable;
use self::tables::prescription::PrescriptionTable;
use self::tables::staff::StaffTable;
use self::tables::thirdparty::ThirdPartyTable;

#[derive(Default, CandidType, Deserialize)]
pub struct DB {
    // staff tables
    pub staff: StaffTable,
    // doctors tables
    pub doctors: DoctorTable,
    pub doctor_prescriptions_rel: BTreeMap<DoctorId, BTreeSet<PrescriptionId>>,
    // patients tables
    pub patients: PatientTable,
    pub patient_prescriptions_rel: BTreeMap<PatientId, BTreeSet<PrescriptionId>>,
    // third party tables
    pub thirdparties: ThirdPartyTable,
    // prescription authorizations tables
    pub prescrition_auths: BTreeMap<PrescriptionAuthId, PrescriptionAuth>,
    // prescriptions tables
    pub prescriptions: PrescriptionTable,
    pub prescription_auths_rel: BTreeMap<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
    // prescription templates tables
    pub prescription_templates: BTreeMap<String, PrescriptionTemplate>,
    // keys tables
    pub keys: KeyTable,
    pub principal_keys_rel: BTreeMap<Principal, BTreeSet<KeyId>>,
    pub key_principal: BTreeMap<String, Principal>,
}

impl DB {
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
        if auths.iter().any(|e| self.prescrition_auths[e].cmp(v) == Ordering::Equal) {
            Err("Authorization already exists".to_string())
        }
        else {
            self.prescrition_auths.insert(k.clone(), v.clone());
            auths.insert(k.clone());
            Ok(())
        }
    }
    
}
