import { ActorSubclass, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as mainIdlFactory, canisterId as mainCanisterId } from "../../../declarations/main";
import { ICProvider } from "../interfaces/icprovider";
import { config } from "../config";
import { Principal } from "@dfinity/principal";
import { IDL } from "@dfinity/candid";
import { Result } from "../interfaces/result";
import { transferErrorToText } from "../libs/icp";

type RequestConnectOptions = {
    whitelist: Array<string>;
    host?: string;
    onConnectionUpdate?: () => void;
};

type Plug = {
    createActor: <T>(args: {
        canisterId: string, 
        interfaceFactory: IDL.InterfaceFactory
    }) => Promise<ActorSubclass<T>>;
    agent: Agent;
    createAgent: (options: {
        host: string, 
        whitelist: Array<string>
    }) => Promise<Agent>;
    getPrincipal: () => Promise<Principal>;
    isConnected: () => Promise<boolean>;
    disconnect: () => Promise<void>;
    requestConnect: (options: RequestConnectOptions) => Promise<string>;
    accountId: string;
    sessionManager: {
        sessionData: { 
            agent: HttpAgent, 
            principalId: string, 
            accountId: string 
        } | null;
    };
    requestTransfer: (args: {
        to: string,
        amount: number,
        opts?: {
            fee?: number,
            memo?: string,
            from_subaccount?: number,
            created_at_time?: {
                timestamp_nanos: number
            },
        },
    }) => Promise<{
        height: number
    }>;
    requestBalance: () => Promise<Array<{
        amount: number
        canisterId: string
        decimals: number
        image?: string
        name: string
        symbol: string
    }>>;
    getManagementCanister: () => Promise<ActorSubclass | undefined>;
  }

class PlugProvider implements ICProvider {
    plug?: Plug;
    config: RequestConnectOptions;

    constructor() {
        this.plug = window.ic?.plug;
        
        const whitelist: string[] = [mainCanisterId];
        
        this.config = {
            whitelist,
            host: config.IC_URL
        };
    }
    
    public async initialize(
    ): Promise<boolean> {
        if(!this.plug) {
            return false;
        }

        return true;
    }

    public async connect(
        options?: any
    ): Promise<Result<any, string>> {
        return {Ok: null};
    } 

    public async isAuthenticated(
    ): Promise<boolean> {
        return await this.plug?.isConnected() || false;
    }

    public async createActor(
        id?: string
    ): Promise<any> {
        if(!config.isProduction) {
            if(this.plug?.agent) {
                await this.plug?.agent.fetchRootKey();
            }
        }

        switch(id) {
            case mainCanisterId:
                return await this._createMainActor();
            default:
                return undefined;
        }
    }

    public getPrincipal(
    ): Principal | undefined {
        const data = this.plug?.sessionManager.sessionData;
        return data? Principal.fromText(data.principalId): undefined;
    }

    public async login(
    ): Promise<Result<any, string>> {
        if(await this.plug?.isConnected()) {
            return {Ok: null};
        }
        
        try {
            await this.plug?.requestConnect(this.config);

            return {Ok: null};
            
        } catch (e: any) {
            return {Err: e.toString()};
        }
    }

    public async transferICP(
        to: Array<number>,
        amount: bigint,
        memo: bigint,
    ): Promise<Result<bigint, string>> {
        //FIXME: it's not possible atm to pass a subaccount to plug.requestTransfer()
        
        return {Err: 'Ledger undefined'};
    }

    public async logout(
    ): Promise<void> {
        this.plug?.disconnect();
    }

    private async _createMainActor(
    ): Promise<ActorSubclass<any>> {
        if(!mainCanisterId) {
            throw Error('Main canister is undefined');
        }

        return await this.plug?.createActor({
            canisterId: mainCanisterId,
            interfaceFactory: mainIdlFactory,
        });
    }
};


export default PlugProvider;