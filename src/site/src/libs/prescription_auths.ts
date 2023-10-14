import { _SERVICE as Main, PrescriptionAuthKind, PrescriptionAuthRequest, PrescriptionAuthResponse } from "../../../declarations/main/main.did";

export enum AuthSubject {
    User,
    Group,
};

export const kinds = [
    { value: 'Read', label: 'Read'},
    { value: 'Write', label: 'Write'},
    { value: 'ReadWrite', label: 'Read and write'},
    { value: 'All', label: 'All'},
];

export const prescriptionAuthStringToSubject = (
    kind: string
): AuthSubject => {
    if(kind === 'User') {
        return AuthSubject.User;
    }

    return AuthSubject.Group;
};

export const prescriptionAuthGetKind = (
    kind: PrescriptionAuthKind
): {value: string, label: string} => {
    let values = kinds.map(k => k.value);
    for(let value of values) {
        if(value in kind) {
            return kinds.filter(k => k.value === value)[0];
        }
    }
    
    return {label: 'Unknown', value: 'Unknown'};
};

export const prescriptionAuthStringTokind = (
    kind: string
): PrescriptionAuthKind => {
    return {[kind]: null} as PrescriptionAuthKind;
};

export const keyGetKindIndex = (
    kind: string
): number => {
    return kinds.findIndex(k => k.value === kind);
}

export const prescriptionAuthCreate = async (
    main: Main,
    req: PrescriptionAuthRequest
): Promise<PrescriptionAuthResponse> => {
    const res = await main.prescription_auth_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const prescriptionAuthDelete = async (
    main: Main,
    id: string
): Promise<void> => {
    const res = await main.prescription_auth_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const prescriptionAuthFindById = async (
    main: Main,
    id: string
): Promise<PrescriptionAuthResponse> => {
    const res = await main.prescription_auth_find_by_id(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const prescriptionAuthFindByPrescription = async (
    main: Main,
    id: string|undefined
): Promise<PrescriptionAuthResponse[]> => {
    if(!id) {
        throw new Error('Invalid id');
    }
    const res = await main.prescription_auth_find_by_prescription(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};
