import { useContext, useEffect } from "react";
import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, UserResponse } from "../../../declarations/main/main.did";
import { canisterId as mainCanisterId } from "../../../declarations/main";
import { ICProvider, ICProviderState, ICProviderType } from "../interfaces/icprovider";
import { ActorActionType, ActorContext } from "../stores/actor";
import { AuthActionType, AuthContext } from "../stores/auth";
import { Result } from "../interfaces/result";
import { IcProviderBuider } from "../libs/icproviderbuilder";
import { accountIdentifierFromBytes, principalToAccountDefaultIdentifier } from "../libs/icp";
import { userFindMe } from "../libs/users";

interface AuthResponse {
    isAuthenticated: boolean;
    isLogged: boolean;
    user?: UserResponse;
    principal?: Principal;
    accountId?: string,
    login: (providerType: ICProviderType) => Promise<Result<UserResponse, string>>;
    logout: () => Promise<void>;
    update: (user: UserResponse) => void;
}

const locks = {
    initialize: false,
    connect: false,
    configure: false,
};

export const useAuth = (
): AuthResponse => {
    const [auth, authDisp] = useContext(AuthContext);
    const [, actorsDisp] = useContext(ActorContext);

    const _createActors = async (
        provider: ICProvider
    ): Promise<Main> => {
        const actors = await Promise.all([
            provider.createActor(mainCanisterId),
        ]);
        actorsDisp({
            type: ActorActionType.SET_MAIN,
            payload: actors[0]
        });

        return actors[0];
    };

    const _destroyActors = async (
    ) => {
        actorsDisp({
            type: ActorActionType.RESET_MAIN,
            payload: undefined
        });
    }

    const _loadUser = async (
        provider: ICProvider,
        main: Main
    ): Promise<Result<UserResponse, string>> => {
        if(!await provider.isAuthenticated()) {
            _destroyUser();
            return {Err: "User not authenticated"};
        }

        const principal = provider.getPrincipal();
        authDisp({
            type: AuthActionType.SET_PRINCIPAL,
            payload: principal
        });
        authDisp({
            type: AuthActionType.SET_ACCOUNT_ID,
            payload: principal?
                accountIdentifierFromBytes(
                    principalToAccountDefaultIdentifier(principal)):
                undefined
        });
        
        const res = await _loadAuthenticatedUser(main);
        if('Err' in res) {
            return res;
        }

        const user = res.Ok;
        
        authDisp({
            type: AuthActionType.SET_USER,
            payload: main? 
                user: 
                undefined
        });

        return {Ok: user};
    };

    const _destroyUser = () => {
        authDisp({
            type: AuthActionType.SET_PRINCIPAL,
            payload: undefined
        });
        authDisp({
            type: AuthActionType.SET_ACCOUNT_ID,
            payload: undefined
        });
        authDisp({
            type: AuthActionType.SET_USER,
            payload: undefined
        });
    };

    const _storeProvider = (
        provider: ICProvider,
        providerType: ICProviderType
    ) => {
        window.localStorage.setItem('providerType', ICProviderType[providerType]);

        authDisp({
            type: AuthActionType.SET_PROVIDER,
            payload: provider
        });
    };

    const _destroyProvider = () => {
        window.localStorage.removeItem('providerType');
        authDisp({
            type: AuthActionType.SET_PROVIDER,
            payload: undefined
        });
    };
        
    const _initialize = async (
        provider: ICProvider,
        updateState: boolean 
    ): Promise<boolean> => {
        if(locks.initialize) {
            return false;
        }

        locks.initialize = true;

        try {
            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Initializing
                });
            }

            if(!await provider.initialize()) {
                if(updateState) {
                    authDisp({
                        type: AuthActionType.SET_STATE,
                        payload: ICProviderState.Disconnected
                    });
                }
                return  false;
            }

            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Initialized
                });
            }
        }
        finally {
            locks.initialize = false;
        }

        return true;
    };

    const _connect = async (
        provider: ICProvider,
        updateState: boolean
    ): Promise<boolean> => {
        if(locks.connect) {
            return false;
        }

        locks.connect = true;

        try {
            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Connecting
                });
            }

            const res = await provider.connect();
            if(res.Err) {
                if(updateState) {
                    authDisp({
                        type: AuthActionType.SET_STATE,
                        payload: ICProviderState.Disconnected
                    });
                }
                return false;
            }

            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Connected
                });
            }
        }
        finally {
            locks.connect = false;
        }

        return true;
    };

    const _configure = async (
        provider: ICProvider,
        updateState: boolean
    ): Promise<Result<UserResponse, string>> => {
        if(locks.configure) {
            return {Err: 'The configure lock stills held'};
        }

        locks.configure = true;

        try {
            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Configuring
                });
            }

            const main = await _createActors(provider);
            const res = await _loadUser(provider, main);
            if('Err' in res) {
                return res;
            }

            const user = res.Ok;

            if(updateState) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Configured
                });
            }

            return {Ok: user};
        }
        finally {
            locks.configure = false;
        }
    };

    const login = async (
        providerType: ICProviderType
    ): Promise<Result<UserResponse, string>> => {
        
        //
        let provider: ICProvider | undefined = new IcProviderBuider().build(providerType);
        if(!provider) {
            return {Err: "Unknown provider"};
        }

        authDisp({
            type: AuthActionType.SET_STATE,
            payload: ICProviderState.Disconnected
        });

        // wait provider to initialize
        if(!await _initialize(provider, false)) {
            return {Err: "IC Provider initialization failed"};
        }

        // wait provider to connect
        if(!await _connect(provider, false)) {
            return {Err: "IC Provider connection failed"};
        }

        // do the logon
        const res = await _configure(provider, false)
        if('Err' in res) {
            return res;
        }

        _storeProvider(provider, providerType);

        authDisp({
            type: AuthActionType.SET_STATE,
            payload: ICProviderState.Configured
        });

        return {Ok: res.Ok};
    };

    const logout = async () => {
        await auth.provider?.logout();
        _destroyUser();
        _destroyActors();
        _destroyProvider();
        authDisp({
            type: AuthActionType.SET_STATE,
            payload: ICProviderState.Idle
        });
    };

    const update = (
        user: UserResponse
    ) => {
        authDisp({
            type: AuthActionType.SET_USER,
            payload: user
        });
    };

    useEffect(() => {
        if(!auth.provider) {
            return;
        }

        switch(auth.state) {
            case ICProviderState.Idle:
                _initialize(auth.provider, true);
                return;

            case ICProviderState.Initialized:
                _connect(auth.provider, true);
                return;

            case ICProviderState.Connected:
                _configure(auth.provider, true);
                return;
        }
        
    }, [auth.provider, auth.state]);

    return {
        isAuthenticated: auth.principal !== undefined,
        isLogged: auth.user !== undefined,
        user: auth.user,
        principal: auth.principal,
        accountId: auth.accountId,
        login,
        logout,
        update,
    }
};

const _loadAuthenticatedUser = async (
    main: Main
): Promise<Result<UserResponse, string>> => {
    try {
        return {Ok: await userFindMe(main)};
    }
    catch(e) {
        return {Err: (e as Error).message};
    }
};