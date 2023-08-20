import {StoicIdentity} from "ic-stoic-identity";
import { ActorSubclass } from "@dfinity/agent";
import { createActor as mainCreateActor, canisterId as mainCanisterId } from "../../../declarations/main";
import { ICProvider } from "../interfaces/icprovider";
import { Result } from "../interfaces/result";
import { Principal } from "@dfinity/principal";
import { buildAgentJsOptions, transferErrorToText } from "../libs/icp";

class StoicProvider implements ICProvider {
    identity?: StoicIdentity;
    
    public async initialize(
    ): Promise<boolean> {
        return true;
    }

    public async connect(
        options?: any
    ): Promise<Result<any, string>> {
        try {
            this.identity = await StoicIdentity.load();
        }
        catch(e: any) {
            return {Err: e.toString()};
        }

        return {Ok: null}
    } 

    public async isAuthenticated(
    ): Promise<boolean> {
        return !!this.identity;
    }

    public async createActor(
        id?: string
    ): Promise<any> {
        switch(id) {
            case mainCanisterId:
                return this._createMainActor();
            default:
                return undefined;
        }
    }

    public getPrincipal(
    ): Principal | undefined {
        return this.identity?.getPrincipal();
    }

    public async login(
    ): Promise<Result<any, string>> {
        try {
            this.identity = await StoicIdentity.connect();
            if(!this.identity) {
                return {Err: 'Login failed, no identity return'};
            }
        }
        catch(e: any) {
            return {Err: e.toString()};
        }

        return {Ok: null};
    }

    public async transferICP(
        to: Array<number>,
        amount: bigint,
        memo: bigint,
    ): Promise<Result<bigint, string>> {
        return {Err: 'Ledger undefined'};
    }

    public async logout(
    ): Promise<void> {
        StoicIdentity.disconnect();
    }

    private _createMainActor(
    ): ActorSubclass<any> {
        if(!mainCanisterId) {
            throw Error('Main canister is undefined');
        }
    
        return mainCreateActor(mainCanisterId, {agentOptions: buildAgentJsOptions(this.identity)})
    }
};


export default StoicProvider;