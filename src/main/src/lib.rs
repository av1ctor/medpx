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
use models::key::{KeyRequest, KeyResponse, Key, KeyId, KeyKind};
use models::prescription::{PrescriptionRequest, PrescriptionResponse, Prescription, PrescriptionId};
use models::user::{UserResponse, UserId, UserRequest, User};
use services::groups::GroupsService;
use services::{users::UsersService, prescriptions::PrescriptionsService, keys::KeysService, prescription_auths::PrescriptionAuthsService};
use utils::random::Xoshiro256ss;
use utils::{serdeser::{serialize, deserialize}, vetkd::VetKdUtil};

const STATE_VERSION: f32 = 0.1;

#[derive(Default, CandidType, Deserialize)]
struct State {
    owner: Option<Principal>,
    vetkd: VetKdUtil,
    rand: Xoshiro256ss,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static DB: RefCell<DB> = RefCell::new(DB::new());    
}

fn _gen_id(
) -> String {
    let (lsu64, msu64) = STATE.with(|rc| {
        let mut state = rc.borrow_mut();
        (state.rand.next(), state.rand.next())
    });
    ulid::Ulid::from_parts(ic_cdk::api::time() / 1000000, (msu64 as u128) << 64 | (lsu64 as u128)).to_string()
}

#[derive(CandidType, Deserialize)]
struct InitArg {
    vetkd_canister_id: String,
    key_name: String
}

#[ic_cdk::init]
fn init(
    arg: InitArg
) {
    ic_cdk::setup();

    STATE.with(|rc| {
        let mut state = rc.borrow_mut();
        state.owner = Some(caller());
        state.vetkd = VetKdUtil::new(arg.vetkd_canister_id, arg.key_name);
        state.rand = Xoshiro256ss::new(ic_cdk::api::time());
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
#[ic_cdk::update]
fn user_create(
    req: UserRequest
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let user = User::new(&req, &caller);
        match UsersService::create(&user, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(user.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn user_update(
    id: UserId,
    req: UserRequest
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let user = User::new(&req, &caller);
        match UsersService::update(&id, &user, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(user.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::update]
fn user_delete(
    id: UserId
) -> Result<(), String> {
    let caller = caller();

    DB.with(|db| {
        UsersService::delete(&id, &mut db.borrow_mut(), &caller)
    })
}

 #[ic_cdk::query]
fn user_find_me(
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match UsersService::find_by_id(&caller, &db.borrow(), &caller) {
            Ok(user) => Ok(user.into()),
            Err(msg) => Err(msg)
        }
    })
}

#[ic_cdk::query]
fn user_find_by_id(
    id: UserId
) -> Result<UserResponse, String> {
    let caller = caller();

    DB.with(|db| {
        match UsersService::find_by_id(&id, &db.borrow(), &caller) {
            Ok(user) => Ok(user.into()),
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
                    Ok(user) => Ok(user.into()),
                    Err(msg) => Err(msg)
                }
            },    
            Err(msg) => Err(msg)
        }
    })
}
#[ic_cdk::query]
fn user_find_prescriptions(
    id: UserId,
    pag: Pagination
) -> Result<Vec<PrescriptionResponse>, String> {
    let caller = caller();

    DB.with(|db| {
        match UsersService::find_prescriptions(&id, pag, &db.borrow(), &caller) {
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
fn prescription_update(
    id: PrescriptionId,
    req: PrescriptionRequest
) -> Result<PrescriptionResponse, String> {
    let caller = caller();

    DB.with(|db| {
        let prescription = Prescription::new(&id, &req, &caller);
        match PrescriptionsService::update(&id, &prescription, &mut db.borrow_mut(), &caller) {
            Ok(()) => Ok(prescription.into()),
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

#[ic_cdk::update]
async fn prescription_get_public_key(
) -> Result<String, String> {
    let caller = caller();

    match DB.with(|db| {
        UsersService::find_by_id(&caller, &db.borrow(), &caller)
    }) {
        Err(msg) => return Err(msg),
        Ok(_) => ()
    };

    STATE.with(|state| {
        PrescriptionsService::get_public_key(
            state.borrow().vetkd.clone()
        )
    }).await
}

#[ic_cdk::update]
async fn prescription_get_encrypted_symmetric_key(
    id: String,
    encryption_public_key: Vec<u8>
) -> Result<String, String> {
    let caller = &caller();

    let prescription = match DB.with(|db| {
        let db = &db.borrow();
        if let Err(err) = UsersService::find_by_id(caller, db, caller) {
            return Err(err);
        }

        PrescriptionsService::find_by_id(&id, db, caller)
    }) {
        Err(msg) => return Err(msg),
        Ok(pres) => pres
    };

    STATE.with(move |state| {
        PrescriptionsService::get_encrypted_symmetric_key(
            prescription.hash.clone(),
            encryption_public_key,
            state.borrow().vetkd.clone()
        )
    }).await
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