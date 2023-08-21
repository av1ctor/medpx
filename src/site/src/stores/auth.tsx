import { Principal } from "@dfinity/principal";
import React, {createContext, useReducer} from "react";
import { ICProvider, ICProviderState } from "../interfaces/icprovider";
import { UserResponse } from "../../../declarations/main/main.did";
import { AES_GCM } from "../libs/vetkd";

export interface AuthState {
    state: ICProviderState;
    provider?: ICProvider;
    principal?: Principal;
    accountId?: string;
    aes_gcm?: AES_GCM;
    user?: UserResponse;
};

export enum AuthActionType {
    SET_STATE,
    SET_PROVIDER,
    SET_PRINCIPAL,
    SET_ACCOUNT_ID,
    SET_USER,
    SET_AES_GCM,
};

export interface AuthAction {
    type: AuthActionType;
    payload: any;
}

const initialState: AuthState = {
    state: ICProviderState.Idle,
    provider: undefined,
    principal: undefined,
    accountId: undefined,
    user: undefined,
    aes_gcm: undefined,
};

export const AuthContext = createContext<[AuthState, (action: AuthAction) => void]>(
    [initialState, (action: AuthAction) => {}]);

const reducer = (state: AuthState, action: AuthAction): AuthState => {
    switch(action.type) {
        case AuthActionType.SET_STATE:
            return {
                ...state,
                state: action.payload
            };

        case AuthActionType.SET_PROVIDER:
            return {
                ...state,
                provider: action.payload
            };
        
        case AuthActionType.SET_PRINCIPAL:
            return {
                ...state,
                principal: action.payload
            };

        case AuthActionType.SET_ACCOUNT_ID:
            return {
                ...state,
                accountId: action.payload
            };

        case AuthActionType.SET_USER:
            return {
                ...state,
                user: action.payload
            };

        case AuthActionType.SET_AES_GCM:
            return {
                ...state,
                aes_gcm: action.payload
            };

        default:
            return state;
    }
};

interface Props {
    provider: ICProvider | undefined;
    children: any
};

export const AuthContextProvider = (props: Props) => {
    const [state, dispatch] = useReducer(
        reducer, {
            ...initialState, 
            provider: props.provider,
    });

    return (
        <AuthContext.Provider
            value={[state, dispatch]}>
            {props.children}
        </AuthContext.Provider>
    );
};


