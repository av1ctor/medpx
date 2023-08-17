import { useMutation, useQueryClient } from "react-query";
import { Principal } from "@dfinity/principal";
import { PatientRequest, PatientResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { patientCreate, patientUpdate } from "../libs/patients";

interface PatientMethods {
    create: (req: PatientRequest) => Promise<PatientResponse>;
    update: (id: Principal, req: PatientRequest) => Promise<PatientResponse>;
}

export const usePatient = (
): PatientMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: PatientRequest}) => {
            return patientCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['patients']);
            }   
        }
    );

    const create = (
        req: PatientRequest
    ): Promise<PatientResponse> => {
        return createMut.mutateAsync({req});
    };

    const updateMut = useMutation(
        async (options: {id: Principal, req: PatientRequest}) => {
            return patientUpdate(main, options.id, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['patients']);
            }   
        }
    );

    const update = (
        id: Principal,
        req: PatientRequest
    ): Promise<PatientResponse> => {
        return updateMut.mutateAsync({id, req});
    };

    return {
        create,
        update,
    }
};
