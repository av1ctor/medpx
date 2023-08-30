import { _SERVICE as Main, PrescriptionRequest, PrescriptionResponse } from "../../../declarations/main/main.did";

export const prescriptionCreate = async (
    main: Main,
    req: PrescriptionRequest
): Promise<PrescriptionResponse> => {
    const res = await main.prescription_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const prescriptionUpdate = async (
    main: Main,
    id: string,
    req: PrescriptionRequest
): Promise<PrescriptionResponse> => {
    const res = await main.prescription_update(id, req);
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
