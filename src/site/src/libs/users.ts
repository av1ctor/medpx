import { Principal } from "@dfinity/principal";
import { DoctorResponse, KeyKind, _SERVICE as Main, Pagination, PatientResponse, PrescriptionResponse, StaffResponse, ThirdPartyKind, ThirdPartyResponse, UserRequest, UserResponse } from "../../../declarations/main/main.did";

export enum UserKind {
    Doctor,
    Patient,
    ThirdParty,
    Staff, 
}

export const thirdPartyKinds = [
    { value: 'Hospital', label: 'Hospital' },
    { value: 'DrugStore', label: 'Drug store' },
    { value: 'Other', label: 'Other' },
];

export const userGetThirdPartyKind = (
    kind: ThirdPartyKind | undefined
): {value: string, label: string} => {
    if(kind) {
        let values = thirdPartyKinds.map(k => k.value);
        for(let value of values) {
            if(value in kind) {
                return thirdPartyKinds.filter(k => k.value === value)[0];
            }
        }
    }
    
    return {label: 'Unknown', value: 'Unknown'};
};

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

    return UserKind.Staff;
};

export const userStringToKind = (
    kind: string
): UserKind => {
    if(kind === 'Doctor') {
        return UserKind.Doctor;
    }
    else if(kind === 'Patient') {
        return UserKind.Patient;
    }
    else if(kind === 'ThirdParty') {
        return UserKind.ThirdParty;
    }

    return UserKind.Staff;
}

export const userIsKind = (
    user: UserResponse | undefined,
    kind: string
): boolean => {
    if(user !== undefined) {
        return kind in user.kind;
    }

    return false;
};

export const userGetPrincipal = (
    user: UserResponse | undefined
): Principal => {
    if (user !== undefined) {
        return user.id;
    }

    return Principal.anonymous();
};

export const userGetName = (
    user: UserResponse | undefined
): string => {
    if (user !== undefined) {
        return user.name;
    }

    return 'Unknown';
};

export const userGetDoctor = (
    user: UserResponse | undefined
): DoctorResponse => {
    if (user !== undefined) {
        if('Doctor' in user.kind) {
            return user.kind.Doctor;
        }
    }

    return {} as DoctorResponse;
};

export const userGetPatient = (
    user: UserResponse | undefined
): PatientResponse => {
    if (user !== undefined) {
        if('Patient' in user.kind) {
            return user.kind.Patient;
        }
    }

    return {} as PatientResponse;
};

export const userGetThirdParty = (
    user: UserResponse | undefined
): ThirdPartyResponse => {
    if (user !== undefined) {
        if('ThirdParty' in user.kind) {
            return user.kind.ThirdParty;
        }
    }

    return {} as ThirdPartyResponse;
};

export const userGetStaff = (
    user: UserResponse | undefined
): StaffResponse => {
    if (user !== undefined) {
        if('Staff' in user.kind) {
            return user.kind.Staff;
        }
    }

    return {} as StaffResponse;
};

export const userCreate = async (
    main: Main,
    req: UserRequest
): Promise<UserResponse> => {
    const res = await main.user_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const userUpdate = async (
    main: Main,
    id: Principal,
    req: UserRequest
): Promise<UserResponse> => {
    const res = await main.user_update(id, req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const userDelete = async (
    main: Main,
    id: Principal
): Promise<void> => {
    const res = await main.user_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
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
    kind: KeyKind,
    country: [string] | [],
    key: string
): Promise<UserResponse> => {
    const res = await main.user_find_by_key(kind, country, key);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const userFindPrescriptions = async (
    main: Main,
    principal: Principal,
    pag: Pagination
): Promise<PrescriptionResponse[]> => {
    const res = await main.user_find_prescriptions(principal, pag);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};
