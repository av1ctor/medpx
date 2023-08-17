import { _SERVICE as Main, PatientRequest, PatientResponse } from "../../../declarations/main/main.did";

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