use std::collections::{BTreeMap, BTreeSet};
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::doctor::{Doctor, DoctorId};
use crate::patient::{Patient, PatientId};
use crate::prescription::{Prescription, PrescriptionId};
use crate::prescription_template::PrescriptionTemplate;
use crate::staff::{Staff, StaffId};
use crate::key::{Key, KeyId};
use crate::authorization::{Authorization, AuthorizationId};

#[derive(Default, CandidType, Deserialize)]
pub struct DB {
    // staff tables
    pub staff: BTreeMap<StaffId, Staff>,
    // doctors tables
    pub doctors: BTreeMap<DoctorId, Doctor>,
    pub doctor_prescriptions: BTreeMap<DoctorId, BTreeSet<PrescriptionId>>,
    // patients tables
    pub patients: BTreeMap<PatientId, Patient>,
    pub patient_prescriptions: BTreeMap<PatientId, BTreeSet<PrescriptionId>>,
    // authorizations tables
    pub authorizations: BTreeMap<AuthorizationId, Authorization>,
    // prescriptions tables
    pub prescriptions: BTreeMap<PrescriptionId, Prescription>,
    pub prescription_authorizations: BTreeMap<PrescriptionId, BTreeSet<AuthorizationId>>,
    // prescription templates tables
    pub prescription_templates: BTreeMap<String, PrescriptionTemplate>,
    // keys tables
    pub keys: BTreeMap<KeyId, Key>,
    pub principal_keys: BTreeMap<Principal, BTreeSet<KeyId>>,
    pub key_principal: BTreeMap<KeyId, Principal>,
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
            self.doctor_prescriptions.insert(k.clone(), BTreeSet::new());
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
            self.patient_prescriptions.insert(k.clone(), BTreeSet::new());
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
        
        let doc_prescriptions = self.doctor_prescriptions.get_mut(&v.doctor).ok_or_else(|| "Doctor not found")?;
        doc_prescriptions.insert(k.clone());

        let pat_prescriptions = self.patient_prescriptions.get_mut(&v.patient).ok_or_else(|| "Patient not found")?;
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
        if !self.principal_keys.contains_key(k) {
            self.principal_keys.insert(k.clone(), BTreeSet::new());
        }
        
        let keys = self.principal_keys.get_mut(k).unwrap();
        if keys.iter().any(|e| self.keys[e] == v.clone()) {
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
    pub fn authorization_insert(
        &mut self,
        id: &AuthorizationId,
        k: &PrescriptionId,
        v: &Authorization
    ) -> Result<(), String> {
        if !self.prescription_authorizations.contains_key(k) {
            self.prescription_authorizations.insert(k.clone(), BTreeSet::new());
        }
        
        let auths = self.prescription_authorizations.get_mut(k).unwrap();
        if auths.iter().any(|e| self.authorizations[e] == v.clone()) {
            Err("Authorization already exists".to_string())
        }
        else {
            self.authorizations.insert(id.clone(), v.clone());
            auths.insert(id.clone());
            Ok(())
        }
    }
    
}
