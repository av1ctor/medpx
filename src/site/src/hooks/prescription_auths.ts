import { UseQueryResult, useMutation, useQuery, useQueryClient } from "react-query";
import { PrescriptionAuthRequest, PrescriptionAuthResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { useAuth } from "./auth";
import { prescriptionAuthCreate, prescriptionAuthDelete, prescriptionAuthFindById, prescriptionAuthFindByPrescription } from "../libs/prescription_auths";

interface PrescriptionAuthMethods {
    create: (req: PrescriptionAuthRequest) => Promise<PrescriptionAuthResponse>;
    remove: (prescription: PrescriptionAuthResponse) => Promise<void>;
}

export const usePrescriptionAuth = (
): PrescriptionAuthMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: PrescriptionAuthRequest}) => {
            return prescriptionAuthCreate(main, options.req);
        },
        {
            onSuccess: (data, _variables) => {
                queryClient.invalidateQueries(['prescription-auths', data.prescription_id]);
            }   
        }
    );

    const create = (
        req: PrescriptionAuthRequest
    ): Promise<PrescriptionAuthResponse> => {
        return createMut.mutateAsync({req});
    };

    const deleteMut = useMutation(
        async (options: {auth: PrescriptionAuthResponse}) => {
            return prescriptionAuthDelete(main, options.auth.id);
        },
        {
            onSuccess: (_data, variables) => {
                queryClient.invalidateQueries(['prescription-auths', variables.auth.prescription_id]);
            }  
        }
    );

    const remove = (
        auth: PrescriptionAuthResponse
    ): Promise<void> => {
        return deleteMut.mutateAsync({auth});
    };

    return {
        create,
        remove
    }
};

export const usePrescriptionAuthsFindByPrescription = (
    prescriptionId: string
): UseQueryResult<PrescriptionAuthResponse[], Error> => {
    const {main} = useActors();

    return useQuery<PrescriptionAuthResponse[], Error>(
        ['prescription-auths', prescriptionId], 
        () => prescriptionAuthFindByPrescription(main, prescriptionId)
    );
};

export const usePrescriptionAuthsFindById = (
    id: string
): UseQueryResult<PrescriptionAuthResponse, Error> => {
    const {main} = useActors();
    const {isLogged} = useAuth()

    return useQuery<PrescriptionAuthResponse, Error>(
        ['prescription-auths', id],
        () => prescriptionAuthFindById(main, id),
        {
            enabled: isLogged
        }
    );
};
