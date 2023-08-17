import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, Pagination, KeyRequest, KeyResponse } from "../../../declarations/main/main.did";

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