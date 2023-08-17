import { useMutation, useQueryClient } from "react-query";
import { Principal } from "@dfinity/principal";
import { DoctorRequest, DoctorResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { doctorCreate, doctorUpdate } from "../libs/doctors";

interface DoctorMethods {
    create: (req: DoctorRequest) => Promise<DoctorResponse>;
    update: (id: Principal, req: DoctorRequest) => Promise<DoctorResponse>;
}

export const useDoctor = (
): DoctorMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: DoctorRequest}) => {
            return doctorCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['doctors']);
            }   
        }
    );

    const create = (
        req: DoctorRequest
    ): Promise<DoctorResponse> => {
        return createMut.mutateAsync({req});
    };

    const updateMut = useMutation(
        async (options: {id: Principal, req: DoctorRequest}) => {
            return doctorUpdate(main, options.id, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['doctors']);
            }   
        }
    );

    const update = (
        id: Principal,
        req: DoctorRequest
    ): Promise<DoctorResponse> => {
        return updateMut.mutateAsync({id, req});
    };
    
    return {
        create,
        update,
    }
};
