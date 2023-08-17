import { useMutation, useQueryClient } from "react-query";
import { PatientRequest, PatientResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { patientCreate } from "../libs/patients";

interface PatientMethods {
    create: (req: PatientRequest) => Promise<PatientResponse>;
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
    
    return {
        create
    }
};