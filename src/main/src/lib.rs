pub mod models;
pub mod db;

use std::cell::RefCell;
use candid::{Principal, CandidType};
use ic_cdk::{caller, trap};
use serde::Deserialize;
use db::DB;
use models::prescription_auth::{PrescritipionAuthRequest, PrescriptionAuthResponse, PrescriptionAuth};
use models::doctor::{Doctor, DoctorRequest, DoctorResponse};
use models::key::{KeyRequest, KeyResponse, Key};
use models::patient::{Patient, PatientRequest, PatientResponse};
use models::prescription::{Prescription, PrescriptionRequest, PrescriptionResponse};
use models::staff::{StaffRequest, Staff, StaffResponse};
use models::thirdparty::{ThirdPartyRequest, ThirdPartyResponse, ThirdParty};

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
        let mut state = rc.borrow_mut();
        state.owner = Some(caller());
    });
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        DB.with(|db| {
            if let Err(err) = ic_cdk::storage::stable_save::<(&State, &DB)>((&state.borrow(), &db.borrow())) {
                trap(&format!(
                    "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                    err
                ));
            };
        });
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with(|state| {
        DB.with(|db| {
            match ic_cdk::storage::stable_restore::<(State, DB)>() {
                Ok((state_, db_)) => {
                    state.replace(state_);
                    db.replace(db_);
                }
                Err(err) => {
                    trap(&format!(
                        "An error occurred when loading from stable memory (post_upgrade): {:?}",
                        err
                    ));
                }
            }
        });
    });
}

#[ic_cdk::update]
fn doctor_create(
    req: DoctorRequest
) -> Result<DoctorResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let doctor = Doctor::new(&req, caller);
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
        let patient = Patient::new(&req, caller);
        match db.patient_insert(caller, &patient) {
            Ok(()) => Ok(patient.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn staff_create(
    req: StaffRequest
) -> Result<StaffResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let staff = Staff::new(&req, caller);
        match db.staff_insert(caller, &staff) {
            Ok(()) => Ok(staff.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn thirdparty_create(
    req: ThirdPartyRequest
) -> Result<ThirdPartyResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let thirdparty = ThirdParty::new(&req, caller);
        match db.thirdparty_insert(caller, &thirdparty) {
            Ok(()) => Ok(thirdparty.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn key_create(
    req: KeyRequest
) -> Result<KeyResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let id = Key::unique_id(&req.country, &req.kind, &req.value);
        let key = Key::new(&id, &req, caller);
        match db.key_insert(&id, caller, &key) {
            Ok(()) => Ok(key.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn prescription_create(
    req: PrescriptionRequest
) -> Result<PrescriptionResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();

        if db.doctor_find_by_id(caller).is_none() {
            return Err("Doctor not found".to_string());
        }
    
        if db.patient_find_by_id(&req.patient).is_none() {
            return Err("Patient not found".to_string());
        }

        let id = _gen_id();
        let prescription = Prescription::new(&id, &req, caller);

        if let Err(msg) = db.prescription_insert(&id, &prescription) {
            return Err(msg);
        };

        Ok(prescription.into())
    })
}

#[ic_cdk::update]
fn prescription_auth_create(
    req: PrescritipionAuthRequest
) -> Result<PrescriptionAuthResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let mut db = rc.borrow_mut();
        let id = _gen_id();
        let auth = PrescriptionAuth::new(&id, &req, caller);
        match db.prescription_auth_insert(&id, &auth) {
            Ok(()) => Ok(auth.into()),
            Err(msg) => Err(msg)
        }
    })
}
