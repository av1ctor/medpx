pub mod models;
pub mod db;
pub mod utils;
pub mod services;

use std::cell::RefCell;
use candid::{Principal, CandidType};
use db::traits::crud::Pagination;
use ic_cdk::api::stable;
use ic_cdk::{caller, trap};
use models::group::{GroupRequest, GroupResponse, Group, GroupId};
use serde::Deserialize;
use db::DB;
use models::prescription_auth::{PrescriptionAuthRequest, PrescriptionAuthResponse, 
    PrescriptionAuth, PrescriptionAuthId};
use models::doctor::{Doctor, DoctorRequest, DoctorResponse, DoctorId};
use models::key::{KeyRequest, KeyResponse, Key, KeyId, KeyKind};
use models::patient::{Patient, PatientRequest, PatientResponse, PatientId};
use models::prescription::{PrescriptionRequest, PrescriptionResponse, Prescription, PrescriptionId};
use models::staff::{StaffRequest, Staff, StaffResponse, StaffId};
use models::thirdparty::{ThirdPartyRequest, ThirdPartyResponse, ThirdParty, ThirdPartyId};
use models::user::{UserResponse, UserId};
use services::groups::GroupsService;
use services::{doctors::DoctorsService, users::UsersService, patients::PatientsService, 
    thirdparties::ThirdPartiesService, staff::StaffService, prescriptions::PrescriptionsService, keys::KeysService, prescription_auths::PrescriptionAuthsService};
use utils::{serdeser::{serialize, deserialize}, vetkd::VetKdUtil};

const STATE_VERSION: f32 = 0.1;
const VETKD_SYSTEM_API_CANISTER_ID: &str = "s55qq-oqaaa-aaaaa-aaakq-cai";

#[derive(Default, CandidType, Deserialize)]
struct State {
    owner: Option<Principal>,
    counter: u128,
    vetkd: VetKdUtil,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static DB: RefCell<DB> = RefCell::new(DB::new());    
}

fn _gen_id(
) -> String {
    let counter = STATE.with(|s| {
        let cnt = &mut s.borrow_mut().counter;
        *cnt += 1;
        cnt.clone()
    });

    ulid::Ulid::from_parts(ic_cdk::api::time() / 1000000, counter).to_string()
}

