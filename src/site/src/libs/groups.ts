import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, Pagination, GroupRequest, GroupResponse } from "../../../declarations/main/main.did";

export const groupCreate = async (
    main: Main,
    req: GroupRequest
): Promise<GroupResponse> => {
    const res = await main.group_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const groupDelete = async (
    main: Main,
    id: string
): Promise<void> => {
    const res = await main.group_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const groupFindById = async (
    main: Main,
    id: string
): Promise<GroupResponse> => {
    const res = await main.group_find_by_id(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const groupFindByUser = async (
    main: Main,
    principal: Principal,
    pag: Pagination
): Promise<GroupResponse[]> => {
    const res = await main.group_find_all_by_user(principal, pag);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};