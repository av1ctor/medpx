import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, ThirdPartyKind, ThirdPartyRequest, ThirdPartyResponse } from "../../../declarations/main/main.did";

export const kinds = [
    { value: 'Hospital', label: 'Hospital' },
    { value: 'DrugStore', label: 'Drug store' },
    { value: 'Other', label: 'Other' },
];

export const thirdPartyGetKind = (
    kind: ThirdPartyKind
): {value: string, label: string} => {
    let values = kinds.map(k => k.value);
    for(let value of values) {
        if(value in kind) {
            return kinds.filter(k => k.value === value)[0];
        }
    }
    
    return {label: 'Unkonwn', value: 'Unkonwn'};
};


export const thirdPartyCreate = async (
    main: Main,
    req: ThirdPartyRequest
): Promise<ThirdPartyResponse> => {
    const res = await main.thirdparty_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
}

export const thirdPartyUpdate = async (
    main: Main,
    id: Principal,
    req: ThirdPartyRequest
): Promise<ThirdPartyResponse> => {
    const res = await main.thirdparty_update(id, req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const thirdPartyDelete = async (
    main: Main,
    id: Principal
): Promise<void> => {
    const res = await main.thirdparty_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

