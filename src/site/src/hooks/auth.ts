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
    login: (providerType: ICProviderType) => Promise<Result<any, string>>;
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
        main?: Main
    ) => {
        if(!await provider.isAuthenticated()) {
            _destroyUser();
            return;
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
        authDisp({
            type: AuthActionType.SET_USER,
            payload: main? 
                await _loadAuthenticatedUser(main): 
                undefined
        });
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
        provider: ICProvider
    ) => {
        if(locks.initialize) {
            return;
        }

        locks.initialize = true;

        try {
            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Initializing
            });

            if(!await provider.initialize()) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Disconnected
                });
                return;
            }

            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Initialized
            });
        }
        finally {
            locks.initialize = false;
        }
    };

    const _connect = async (
        provider: ICProvider
    ) => {
        if(locks.connect) {
            return;
        }

        locks.connect = true;

        try {
            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Connecting
            });

            const res = await provider.connect();
            if(res.Err) {
                authDisp({
                    type: AuthActionType.SET_STATE,
                    payload: ICProviderState.Disconnected
                });
                return;
            }

            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Connected
            });
        }
        finally {
            locks.connect = false;
        }
    };

    const _configure = async (
        provider: ICProvider
    ) => {
        if(locks.configure) {
            return;
        }

        locks.configure = true;

        try {
            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Configuring
            });

            const main = await _createActors(provider);
            await _loadUser(provider, main);

            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Configured
            });
        }
        finally {
            locks.configure = false;
        }
    };

    const login = async (
        providerType: ICProviderType
    ): Promise<Result<any, string>> => {
        
        //
        let provider: ICProvider | undefined = new IcProviderBuider().build(providerType);
        if(!provider) {
            return {Err: "Unknown provider"};
        }

        // wait provider to initialize
        authDisp({
            type: AuthActionType.SET_STATE,
            payload: ICProviderState.Initializing
        });

        if(!await provider.initialize()) {
            authDisp({
                type: AuthActionType.SET_STATE,
                payload: ICProviderState.Disconnected
            });

            return {Err: "IC Provider initialization failed"};
        }

        // do the logon
        const res = await provider?.login();
        if(res.Err) {
            return res;
        }

        _storeProvider(provider, providerType);

        authDisp({
            type: AuthActionType.SET_STATE,
            payload: ICProviderState.Initialized
        });

        return {Ok: null};
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
                _initialize(auth.provider);
                return;

            case ICProviderState.Initialized:
                _connect(auth.provider);
                return;

            case ICProviderState.Connected:
                _configure(auth.provider);
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
): Promise<UserResponse|undefined> => {
    try {
        return await userFindMe(main);
    }
    catch(e) {
        //console.log(e)
    }

    return undefined;
};