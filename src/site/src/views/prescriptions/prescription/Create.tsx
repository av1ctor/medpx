import React, { useCallback, useEffect, useMemo, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Grid, Space, Textarea, Text, Stepper } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { usePrescription } from "../../../hooks/prescriptions";
import { PrescriptionResponse, UserResponse } from "../../../../../declarations/main/main.did";
import { userGetPrincipal } from "../../../libs/users";
import PrescriptionView from "./View";
import { useAuth } from "../../../hooks/auth";
import { UserLookup } from "../../users/user/Lookup";
import { UserAvatar } from "../../../components/UserAvatar";
import CertSigner from "../../../components/Certificates/CertSigner";
import { Principal } from "@dfinity/principal";

const schema = yup.object().shape({
    contents: yup.string().min(16).max(4096),
});

interface StepsControlProps {
    active: number;
    disabled?: boolean;
    onPrev: () => void;
    onNext: () => void;
}

const StepsControl = (props: StepsControlProps) => {
    return (
        <Grid>
            <Grid.Col md={6} sm={12}>
                <Button
                    color="green"
                    fullWidth
                    disabled={props.active === 0}
                    onClick={props.onPrev}
                >
                    Previous
                </Button>
            </Grid.Col>
            <Grid.Col md={6} sm={12}>
                <Button
                    color="blue"
                    fullWidth
                    disabled={props.disabled}
                    onClick={props.onNext}
                >
                    Next
                </Button>
            </Grid.Col>
        </Grid>
    );
};

interface Props {
    onSuccess: (msg: string) => void;
}

const PrescriptionCreate = (props: Props) => {
    const {principal, aes_gcm} = useAuth();
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {preCreate, postCreate} = usePrescription();
    const [patient, setPatient] = useState<UserResponse|undefined>();
    const [active, setActive] = useState(0);
    const [prescription, setPrescription] = useState<PrescriptionResponse|undefined>();
    const [cipherTextHash, setCipherTextHash] = useState<Uint8Array|undefined>();
    const [cipherText, setCipherText] = useState<Uint8Array|undefined>();
    
    const form = useForm({
        initialValues: {
            contents: '',
        },
    
        validate: yupResolver(schema),
    });

    const mockPrescription = useMemo((
    ): PrescriptionResponse => {
        return  {
            id: 'temp',
            created_at: BigInt(Date.now()) * 1000000n,
            doctor: principal || Principal.anonymous(),
            patient: userGetPrincipal(patient),
            plain_text_hash: [],
            cipher_text: new TextEncoder().encode(form.values.contents),
        };
    }, [form.values.contents, principal, patient]);

    const handlePreCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            const plainTextHash = new Uint8Array(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(values.contents)));

            const principal = userGetPrincipal(patient);
            
            const prescription = await preCreate({
                patient: principal,
                plain_text_hash: plainTextHash,
            });

            setPrescription(prescription);
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, aes_gcm, patient]);

    const handleSigned = useCallback(async (cert: string, signature: Uint8Array) => {
        try {
            if(!prescription) {
                throw Error("Prescription is undefined");
            }

            if(!cipherText) {
                throw Error("Cipher text is undefined");
            }

            if(!cipherTextHash) {
                throw Error("Cipher text hash is undefined");
            }
            
            await postCreate(
                prescription.id,
                {
                    cipher_text: cipherText,
                    cipher_text_hash: cipherTextHash,
                    signature: signature,
                    cert: cert,
                }
            );

            props.onSuccess('Prescription created!');
        }
        catch(e: any) {
            showError(e);
        }
    }, [main, prescription, cipherTextHash, cipherText]);

    const encrypt = useCallback(async () =>  {
        if(!prescription || !!cipherText) {
            return;
        }

        if(!aes_gcm) {
            throw Error("AES-GCM undefined");
        }
        
        const rawKey = await aes_gcm.genRawKey(prescription);
        if('Err' in rawKey) {
            showError(`Raw key generation failed: ${rawKey.Err}`);
        }
        
        const _cipherText = await aes_gcm.encrypt(form.values.contents, rawKey.Ok as any);
        const _cipherTextHash = await crypto.subtle.digest("SHA-256", _cipherText);

        setCipherText(_cipherText);
        setCipherTextHash(new Uint8Array(_cipherTextHash));

        setActive(3);
    }, [aes_gcm, prescription, form.values.contents, cipherText]);

    useEffect(() => {
        encrypt();
    }, [encrypt]);

    const handlePrev = useCallback(() => { 
        setActive(active => active - 1);
    }, []);

    const handleNext = useCallback(() => { 
        setActive(active + 1);
    }, [active]);

    return (
        <>
            <Container>
                <Stepper 
                    active={active} 
                    onStepClick={setActive}
                    breakpoint="sm"
                >
                    <Stepper.Step 
                        label="Patient" 
                        description="Choose a patient"
                    >
                        <UserLookup 
                            user={patient}
                            onChange={setPatient}
                        />

                        <Space h="xl" />

                        <StepsControl
                            active={active}
                            disabled={!patient}
                            onPrev={handlePrev}
                            onNext={handleNext}
                        />
                    </Stepper.Step>

                    <Stepper.Step 
                        label="Contents" 
                        description="Compose the prescription"
                        allowStepSelect={false}
                    >
                        <UserAvatar user={patient} />

                        <Space h="1rem" />

                        <Textarea
                            label="Contents"
                            placeholder="Contents"
                            minRows={20}
                            {...form.getInputProps('contents')}
                        />

                        <Space h="xl" />

                        <StepsControl
                            active={active}
                            disabled={form.values.contents.length < 16}
                            onPrev={handlePrev}
                            onNext={handleNext}
                        />
                    </Stepper.Step>
                    <Stepper.Step 
                        label="Preview" 
                        description="Preview the prescription"
                        allowStepSelect={false}
                    >
                        <div className="card">
                            <PrescriptionView 
                                item={mockPrescription}
                                isEncrypted={false}
                            />
                        </div>

                        <Space h="lg"/>

                        <Grid>
                            <Grid.Col md={6} sm={12}>
                                <Button
                                    color="green"
                                    fullWidth
                                    onClick={handlePrev}
                                >
                                    Previous
                                </Button>
                            </Grid.Col>
                            <Grid.Col md={6} sm={12}>
                                <form onSubmit={form.onSubmit(handlePreCreate)}>
                                    <Button
                                        color="blue"
                                        fullWidth
                                        type="submit"
                                    >
                                        Next
                                    </Button>
                                </form>
                            </Grid.Col>
                        </Grid>
                    </Stepper.Step>
                    <Stepper.Step 
                        label="Sign" 
                        description="Sign the prescription"
                        allowStepSelect={false}
                    >
                        {cipherTextHash?
                            <CertSigner 
                                hash={cipherTextHash}
                                onSuccess={handleSigned}
                            />
                        :
                            <Text>
                                Encrypting, please wait...
                            </Text>
                        }
                    </Stepper.Step>
                </Stepper>
            </Container>
        </>
    );
};

export default PrescriptionCreate;