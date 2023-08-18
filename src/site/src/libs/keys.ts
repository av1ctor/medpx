import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, Pagination, KeyRequest, KeyResponse, KeyKind } from "../../../declarations/main/main.did";

export const kinds = [
    { value: 'EmailAddress', label: 'Email address' },
    { value: 'PassportNumber', label: 'Passport number' },
    { value: 'PhoneNumber', label: 'Phone number' },
    { value: 'IdCardNumber', label: 'Id card number' },
    { value: 'DriverLicenseNumber', label: 'Driver license number' },
    { value: 'DoctorLicenseNumber', label: 'Doctor license number' },
    { value: 'Random', label: 'Random key' },
];

export const keyGetKind = (
    kind: KeyKind
): {value: string, label: string} => {
    let values = kinds.map(k => k.value);
    for(let value of values) {
        if(value in kind) {
            return kinds.filter(k => k.value === value)[0];
        }
    }
    
    return {label: 'Unkonwn', value: 'Unkonwn'};
};

export const keyCreate = async (
    main: Main,
    req: KeyRequest
): Promise<KeyResponse> => {
    const res = await main.key_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const keyDelete = async (
    main: Main,
    id: string
): Promise<void> => {
    const res = await main.key_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const keyFindByUser = async (
    main: Main,
    principal: Principal,
    pag: Pagination
): Promise<KeyResponse[]> => {
    const res = await main.key_find_all_by_user(principal, pag);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};