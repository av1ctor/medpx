use std::{cell::RefCell, collections::{BTreeMap, BTreeSet}};
use candid::{Principal, CandidType};
use doctor::{Doctor, DoctorRequest, DoctorResponse};
use ic_cdk::{caller, trap};
use patient::{Patient, PatientRequest, PatientResponse};
use prescription::{Prescription, PrescriptionRequest, PrescriptionResponse};
use serde::Deserialize;

pub mod doctor;
pub mod patient;
pub mod prescription;

#[derive(Default, CandidType, Deserialize)]
struct State {
    owner: Option<Principal>,
    counter: u128,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static DOCTORS: RefCell<BTreeMap<Principal, Doctor>> = RefCell::new(BTreeMap::new());
    static PATIENTS: RefCell<BTreeMap<Principal, Patient>> = RefCell::new(BTreeMap::new());
    static PRESCRIPTIONS: RefCell<BTreeMap<String, Prescription>> = RefCell::new(BTreeMap::new());
    static DOCTOR_PRESCRIPTIONS: RefCell<BTreeMap<Principal, BTreeSet<String>>> = RefCell::new(BTreeMap::new());
    static PATIENT_PRESCRIPTIONS: RefCell<BTreeMap<Principal, BTreeSet<String>>> = RefCell::new(BTreeMap::new());
}

fn _gen_id(
) -> String {
    let counter = STATE.with(|s| {
        let mut cnt = s.borrow_mut().counter;
        cnt += 1;
        cnt
    });

    ulid::Ulid::from_parts(ic_cdk::api::time(), counter).to_string()
}

fn _find_patient(
    principal: &Principal 
) -> Result<Patient, String> {
    PATIENTS.with(|rc| {
        let patients = rc.borrow();
        if !patients.contains_key(principal) {
            return Err("Unknown patient".to_string());
        }

        Ok(patients[principal].clone())
    })
}

fn _find_doctor(
    principal: &Principal 
) -> Result<Doctor, String> {
    DOCTORS.with(|rc| {
        let doctors = rc.borrow();
        if !doctors.contains_key(principal) {
            return Err("Unknown doctor".to_string());
        }

        Ok(doctors[principal].clone())
    })
}

#[ic_cdk::init]
fn init(
) {
    ic_cdk::setup();

    STATE.with(|rc| {
        let mut s = rc.borrow_mut();
        s.owner = Some(caller());
    });
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|rc| {
        if let Err(err) = ic_cdk::storage::stable_save::<(&State,)>((
            &rc.borrow(),
        )) {
            trap(&format!(
                "An error occurred when saving STATE to stable memory (pre_upgrade): {:?}",
                err
            ));
        };
    });
    
    DOCTORS.with(|rc| {
        if let Err(err) = ic_cdk::storage::stable_save::<(&BTreeMap<Principal, Doctor>,)>((
            &rc.borrow(),
        )) {
            trap(&format!(
                "An error occurred when saving DOCTORS to stable memory (pre_upgrade): {:?}",
                err
            ));
        };
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with(|rc| {
        match ic_cdk::storage::stable_restore::<(State, )>() {
            Ok((s_, )) => {
                rc.replace(s_);
            }
            Err(err) => {
                trap(&format!(
                    "An error occurred when loading STATE from stable memory (post_upgrade): {:?}",
                    err
                ));
            }
        }
    });
}

#[ic_cdk::update]
fn doctor_create(
    req: DoctorRequest
) -> Result<DoctorResponse, String> {
    let caller = &caller();

    DOCTORS.with(|rc| {
        let mut doctors = rc.borrow_mut();
        if doctors.contains_key(caller) {
            return Err("Doctor already exists".to_string());
        }

        let doc = Doctor::new(&req);
        doctors.insert(*caller, doc.clone());

        DOCTOR_PRESCRIPTIONS.with(|rc| {
            let mut doc_presc = rc.borrow_mut();
            doc_presc.insert(*caller, BTreeSet::new());
        });

        Ok(doc.into())
    })
}

#[ic_cdk::update]
fn patient_create(
    req: PatientRequest
) -> Result<PatientResponse, String> {
    let caller = &caller();

    PATIENTS.with(|rc| {
        let mut patients = rc.borrow_mut();
        if patients.contains_key(caller) {
            return Err("Patient already exists".to_string());
        }

        let pat = Patient::new(&req);
        patients.insert(*caller, pat.clone());

        PATIENT_PRESCRIPTIONS.with(|rc| {
            let mut pat_presc = rc.borrow_mut();
            pat_presc.insert(*caller, BTreeSet::new());
        });
    
        Ok(pat.into())
    })
}

#[ic_cdk::update]
fn prescription_create(
    req: PrescriptionRequest
) -> Result<PrescriptionResponse, String> {
    let caller = &caller();

    if !caller.eq(&req.doctor) {
        return Err("Forbidden".to_string());
    }

    let mut doctor = match _find_doctor(&req.doctor) {
        Err(err) => return Err(err),
        Ok(patient) => patient
    };

    let mut patient = match _find_patient(&req.patient) {
        Err(err) => return Err(err),
        Ok(patient) => patient
    };

    let id = _gen_id();
    let prescription = PRESCRIPTIONS.with(|rc| {
        let mut prescriptions = rc.borrow_mut();
        prescriptions.insert(id.clone(), Prescription::new(&id, &req));
        prescriptions[&id].clone()
    });

    DOCTOR_PRESCRIPTIONS.with(|rc| {
        let mut doc_presc = rc.borrow_mut();
        let prescriptions = doc_presc.get_mut(&req.doctor).expect("Unknown doctor");
        prescriptions.insert(id.clone());
    });

    PATIENT_PRESCRIPTIONS.with(|rc| {
        let mut pat_presc = rc.borrow_mut();
        let prescriptions = pat_presc.get_mut(&req.patient).expect("Unknown patient");
        prescriptions.insert(id.clone());
    });

    DOCTORS.with(|rc| {
        let mut doctors = rc.borrow_mut();
        doctor.num_prescriptions += 1;
        doctors.insert(req.doctor, doctor);
    });

    PATIENTS.with(|rc| {
        let mut patients = rc.borrow_mut();
        patient.num_prescriptions += 1;
        patients.insert(req.patient, patient);
    });

    Ok(prescription.into())
}

