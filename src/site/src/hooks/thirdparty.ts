import { useMutation, useQueryClient } from "react-query";
import { ThirdPartyRequest, ThirdPartyResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { thirdPartyCreate, thirdPartyDelete, thirdPartyUpdate } from "../libs/thirdparties";
import { Principal } from "@dfinity/principal";

interface ThirdPartyMethods {
    create: (req: ThirdPartyRequest) => Promise<ThirdPartyResponse>;
    update: (id: Principal, req: ThirdPartyRequest) => Promise<ThirdPartyResponse>;
    remove: (id: Principal) => Promise<void>;
}

export const useThirdParty = (
): ThirdPartyMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: ThirdPartyRequest}) => {
            return thirdPartyCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['thirdparties']);
            }   
        }
    );

    const create = (
        req: ThirdPartyRequest
    ): Promise<ThirdPartyResponse> => {
        return createMut.mutateAsync({req});
    };

    const updateMut = useMutation(
        async (options: {id: Principal, req: ThirdPartyRequest}) => {
            return thirdPartyUpdate(main, options.id, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['thirdparties']);
            }   
        }
    );

    const update = (
        id: Principal,
        req: ThirdPartyRequest
    ): Promise<ThirdPartyResponse> => {
        return updateMut.mutateAsync({id, req});
    };

    const deleteMut = useMutation(
        async (options: {id: Principal}) => {
            return thirdPartyDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['thirdparties']);
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