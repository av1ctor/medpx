import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, DoctorRequest, DoctorResponse, PrescriptionResponse, Pagination } from "../../../declarations/main/main.did";

export const doctorCreate = async (
    main: Main,
    req: DoctorRequest
): Promise<DoctorResponse> => {
    const res = await main.doctor_create(req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const doctorUpdate = async (
    main: Main,
    id: Principal,
    req: DoctorRequest
): Promise<DoctorResponse> => {
    const res = await main.doctor_update(id, req);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};

export const doctorDelete = async (
    main: Main,
    id: Principal
): Promise<void> => {
    const res = await main.doctor_delete(id);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return; 
};

export const doctorFindPrescriptions = async (
    main: Main,
    principal: Principal,
    pag: Pagination
): Promise<PrescriptionResponse[]> => {
    const res = await main.doctor_find_prescriptions(principal, pag);
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};