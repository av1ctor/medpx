pub mod models;
pub mod db;
pub mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use candid::{Principal, CandidType};
use db::traits::{table::Table, crud::{Crud, CrudSubscribable}};
use ic_cdk::api::stable;
use ic_cdk::{caller, trap};
use serde::Deserialize;
use db::DB;
use models::prescription_auth::{PrescritipionAuthRequest, PrescriptionAuthResponse, PrescriptionAuth};
use models::doctor::{Doctor, DoctorRequest, DoctorResponse};
use models::key::{KeyRequest, KeyResponse, Key};
use models::patient::{Patient, PatientRequest, PatientResponse};
use models::prescription::{PrescriptionRequest, PrescriptionResponse, Prescription};
use models::staff::{StaffRequest, Staff, StaffResponse};
use models::thirdparty::{ThirdPartyRequest, ThirdPartyResponse, ThirdParty};
use utils::serdeser::{serialize, deserialize};
use crate::db::tables::doctors::DoctorsTable;
use crate::db::tables::doctor_prescriptions_rel::DoctorPrescriptionsRelTable;
use crate::db::tables::keys::KeysTable;
use crate::db::tables::key_principal_rel::KeyPrincipalRelTable;
use crate::db::tables::patients::PatientsTable;
use crate::db::tables::patient_prescriptions_rel::PatientPrescriptionsRelTable;
use crate::db::tables::prescription_auths_rel::PrescriptionAuthsRelTable;
use crate::db::tables::prescriptions::PrescriptionsTable;
use crate::db::tables::principal_keys_rel::PrincipalKeysRelTable;
use crate::db::tables::prescription_auths::PrescriptionAuthsTable;
use crate::db::tables::prescription_templates::PrescriptionTemplatesTable;
use crate::db::tables::staff::StaffTable;
use crate::db::tables::thirdparties::ThirdPartiesTable;

#[derive(Default, CandidType, Deserialize)]
struct State {
    owner: Option<Principal>,
    counter: u128,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static DB: RefCell<DB> = RefCell::new(DB::new(
        Rc::new(RefCell::new(DoctorsTable::new())), 
        Rc::new(RefCell::new(PatientsTable::new())), 
        Rc::new(RefCell::new(StaffTable::new())), 
        Rc::new(RefCell::new(ThirdPartiesTable::new())), 
        Rc::new(RefCell::new(PrescriptionsTable::new())), 
        Rc::new(RefCell::new(KeysTable::new())), 
        Rc::new(RefCell::new(PrescriptionAuthsTable::new())), 
        Rc::new(RefCell::new(PrescriptionTemplatesTable::new())),
        Rc::new(RefCell::new(DoctorPrescriptionsRelTable::new())),
        Rc::new(RefCell::new(PatientPrescriptionsRelTable::new())),
        Rc::new(RefCell::new(PrescriptionAuthsRelTable::new())),
        Rc::new(RefCell::new(PrincipalKeysRelTable::new())),
        Rc::new(RefCell::new(KeyPrincipalRelTable::new())),
    ));    
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
    let mut writter = stable::StableWriter::default();
    
    // must be the first
    DB.with(|db| {
        if let Err(err) = db.borrow().serialize(&mut writter) {
            trap(&format!(
                "An error occurred when saving DB to stable memory (pre_upgrade): {:?}",
                err
            ));
        }
    });

    STATE.with(|state| {
        if let Err(err) = serialize(&state.take(), &mut writter) {
            trap(&format!(
                "An error occurred when saving STATE to stable memory (pre_upgrade): {:?}",
                err
            ));
        }
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let mut reader = stable::StableReader::default();

    DB.with(|db| {
        if let Err(err) = db.borrow_mut().deserialize(&mut reader) {
            trap(&format!(
                "An error occurred when loading DB from stable memory (post_upgrade): {:?}",
                err
            ));
        }
    });

    STATE.with(|state| {
        match deserialize(&mut reader) {
            Err(err) =>
            trap(&format!(
                "An error occurred when loading STATE from stable memory (post_upgrade): {:?}",
                err
            )),
            Ok(state_) => state.replace(state_)
        }
    });
}

#[ic_cdk::update]
fn doctor_create(
    req: DoctorRequest
) -> Result<DoctorResponse, String> {
    let caller = &caller();

    DB.with(|rc| {
        let doctor = Doctor::new(&req, caller);
        match rc.borrow_mut().doctors.borrow_mut().insert(*caller, doctor.clone()) {
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
        let patient = Patient::new(&req, caller);
        match rc.borrow_mut().patients.borrow_mut().insert(*caller, patient.clone()) {
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
        let staff = Staff::new(&req, caller);
        match rc.borrow_mut().staff.borrow_mut().insert(*caller, staff.clone()) {
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
        let thirdparty = ThirdParty::new(&req, caller);
        match rc.borrow_mut().thirdparties.borrow_mut().insert(*caller, thirdparty.clone()) {
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
        let key = Key::new(&req, caller);
        match rc.borrow_mut().keys.borrow_mut().insert(key.id.clone(), key.clone()) {
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
        let db = rc.borrow_mut();

        if db.doctors.borrow().find_by_id(caller).is_none() {
            return Err("Doctor not found".to_string());
        }
    
        if db.patients.borrow().find_by_id(&req.patient).is_none() {
            return Err("Patient not found".to_string());
        }

        let id = _gen_id();
        let prescription = Prescription::new(&id, &req, caller);

        if let Err(msg) = db.prescriptions.borrow_mut().insert(id, prescription.clone()) {
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
        let id = _gen_id();
        let auth = PrescriptionAuth::new(&id, &req, caller);
        match rc.borrow_mut().prescription_auths.borrow_mut().insert(id, auth.clone()) {
            Ok(()) => Ok(auth.into()),
            Err(msg) => Err(msg)
        }
    })
}