#[ic_cdk::init]
fn init(
) {
    ic_cdk::setup();

    STATE.with(|rc| {
        let mut state = rc.borrow_mut();
        state.owner = Some(caller());
        state.vetkd = VetKdUtil::new(VETKD_SYSTEM_API_CANISTER_ID.to_string(), "test_key_1".to_string());
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
                Ok(UserResponse {
                    kind: UsersService::find_by_kind(&caller, user.kind, &db.borrow()),
                    active: user.active, 
                    banned: user.banned, 
                })
            },
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn user_find_id(
    id: UserId
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match UsersService::find_by_id(&id, &db.borrow(), &caller) {
            Ok(user) => {
                Ok(UserResponse {
                    kind: UsersService::find_by_kind(&id, user.kind, &db.borrow()),
                    active: user.active, 
                    banned: user.banned, 
                })
            },
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn user_find_by_key(
    kind: KeyKind,
    country: Option<String>,
    key: String,
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match KeysService::find_by_value(&kind, &country, &key, &db.borrow(), &caller) {
            Ok(key) => {
                match UsersService::find_by_id(&key.created_by, &db.borrow(), &caller) {
                    Ok(user) => {
                        Ok(UserResponse {
                            kind: UsersService::find_by_kind(&key.created_by, user.kind, &db.borrow()),
                            active: user.active, 
                            banned: user.banned, 
                        })
                    },
                    Err(msg) => Err(msg)
                }
            },    
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
async fn user_get_public_key(
    derivation_path: Vec<u8>
) -> Result<String, String> {
    let caller = &caller();

    match DB.with(|db| {
        UsersService::find_by_id(caller, &db.borrow(), caller)
    }) {
        Err(msg) => return Err(msg),
        Ok(_) => {}
    };
    
    STATE.with(|state| {
        UsersService::get_public_key(
            state.borrow().vetkd.clone(),
            derivation_path
        )
    }).await
}

#[ic_cdk::update]
async fn user_get_encrypted_symmetric_key(
    derivation_path: Vec<u8>,
    derivation_id: Vec<u8>,
    encryption_public_key: Vec<u8>
) -> Result<String, String> {
    let caller = caller();

    match DB.with(|db| {
        UsersService::find_by_id(&caller, &db.borrow(), &caller)
    }) {
        Err(msg) => return Err(msg),
        Ok(_) => {}
    };

    STATE.with(|rc| {
        UsersService::get_encrypted_symmetric_key(
            rc.borrow().vetkd.clone(), 
            derivation_path,
            derivation_id,
            encryption_public_key
        )
    }).await
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
) -> Result<Vec<PrescriptionResponse>, String> {
    let caller = caller();

    DB.with(|db| {
        match DoctorsService::find_prescriptions(&id, pag, &db.borrow(), &caller) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
            Err(msg) => Err(msg)
        }
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
) -> Result<Vec<PrescriptionResponse>, String> {
    let caller = caller();

    DB.with(|db| {
        match PatientsService::find_prescriptions(&id, pag, &db.borrow(), &caller) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
            Err(msg) => Err(msg)
        }
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

#[ic_cdk::query]
fn thirdparty_find_prescriptions(
    id: ThirdPartyId,
    pag: Pagination
) -> Result<Vec<PrescriptionResponse>, String> {
    let caller = caller();

    DB.with(|db| {
        match ThirdPartiesService::find_prescriptions(&id, pag, &db.borrow(), &caller) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
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

#[ic_cdk::update]
fn key_delete(
    id: KeyId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        KeysService::delete(&id, &mut db.borrow_mut(), &caller)
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
fn key_find_by_value(
    kind: KeyKind,
    country: Option<String>,
    value: String
) -> Result<KeyResponse, String> {
    let caller = &caller();

    DB.with(|db| {
        match KeysService::find_by_value(&kind, &country, &value, &db.borrow(), &caller) {
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

#[ic_cdk::update]
fn prescription_delete(
    id: PrescriptionId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        PrescriptionsService::delete(&id, &mut db.borrow_mut(), &caller)
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
    req: PrescriptionAuthRequest
) -> Result<PrescriptionAuthResponse, String> {
    let caller = caller();

    DB.with(|rc| {
        let id = _gen_id();
        let auth = PrescriptionAuth::new(&id, &req, &caller);
        match PrescriptionAuthsService::create(&auth, &mut rc.borrow_mut(), &caller) {
            Ok(()) => Ok(auth.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn prescription_auth_delete(
    id: PrescriptionAuthId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|rc| {
        PrescriptionAuthsService::delete(&id, &mut rc.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn prescription_auth_find_by_id(
    id: PrescriptionAuthId
) -> Result<PrescriptionAuthResponse, String> {
    DB.with(|db| {
        match PrescriptionAuthsService::find_by_id(&id, &db.borrow(), &caller()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn prescription_auth_find_by_prescription(
    id: PrescriptionId
) -> Result<Vec<PrescriptionAuthResponse>, String> {
    DB.with(|db| {
        match PrescriptionAuthsService::find_by_prescription(&id, &db.borrow(), &caller()) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
            Err(msg) => Err(msg)
        }
    })
}

/*
 * groups facade
 */
#[ic_cdk::update]
fn group_create(
    req: GroupRequest
) -> Result<GroupResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let id = _gen_id();
        let group = Group::new(&id, &req, &caller);
        match GroupsService::create(&group, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(group.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn group_update(
    id: GroupId,
    req: GroupRequest
) -> Result<GroupResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let group = Group::new(&id, &req, &caller);
        match GroupsService::update(&id, &group, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(group.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn group_delete(
    id: GroupId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        GroupsService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

#[ic_cdk::query]
fn group_find_by_id(
    id: GroupId
) -> Result<GroupResponse, String> {
    DB.with(|db| {
        match GroupsService::find_by_id(&id, &db.borrow()) {
            Ok(e) => Ok(e.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn group_find_all_by_user(
    id: UserId,
    pag: Pagination
) -> Result<Vec<GroupResponse>, String> {
    let caller = &caller();

    DB.with(|db| {
        match GroupsService::find_all_by_user(&id, pag, &db.borrow(), &caller) {
            Ok(list) => Ok(list.iter().map(|e| e.clone().into()).collect()),
            Err(msg) => Err(msg)
        }
    })
}