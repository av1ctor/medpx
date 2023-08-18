import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, Pagination, PatientRequest, PatientResponse, PrescriptionResponse } from "../../../declarations/main/main.did";

export const patientCreate = async (
    main: Main,
    req: PatientRequest
): Promise<PatientResponse> => {
    const res = await main.patient_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const patientUpdate = async (
    main: Main,
    id: Principal,
    req: PatientRequest
): Promise<PatientResponse> => {
    const res = await main.patient_update(id, req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const patientDelete = async (
    main: Main,
    id: Principal
): Promise<void> => {
    const res = await main.patient_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const patientFindPrescriptions = async (
    main: Main,
    principal: Principal,
    pag: Pagination
): Promise<PrescriptionResponse[]> => {
    const res = await main.patient_find_prescriptions(principal, pag);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};