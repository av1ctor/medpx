import React, {createContext, useReducer} from "react";
import { _SERVICE as Main } from "../../../declarations/main/main.did";
import { createActor as mainCreateActor, canisterId as mainCanisterId } from "../../../declarations/main";
import { config } from "../config";

export interface ActorState {
    main: Main;
};

export enum ActorActionType {
    SET_MAIN,
    RESET_MAIN,
};

export interface ActorAction {
    type: ActorActionType;
    payload: any;
};

const options = {agentOptions: {host: config.IC_GATEWAY}};

const initialState: ActorState = {
    main: mainCreateActor(mainCanisterId, options),
};

export const ActorContext = createContext<[ActorState, (action: ActorAction) => void]>([
    initialState, (action: ActorAction) => {}
]);

const reducer = (state: ActorState, action: ActorAction): ActorState => {
    switch(action.type) {
        case ActorActionType.SET_MAIN:
            return {
                ...state,
                main: action.payload
            };

        case ActorActionType.RESET_MAIN:
            return {
                ...state,
                main: mainCreateActor(mainCanisterId, options)
            };
    
        default:
            return state;
    }
};

interface Props {
    children: any
};

export const ActorContextProvider = (props: Props) => {
    const [state, dispatch] = useReducer(reducer, initialState);

    return (
        <ActorContext.Provider
            value={[state, dispatch]}>
            {props.children}
        </ActorContext.Provider>
    );
};