import { _SERVICE as Main, DoctorRequest, DoctorResponse } from "../../../declarations/main/main.did";

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