pub mod models;
pub mod db;
pub mod utils;
pub mod services;

use std::cell::RefCell;
use std::rc::Rc;
use candid::{Principal, CandidType};
use db::traits::{table::Table, crud::{Crud, Pagination}};
use ic_cdk::api::stable;
use ic_cdk::{caller, trap};
use serde::Deserialize;
use db::DB;
use models::prescription_auth::{PrescritipionAuthRequest, PrescriptionAuthResponse, PrescriptionAuth, PrescriptionAuthId};
use models::doctor::{Doctor, DoctorRequest, DoctorResponse, DoctorId};
use models::key::{KeyRequest, KeyResponse, Key, KeyId};
use models::patient::{Patient, PatientRequest, PatientResponse, PatientId};
use models::prescription::{PrescriptionRequest, PrescriptionResponse, Prescription, PrescriptionId};
use models::staff::{StaffRequest, Staff, StaffResponse, StaffId};
use models::thirdparty::{ThirdPartyRequest, ThirdPartyResponse, ThirdParty, ThirdPartyId};
use models::user::{UserResponse, UserKind, UserKindResponse, UserId};
use services::{doctors::DoctorsService, users::UsersService, patients::PatientsService, thirdparties::ThirdPartiesService, staff::StaffService, prescriptions::PrescriptionsService, keys::KeysService};
use utils::serdeser::{serialize, deserialize};
use crate::db::tables::doctors::DoctorsTable;
use crate::db::tables::doctor_prescriptions_rel::DoctorPrescriptionsRelTable;
use crate::db::tables::users::UsersTable;
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

const STATE_VERSION: f32 = 0.1;

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
        Rc::new(RefCell::new(UsersTable::new())), 
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
        if let Err(err) = serialize(&state.take(), STATE_VERSION, &mut writter) {
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
        match deserialize(STATE_VERSION, &mut reader) {
            Err(err) =>
            trap(&format!(
                "An error occurred when loading STATE from stable memory (post_upgrade): {:?}",
                err
            )),
            Ok(state_) => state.replace(state_)
        }
    });
}

/*
 * users facade
 */
#[ic_cdk::query]
fn user_find_me(
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match UsersService::find_by_id(&caller, &db.borrow(), &caller) {
            Ok(user) => {
                Ok(UserResponse{ 
                    kind: match user.kind {
                        UserKind::Doctor(_) => 
                            UserKindResponse::Doctor(DoctorsService::find_by_id(&caller, &db.borrow()).unwrap().into()),
                        UserKind::Patient(_) =>     
                            UserKindResponse::Patient(PatientsService::find_by_id(&caller, &db.borrow()).unwrap().into()),
                        UserKind::ThirdParty(_) => 
                            UserKindResponse::ThirdParty(ThirdPartiesService::find_by_id(&caller, &db.borrow()).unwrap().into()),
                        UserKind::Staff(_) => 
                            UserKindResponse::Staff(StaffService::find_by_id(&caller, &db.borrow()).unwrap().into()),
                    }, 
                    active: user.active, 
                    banned: user.banned, 
                })
            },
            Err(msg) => Err(msg)
        }
    })
}

/*
 * doctors facade
 */
