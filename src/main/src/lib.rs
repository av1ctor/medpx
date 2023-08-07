use std::cell::RefCell;
use candid::{Principal, CandidType};
use db::DB;
use doctor::{Doctor, DoctorRequest, DoctorResponse};
use ic_cdk::{caller, trap};
use patient::{Patient, PatientRequest, PatientResponse};
use prescription::{Prescription, PrescriptionRequest, PrescriptionResponse};
use serde::Deserialize;

pub mod doctor;
pub mod patient;
pub mod prescription;
pub mod db;

#[derive(Default, CandidType, Deserialize)]
struct State {
    owner: Option<Principal>,
    counter: u128,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static DB: RefCell<DB> = RefCell::default();
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
    
    DB.with(|rc| {
        if let Err(err) = ic_cdk::storage::stable_save::<(&DB,)>((
            &rc.borrow(),
        )) {
            trap(&format!(
                "An error occurred when saving DB to stable memory (pre_upgrade): {:?}",
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

    DB.with(|rc| {
        match ic_cdk::storage::stable_restore::<(DB, )>() {
            Ok((s_, )) => {
                rc.replace(s_);
            }
            Err(err) => {
                trap(&format!(
                    "An error occurred when loading DB from stable memory (post_upgrade): {:?}",
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

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let doctor = Doctor::new(&req);
        match db.doctor_insert(caller, &doctor) {
            Ok(()) => Ok(doctor.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn patient_create(
    req: PatientRequest
) -> Result<PatientResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let patient = Patient::new(&req);
        match db.patient_insert(caller, &patient) {
            Ok(()) => Ok(patient.into()),
            Err(msg) => Err(msg)
        }
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

    DB.with(|rc| {
        let mut db = rc.borrow_mut();

        let mut doctor = match db.doctor_find_by_id(&req.doctor) {
            Err(msg) => return Err(msg),
            Ok(patient) => patient
        };
    
        let mut patient = match db.patient_find_by_id(&req.patient) {
            Err(msg) => return Err(msg),
            Ok(patient) => patient
        };

        let id = _gen_id();
        let prescription = Prescription::new(&id, &req);

        db.prescription_insert(
            &id, 
            &prescription, 
            &mut doctor, 
            &mut patient
        );
        
        Ok(prescription.into())
    })
}

