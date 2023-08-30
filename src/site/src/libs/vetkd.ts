import * as vetkd from "ic-vetkd-utils";
import { Principal } from "@dfinity/principal";
import { _SERVICE as Main, PrescriptionResponse } from "../../../declarations/main/main.did";
import { Result } from "../interfaces/result";

export class AES_GCM {
    main: Main;
    pk_bytes: Uint8Array = Uint8Array.from([]);

    constructor(main: Main) {
        this.main = main;
    }

    async init(
    ): Promise<Result<null, string>> {
        await vetkd.default();
        
        const res = await this.main.prescription_get_public_key();
        if('Err' in res) {
            return res;
        }

        this.pk_bytes = hex_decode(res.Ok);

        return {Ok: null};
    }

    async genRawKey(
        prescription: PrescriptionResponse
    ): Promise<Result<Uint8Array, string>> {
        
        const seed = window.crypto.getRandomValues(new Uint8Array(32));
        const tsk = new vetkd.TransportSecretKey(seed);
        const res = await this.main.prescription_get_encrypted_symmetric_key(
            prescription.id,
            tsk.public_key()
        );
        if('Err' in res) {
            return res;
        }

        try {
            const ek_bytes = hex_decode(res.Ok);

            const rawKey = tsk.decrypt_and_hash(
                ek_bytes,
                this.pk_bytes,
                prescription.hash as Uint8Array,
                32,
                new TextEncoder().encode("aes-256-gcm")
            );

            return {Ok: rawKey};
        }
        catch(e: any) {
            return {Err: e.message};
        }
    }

    async encrypt (
        message: string,
        rawKey: Uint8Array
    ): Promise<Uint8Array> {
        const iv = window.crypto.getRandomValues(new Uint8Array(12));
        try {
            const aes_key = await window.crypto.subtle.importKey("raw", rawKey, "AES-GCM", false, ["encrypt"]);
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
        iv_and_ciphertext: Uint8Array,
        rawKey: Uint8Array
    ): Promise<string> {
        const iv = iv_and_ciphertext.subarray(0, 12);
        const ciphertext = iv_and_ciphertext.subarray(12);
        
        try {
            const aes_key = await window.crypto.subtle.importKey("raw", rawKey, "AES-GCM", false, ["decrypt"]);
            
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
