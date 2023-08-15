import { _SERVICE as Main, UserResponse } from "../../../declarations/main/main.did";

const anonymous: UserResponse = {
    active: false,
    kind: {
        patient: {
            id: "anonymous",
            name: "Anonymous",
            birth_date: 0n
        }
    },
    banned: false
};

export const findMe = async (
    main?: Main
): Promise<UserResponse> => {
    if(!main) {
        return anonymous;
    }

    const res = await main.user_find_me();
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};