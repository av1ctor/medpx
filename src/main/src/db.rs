use std::collections::{BTreeMap, BTreeSet};
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::{doctor::Doctor, patient::Patient, prescription::Prescription, prescription_template::PrescriptionTemplate, staff::Staff, key::Key};

#[derive(Default, CandidType, Deserialize)]
pub struct DB {
    pub staff: BTreeMap<Principal, Staff>,
    pub doctors: BTreeMap<Principal, Doctor>,
    pub patients: BTreeMap<Principal, Patient>,
    pub prescriptions: BTreeMap<String, Prescription>,
    pub doctor_prescriptions: BTreeMap<Principal, BTreeSet<String>>,
    pub patient_prescriptions: BTreeMap<Principal, BTreeSet<String>>,
    pub prescription_templates: BTreeMap<String, PrescriptionTemplate>,
    pub key_principal: BTreeMap<String, Principal>,
    pub principal_keys: BTreeMap<Principal, BTreeSet<Key>>,
}

impl DB {
    /**
     * staff table
     */
    pub fn staff_insert(
        &mut self,
        k: &Principal,
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
        k: &Principal,
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
        k: &Principal 
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
        k: &Principal,
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
        k: &Principal,
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
        k: &Principal 
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
        k: &Principal,
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
        k: &Principal,
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
        k: &Principal
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
        k: &String,
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
     * key table
     */
    pub fn key_insert(
        &mut self,
        k: &Principal,
        v: &Key
    ) -> Result<(), String> {
        if !self.principal_keys.contains_key(k) {
            self.principal_keys.insert(k.clone(), BTreeSet::new());
        }
        
        let keys = self.principal_keys.get_mut(k).unwrap();
        if keys.contains(v) {
            Err("Key already exists".to_string())
        }
        else {
            keys.insert(v.clone());
            self.key_principal.insert(v.value.clone(), k.clone());
            Ok(())
        }
    }
    
}
