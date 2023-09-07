import { _SERVICE as Main, PrescriptionPreRequest, PrescriptionPostRequest, PrescriptionResponse } from "../../../declarations/main/main.did";

export const prescriptionPreCreate = async (
    main: Main,
    req: PrescriptionPreRequest
): Promise<PrescriptionResponse> => {
    const res = await main.prescription_pre_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const prescriptionPostCreate = async (
    main: Main,
    id: string,
    req: PrescriptionPostRequest
): Promise<PrescriptionResponse> => {
    const res = await main.prescription_post_create(id, req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const prescriptionDelete = async (
    main: Main,
    id: string
): Promise<void> => {
    const res = await main.prescription_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const prescriptionFindById = async (
    main: Main,
    id: string
): Promise<PrescriptionResponse> => {
    const res = await main.prescription_find_by_id(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};