#[ic_cdk::update]
fn doctor_create(
    req: DoctorRequest
) -> Result<DoctorResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let doctor = Doctor::new(&req, &caller);
        match DoctorsService::create(&doctor, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(doctor.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn doctor_update(
    id: DoctorId,
    req: DoctorRequest
) -> Result<DoctorResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let doctor = Doctor::new(&req, &caller);
        match DoctorsService::update(&id, &doctor, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(doctor.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn doctor_delete(
    id: DoctorId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        DoctorsService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn doctor_find_by_id(
    id: DoctorId
) -> Result<DoctorResponse, String> {
    DB.with(|db| {
        match DoctorsService::find_by_id(&id, &db.borrow()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn doctor_find_prescriptions(
    id: DoctorId,
    pag: Pagination
) -> Result<Vec<PrescriptionId>, String> {
    let caller = caller();

    DB.with(|db| {
        DoctorsService::find_prescriptions(&id, pag, &db.borrow(), &caller)
    })
}

/*
 * patients facade
 */
#[ic_cdk::update]
fn patient_create(
    req: PatientRequest
) -> Result<PatientResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let patient = Patient::new(&req, &caller);
        match PatientsService::create(&patient, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(patient.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn patient_update(
    id: PatientId,
    req: PatientRequest
) -> Result<PatientResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let patient = Patient::new(&req, &caller);
        match PatientsService::update(&id, &patient, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(patient.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn patient_delete(
    id: PatientId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        PatientsService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn patient_find_by_id(
    id: PatientId
) -> Result<PatientResponse, String> {
    DB.with(|db| {
        match PatientsService::find_by_id(&id, &db.borrow()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn patient_find_prescriptions(
    id: PatientId,
    pag: Pagination
) -> Result<Vec<PrescriptionId>, String> {
    let caller = caller();

    DB.with(|db| {
        PatientsService::find_prescriptions(&id, pag, &db.borrow(), &caller)
    })
}

/*
 * staff facade
 */
#[ic_cdk::update]
fn staff_create(
    req: StaffRequest
) -> Result<StaffResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let staff = Staff::new(&req, &caller);
        match StaffService::create(&staff, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(staff.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn staff_update(
    id: StaffId,
    req: StaffRequest
) -> Result<StaffResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let staff = Staff::new(&req, &caller);
        match StaffService::update(&id, &staff, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(staff.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn staff_delete(
    id: StaffId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        StaffService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn staff_find_by_id(
    id: StaffId
) -> Result<StaffResponse, String> {
    DB.with(|db| {
        match StaffService::find_by_id(&id, &db.borrow()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

/*
 * thirdparty facade
 */
#[ic_cdk::update]
fn thirdparty_create(
    req: ThirdPartyRequest
) -> Result<ThirdPartyResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let thirdparty = ThirdParty::new(&req, &caller);
        match ThirdPartiesService::create(&thirdparty, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(thirdparty.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn thirdparty_update(
    id: ThirdPartyId,
    req: ThirdPartyRequest
) -> Result<ThirdPartyResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let thirdparty = ThirdParty::new(&req, &caller);
        match ThirdPartiesService::update(&id, &thirdparty, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(thirdparty.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn thirdparty_delete(
    id: ThirdPartyId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        ThirdPartiesService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn thirdparty_find_by_id(
    id: ThirdPartyId
) -> Result<ThirdPartyResponse, String> {
    DB.with(|db| {
        match ThirdPartiesService::find_by_id(&id, &db.borrow()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

/*
 * keys facade
 */
#[ic_cdk::update]
fn key_create(
    req: KeyRequest
) -> Result<KeyResponse, String> {
    let caller = caller();

    DB.with(|rc| {
        let key = Key::new(&req, &caller);
        match KeysService::create(&key, &mut rc.borrow_mut(), &caller) {
            Ok(()) => Ok(key.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn key_find_by_id(
    id: KeyId
) -> Result<KeyResponse, String> {
    let caller = &caller();

    DB.with(|db| {
        match KeysService::find_by_id(&id, &db.borrow(), &caller) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn key_find_all_by_user(
    id: UserId,
    pag: Pagination
) -> Result<Vec<KeyResponse>, String> {
    let caller = &caller();

    DB.with(|db| {
        match KeysService::find_all_by_user(&id, pag, &db.borrow(), &caller) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
            Err(msg) => Err(msg)
        }
    })
}

/*
 * prescriptions facade
 */
#[ic_cdk::update]
fn prescription_create(
    req: PrescriptionRequest
) -> Result<PrescriptionResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let id = _gen_id();
        let prescription = Prescription::new(&id, &req, &caller);

        match PrescriptionsService::create(&prescription, &mut db.borrow_mut(), &caller) {
            Ok(_) => Ok(prescription.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn prescription_find_by_id(
    id: PrescriptionId
) -> Result<PrescriptionResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match PrescriptionsService::find_by_id(&id, &db.borrow(), &caller) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

/*
 * prescriptions access authorization facade
 */
#[ic_cdk::update]
fn prescription_auth_create(
    req: PrescritipionAuthRequest
) -> Result<PrescriptionAuthResponse, String> {
    let caller = caller();

    DB.with(|rc| {
        let id = _gen_id();
        let auth = PrescriptionAuth::new(&id, &req, &caller);
        match rc.borrow_mut().prescription_auths.borrow_mut().insert(id, auth.clone()) {
            Ok(()) => Ok(auth.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn prescription_auth_find_by_id(
    id: PrescriptionAuthId
) -> Result<PrescriptionAuthResponse, String> {
    DB.with(|db| {
        match db.borrow_mut().prescription_auths.borrow_mut().find_by_id(&id) {
            Some(e) => Ok(e.to_owned().into()),
            None => Err("Not found".to_string())
        }
    })
}
