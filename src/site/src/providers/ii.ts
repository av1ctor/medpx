import { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { createActor as mainCreateActor, canisterId as mainCanisterId } from "../../../declarations/main";
import { canisterId as siteCanisterId } from "../../../declarations/site";
import { ICProvider } from "../interfaces/icprovider";
import { Result } from "../interfaces/result";
import { config } from "../config";
import { Principal } from "@dfinity/principal";
import { buildAgentJsOptions } from "../libs/icp";

class InternetIdentityProvider implements ICProvider {
    client?: AuthClient;
    identity?: Identity;
    
    public async initialize(
    ): Promise<boolean> {
        this.client = await AuthClient.create({idleOptions: {disableIdle: true}});
        return this.client !== undefined;
    }

    public async connect(
        options?: any
    ): Promise<Result<any, string>> {
        try {
            this.identity = this.client?.getIdentity();
            if(!this.identity) {
                return {Err: 'IC Identity should not be null'};
            }
        }
        catch(e: any) {
            return {Err: e.toString()};
        }

        return {Ok: null}
    } 

    public async isAuthenticated(
    ): Promise<boolean> {
        return await this.client?.isAuthenticated() || false;
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
        const width = 500;
        const height = screen.height;
        const left = ((screen.width/2)-(width/2))|0;
        const top = ((screen.height/2)-(height/2))|0; 
        
        return new Promise((resolve) => {
            this.client?.login({
                identityProvider: config.II_URL,
                derivationOrigin: config.isProduction?
                     `https://${siteCanisterId}.ic0.app`: 
                     undefined,
                maxTimeToLive: BigInt(7 * 24) * BigInt(3_600_000_000_000), // 1 week
                windowOpenerFeatures: `toolbar=0,location=0,menubar=0,width=${width},height=${height},top=${top},left=${left}`,
                onSuccess: () => {
                    resolve({Ok: null});
                },
                onError: (msg: string|undefined) => {
                    resolve({Err: msg});
                 }
            });
        });
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
        await this.client?.logout();
    }

    private _createMainActor(
    ): ActorSubclass<any> {
        if(!mainCanisterId) {
            throw Error('Main canister is undefined');
        }
    
        return mainCreateActor(mainCanisterId, {agentOptions: buildAgentJsOptions(this.identity)})
    }
};


export default InternetIdentityProvider;