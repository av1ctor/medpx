import { UseQueryResult, useMutation, useQuery, useQueryClient } from "react-query";
import { Principal } from "@dfinity/principal";
import { PatientRequest, PatientResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { patientCreate, patientDelete, patientFindById, patientUpdate } from "../libs/patients";

interface PatientMethods {
    create: (req: PatientRequest) => Promise<PatientResponse>;
    update: (id: Principal, req: PatientRequest) => Promise<PatientResponse>;
    remove: (id: Principal) => Promise<void>;
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

    const deleteMut = useMutation(
        async (options: {id: Principal}) => {
            return patientDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['patients']);
            }   
        }
    );

    const remove = (
        id: Principal
    ): Promise<void> => {
        return deleteMut.mutateAsync({id});
    };

    return {
        create,
        update,
        remove,
    }
};

export const usePatientFindById = (
    id: Principal
): UseQueryResult<PatientResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<PatientResponse, Error>(
        ['patients', id],
        () => patientFindById(main, id)
    );
}; 
