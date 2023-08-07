use std::collections::{BTreeMap, BTreeSet};
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::{doctor::Doctor, patient::Patient, prescription::Prescription};

#[derive(Default, CandidType, Deserialize)]
pub struct DB {
    pub doctors: BTreeMap<Principal, Doctor>,
    pub patients: BTreeMap<Principal, Patient>,
    pub prescriptions: BTreeMap<String, Prescription>,
    pub doctor_prescriptions: BTreeMap<Principal, BTreeSet<String>>,
    pub patient_prescriptions: BTreeMap<Principal, BTreeSet<String>>,
}

impl DB {
    /**
     * doctors table
     */
    pub fn doctor_insert(
        &mut self,
        principal: &Principal,
        doctor: &Doctor
    ) -> Result<(), String> {
        if self.doctors.contains_key(principal) {
            Err("Doctor already exists".to_string())
        }
        else {
            self.doctors.insert(principal.clone(), doctor.clone());
            self.doctor_prescriptions.insert(principal.clone(), BTreeSet::new());
            Ok(())
        }
    }

    pub fn doctor_update(
        &mut self,
        principal: &Principal,
        doctor: &Doctor
    ) -> Result<(), String> {
        if !self.doctors.contains_key(principal) {
            Err("Unknown Doctor".to_string())
        }
        else {
            self.doctors.insert(principal.clone(), doctor.clone());
            Ok(())
        }
    }

    pub fn doctor_find_by_id(
        &self,
        principal: &Principal 
    ) -> Result<Doctor, String> {
        if !self.doctors.contains_key(principal) {
            return Err("Unknown doctor".to_string());
        }

        Ok(self.doctors[principal].clone())
    }

    /**
     * patients table
     */
    pub fn patient_insert(
        &mut self,
        principal: &Principal,
        patient: &Patient
    ) -> Result<(), String> {
        if self.patients.contains_key(principal) {
            Err("Patient already exists".to_string())
        }
        else {
            self.patients.insert(principal.clone(), patient.clone());
            self.patient_prescriptions.insert(principal.clone(), BTreeSet::new());
            Ok(())
        }
    }

    pub fn patient_update(
        &mut self,
        principal: &Principal,
        patient: &Patient
    ) -> Result<(), String> {
        if !self.patients.contains_key(principal) {
            Err("Unknown Patient".to_string())
        }
        else {
            self.patients.insert(principal.clone(), patient.clone());
            Ok(())
        }
    }

    pub fn patient_find_by_id(
        &self,
        principal: &Principal
    ) -> Result<Patient, String> {
        if !self.patients.contains_key(principal) {
            return Err("Unknown patient".to_string());
        }

        Ok(self.patients[principal].clone())
    }

    /**
     * prescriptions table
     */
    pub fn prescription_insert(
        &mut self,
        id: &String,
        prescription: &Prescription
    ) {
        self.prescriptions.insert(id.clone(), prescription.clone());
        
        let prescriptions = self.doctor_prescriptions.get_mut(&prescription.doctor).unwrap();
        prescriptions.insert(id.clone());

        let prescriptions = self.patient_prescriptions.get_mut(&prescription.patient).unwrap();
        prescriptions.insert(id.clone());
    }
    
}
