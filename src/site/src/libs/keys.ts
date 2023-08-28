import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, Pagination, KeyRequest, KeyResponse, KeyKind } from "../../../declarations/main/main.did";

export enum Uniqueness {
    Worldwide,
    Countrywide,
    Statewide
}

export const kinds = [
    { value: 'EmailAddress', label: 'Email address', uniqueness: Uniqueness.Worldwide },
    { value: 'PhoneNumber', label: 'Phone number', uniqueness: Uniqueness.Countrywide },
    { value: 'Random', label: 'Random key', uniqueness: Uniqueness.Worldwide },
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
    
    return {label: 'Unknown', value: 'Unknown'};
};

export const keyStringTokind = (
    kind: string
): KeyKind => {
    return {[kind]: null} as KeyKind;
};

export const keyGetKindIndex = (
    kind: string
): number => {
    return kinds.findIndex(k => k.value === kind);
}

export const keyGetKindUniqueness = (
    kind: string
): Uniqueness => {
    return kinds.find(k => k.value === kind)?.uniqueness || Uniqueness.Worldwide;
}

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