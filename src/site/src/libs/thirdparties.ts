import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, ThirdPartyKind, ThirdPartyRequest, ThirdPartyResponse } from "../../../declarations/main/main.did";

export const kinds = [
    { value: 'Hospital', label: 'Hospital' },
    { value: 'DrugStore', label: 'Drug store' },
    { value: 'Other', label: 'Other' },
];

export const thirdPartyGetKind = (
    kind: ThirdPartyKind
): string => {
    if('Hospital' in kind)
        return 'Hospital'
    else if('DrugStore' in kind)
        return 'DrugStore'
    else
        return 'Other';
};

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