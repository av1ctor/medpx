import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, UserResponse } from "../../../declarations/main/main.did";

export enum UserKind {
    Doctor,
    Patient,
    ThirdParty,
    Staff, 
}

export const userGetKind = (
    user: UserResponse | undefined
): UserKind => {
    if(user !== undefined) {
        if('Doctor' in user.kind) {
            return UserKind.Doctor;
        }
        else if('Patient' in user.kind) {
            return UserKind.Patient;
        }
        else if('ThirdParty' in user.kind) {
            return UserKind.ThirdParty;
        }
    }

    return UserKind.Staff
};

export const userGetPrincipal = (
    user: UserResponse | undefined
): Principal => {
    if (user !== undefined) {
        if('Doctor' in user.kind) {
            return user.kind.Doctor.id;
        }
        else if('Patient' in user.kind) {
            return user.kind.Patient.id;
        }
        else if('ThirdParty' in user.kind) {
            return user.kind.ThirdParty.id;
        }
        else if('Staff' in user.kind) {
            return user.kind.Staff.id;
        }
    }

    return Principal.anonymous();
};

export const userFindMe = async (
    main: Main
): Promise<UserResponse> => {
    const res = await main.user_find_me();
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const userFindById = async (
    main: Main,
    id: Principal
): Promise<UserResponse> => {
    const res = await main.user_find_by_id(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const userFindByKey = async (
    main: Main,
    key: string
): Promise<UserResponse> => {
    const res = await main.user_find_by_key(key);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};