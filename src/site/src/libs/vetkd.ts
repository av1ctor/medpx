import * as vetkd from "ic-vetkd-utils";
import { Principal } from "@dfinity/principal";
import { _SERVICE as Main } from "../../../declarations/main/main.did";
import { canisterId } from "../../../declarations/main";
import { Result } from "../interfaces/result";

export class AES_GCM {
    rawKey: Uint8Array = new Uint8Array([]);

    async init(
        main: Main,
        derivationPath: string
    ): Promise<Result<null, string>> {
        await vetkd.default();
        const seed = window.crypto.getRandomValues(new Uint8Array(32));
        const tsk = new vetkd.TransportSecretKey(seed);
        
        const derivation = new TextEncoder().encode(derivationPath);
        
        return await Promise.all([
            main.user_get_encrypted_symmetric_key(derivation, tsk.public_key()), 
            main.user_get_public_key(derivation)
        ]).then(
            (res) => {
                if('Err' in res[0]) {
                    return res[0];
                }
                if('Err' in res[1]) {
                    return res[1];
                }

                const ek_bytes_hex = 'Ok' in res[0] && res[0].Ok;
                const pk_bytes_hex = 'Ok' in res[1] && res[1].Ok;

                this.rawKey = tsk.decrypt_and_hash(
                    hex_decode(ek_bytes_hex || ''),
                    hex_decode(pk_bytes_hex || ''),
                    Principal.fromText(canisterId).toUint8Array(),
                    32,
                    new TextEncoder().encode("aes-256-gcm")
                );

                return {Ok: null};
            },
            (reason: any) => {
                return {Err: reason}
            }
        )
    }

    async encrypt (
        message: string
    ): Promise<Uint8Array> {
        const iv = window.crypto.getRandomValues(new Uint8Array(12));
        try {
            const aes_key = await window.crypto.subtle.importKey("raw", this.rawKey, "AES-GCM", false, ["encrypt"]);
            try {
                const message_encoded = new TextEncoder().encode(message);
                const ciphertext_buffer = await window.crypto.subtle.encrypt(
                    { name: "AES-GCM", iv: iv },
                    aes_key,
                    message_encoded
                );
                const ciphertext = new Uint8Array(ciphertext_buffer);
                var iv_and_ciphertext = new Uint8Array(iv.length + ciphertext.length);
                iv_and_ciphertext.set(iv, 0);
                iv_and_ciphertext.set(ciphertext, iv.length);
                return iv_and_ciphertext;
            }
            catch(e: any) {
                throw new Error( `Error at window.crypto.subtle.encrypt: "${e.message}"`);    
            }
        }
        catch(e: any) {
            throw new Error( `Error at window.crypto.subtle.importKey: "${e.message}"`);
        }
    }

    async decrypt (
        iv_and_ciphertext: Uint8Array
    ): Promise<string> {
        const iv = iv_and_ciphertext.subarray(0, 12);
        const ciphertext = iv_and_ciphertext.subarray(12);
        
        try {
            const aes_key = await window.crypto.subtle.importKey("raw", this.rawKey, "AES-GCM", false, ["decrypt"]);
            
            try {
                let decrypted = await window.crypto.subtle.decrypt(
                    { name: "AES-GCM", iv: iv },
                    aes_key,
                    ciphertext
                );
                return new TextDecoder().decode(decrypted);
            }
            catch(e: any) {
                return `Error at window.crypto.subtle.decrypt: "${e.message}"`;
            }
        }
        catch(e: any) {
            return `Error at window.crypto.subtle.importKey: "${e.message}"`;
        }
    }
}

const hex_decode = (
    hex: string
): Uint8Array => {
    const match = hex.match(/.{1,2}/g);
    if(!match) {
        return new Uint8Array([]);
    }
    
    return Uint8Array.from(match
        .map((byte) => parseInt(byte, 16))
    );
};
