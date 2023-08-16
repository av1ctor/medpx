import { _SERVICE as Main, UserResponse } from "../../../declarations/main/main.did";

export const findMe = async (
    main: Main
): Promise<UserResponse> => {
    const res = await main.user_find_me();
    if('Err' in res) {
        throw new Error(res.Err);
    }
    return res.Ok; 
};