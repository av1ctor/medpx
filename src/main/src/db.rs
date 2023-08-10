use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::models::doctor::{Doctor, DoctorId};
use crate::models::patient::{Patient, PatientId};
use crate::models::prescription::{Prescription, PrescriptionId};
use crate::models::prescription_template::PrescriptionTemplate;
use crate::models::staff::{Staff, StaffId};
use crate::models::key::{Key, KeyId};
use crate::models::prescription_auth::{PrescriptionAuth, PrescriptionAuthId};
use crate::models::thirdparty::{ThirdPartyId, ThirdParty};

#[derive(Default, CandidType, Deserialize)]
pub struct DB {
    // staff tables
    pub staff: BTreeMap<StaffId, Staff>,
    // doctors tables
    pub doctors: BTreeMap<DoctorId, Doctor>,
    pub doctor_prescriptions_rel: BTreeMap<DoctorId, BTreeSet<PrescriptionId>>,
    // patients tables
    pub patients: BTreeMap<PatientId, Patient>,
    pub patient_prescriptions_rel: BTreeMap<PatientId, BTreeSet<PrescriptionId>>,
    // third party tables
    pub thirdparty: BTreeMap<ThirdPartyId, ThirdParty>,
    // prescription authorizations tables
    pub prescrition_auths: BTreeMap<PrescriptionAuthId, PrescriptionAuth>,
    // prescriptions tables
    pub prescriptions: BTreeMap<PrescriptionId, Prescription>,
    pub prescription_auths_rel: BTreeMap<PrescriptionId, BTreeSet<PrescriptionAuthId>>,
    // prescription templates tables
    pub prescription_templates: BTreeMap<String, PrescriptionTemplate>,
    // keys tables
    pub keys: BTreeMap<KeyId, Key>,
    pub principal_keys_rel: BTreeMap<Principal, BTreeSet<KeyId>>,
    pub key_principal: BTreeMap<String, Principal>,
}

impl DB {
    /**
     * staff table
     */
    pub fn staff_insert(
        &mut self,
        k: &StaffId,
        v: &Staff
    ) -> Result<(), String> {
        if self.staff.contains_key(k) {
            Err("Staff member already exists".to_string())
        }
        else {
            self.staff.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn staff_update(
        &mut self,
        k: &StaffId,
        v: &Staff
    ) -> Result<(), String> {
        if !self.staff.contains_key(k) {
            Err("Staff member not found".to_string())
        }
        else {
            self.staff.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn staff_find_by_id(
        &self,
        k: &StaffId 
    ) -> Option<Staff> {
        if !self.staff.contains_key(k) {
            None
        }
        else {
            Some(self.staff[k].clone())
        }
    }

    /**
     * doctors table
     */
    pub fn doctor_insert(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if self.doctors.contains_key(k) {
            Err("Doctor already exists".to_string())
        }
        else {
            self.doctors.insert(k.clone(), v.clone());
            self.doctor_prescriptions_rel.insert(k.clone(), BTreeSet::new());
            Ok(())
        }
    }

    pub fn doctor_update(
        &mut self,
        k: &DoctorId,
        v: &Doctor
    ) -> Result<(), String> {
        if !self.doctors.contains_key(k) {
            Err("Doctor not found".to_string())
        }
        else {
            self.doctors.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn doctor_find_by_id(
        &self,
        k: &DoctorId 
    ) -> Option<Doctor> {
        if !self.doctors.contains_key(k) {
            None
        }
        else {
            Some(self.doctors[k].clone())
        }
    }

    /**
     * patients table
     */
    pub fn patient_insert(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if self.patients.contains_key(k) {
            Err("Patient already exists".to_string())
        }
        else {
            self.patients.insert(k.clone(), v.clone());
            self.patient_prescriptions_rel.insert(k.clone(), BTreeSet::new());
            Ok(())
        }
    }

    pub fn patient_update(
        &mut self,
        k: &PatientId,
        v: &Patient
    ) -> Result<(), String> {
        if !self.patients.contains_key(k) {
            Err("Patient not found".to_string())
        }
        else {
            self.patients.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn patient_find_by_id(
        &self,
        k: &PatientId
    ) -> Option<Patient> {
        if !self.patients.contains_key(k) {
            None
        }
        else {
            Some(self.patients[k].clone())             
        }
    }

    /**
     * third party table
     */
    pub fn thirdparty_insert(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if self.thirdparty.contains_key(k) {
            Err("Third party already exists".to_string())
        }
        else {
            self.thirdparty.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn thirdparty_update(
        &mut self,
        k: &ThirdPartyId,
        v: &ThirdParty
    ) -> Result<(), String> {
        if !self.thirdparty.contains_key(k) {
            Err("Third party not found".to_string())
        }
        else {
            self.thirdparty.insert(k.clone(), v.clone());
            Ok(())
        }
    }

    pub fn thirdparty_find_by_id(
        &self,
        k: &ThirdPartyId 
    ) -> Option<ThirdParty> {
        if !self.thirdparty.contains_key(k) {
            None
        }
        else {
            Some(self.thirdparty[k].clone())
        }
    }

    /**
     * prescriptions table
     */
    pub fn prescription_insert(
        &mut self,
        k: &PrescriptionId,
        v: &Prescription
    ) -> Result<(), String> {
        if self.prescriptions.contains_key(k) {
            return Err("Prescription already exists".to_string());
        }
        
        self.prescriptions.insert(k.clone(), v.clone());
        
        let doc_prescriptions = self.doctor_prescriptions_rel
            .get_mut(&v.doctor).ok_or_else(|| "Doctor not found")?;
        doc_prescriptions.insert(k.clone());

        let pat_prescriptions = self.patient_prescriptions_rel
            .get_mut(&v.patient).ok_or_else(|| "Patient not found")?;
        pat_prescriptions.insert(k.clone());

        Ok(())
    }

    /**
     * keys table
     */
    pub fn key_insert(
        &mut self,
        id: &KeyId,
        k: &Principal,
        v: &Key
    ) -> Result<(), String> {
        if !self.principal_keys_rel.contains_key(k) {
            self.principal_keys_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let keys = self.principal_keys_rel.get_mut(k).unwrap();
        if keys.iter().any(|e| self.keys[e].cmp(v) == Ordering::Equal) {
            Err("Key already exists".to_string())
        }
        else {
            self.keys.insert(id.clone(), v.clone());
            keys.insert(id.clone());
            self.key_principal.insert(id.clone(), k.clone());
            Ok(())
        }
    }

    /**
     * authorizations table
     */
    pub fn prescription_auth_insert(
        &mut self,
        id: &PrescriptionAuthId,
        v: &PrescriptionAuth
    ) -> Result<(), String> {
        let k = &v.prescription_id;
        
        if !self.prescription_auths_rel.contains_key(k) {
            self.prescription_auths_rel.insert(k.clone(), BTreeSet::new());
        }
        
        let auths = self.prescription_auths_rel.get_mut(k).unwrap();
        if auths.iter().any(|e| self.prescrition_auths[e].cmp(v) == Ordering::Equal) {
            Err("Authorization already exists".to_string())
        }
        else {
            self.prescrition_auths.insert(id.clone(), v.clone());
            auths.insert(id.clone());
            Ok(())
        }
    }
    
}
