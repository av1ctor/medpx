import React, { useCallback, useMemo, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Grid, Space, Textarea, Stepper } from "@mantine/core";
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
    const {create, update} = usePrescription();
    const [patient, setPatient] = useState<UserResponse|undefined>();
    const [active, setActive] = useState(0);
    const [hash, setHash] = useState<Uint8Array>(new Uint8Array());
    const [signature, setSignature] = useState<Uint8Array|undefined>();
    const [cert, setCert] = useState<string|undefined>();
    
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
            hash: hash,
            signature: signature || [],
            cert: '',
            contents: new TextEncoder().encode(form.values.contents),
        };
    }, [form.values.contents, principal, patient, hash, signature]);

    const handleSigned = useCallback((cert: string, signature: Uint8Array) => {
        setCert(cert);
        setSignature(signature);
    }, []);

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            if(!aes_gcm) {
                throw Error("AES-GCM undefined");
            }

            if(!hash) {
                throw Error("Content's hash undefined");
            }

            if(!cert) {
                throw Error("Certificate undefined");
            }

            if(!signature) {
                throw Error("Signature undefined");
            }

            const principal = userGetPrincipal(patient);
            
            const prescription = await create({
                patient: principal,
                contents: [],
                hash,
                signature,
                cert,
            });
            
            const rawKey = await aes_gcm.genRawKey(prescription);
            if('Err' in rawKey || !rawKey.Ok) {
                throw new Error(`Raw key generation failed: ${rawKey.Err}`);
            }
            
            const contents = await aes_gcm.encrypt(values.contents, rawKey.Ok);

            await update(
                prescription.id,
                {
                    patient: principal,
                    contents: [contents],
                    hash,
                    signature,
                    cert,
                }
            );

            props.onSuccess('Prescription created!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, aes_gcm, patient, hash, signature]);

    const calcHash = useCallback(async () => {
        const hash = new Uint8Array(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(form.values.contents)));
        setHash(hash);
        setSignature(undefined);
    }, [form.values.contents]);

    const handlePrev = useCallback(() => { 
        setActive(active => active - 1);
    }, []);

    const handleNext = useCallback(() => { 
        if(active === 1) {
            calcHash();
        }
        setActive(active + 1);
    }, [active, calcHash]);

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
                        allowStepSelect={!!patient}
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
                        label="Signature" 
                        description="Sign the prescription"
                        allowStepSelect={form.values.contents.length >= 16}
                    >
                        <div className="card">
                            {!signature?
                                <CertSigner 
                                    hash={hash}
                                    onSuccess={handleSigned}
                                />
                            :
                                <PrescriptionView 
                                    item={mockPrescription}
                                    isEncrypted={false}
                                />
                            }
                        </div>

                        <Space h="lg"/>

                        <form onSubmit={form.onSubmit(handleCreate)}>
                            <Button
                                color="red"
                                fullWidth
                                disabled={!signature}
                                type="submit"
                            >
                                Submit
                            </Button>
                        </form>
                    </Stepper.Step>
                </Stepper>
            </Container>
        </>
    );
};

export default PrescriptionCreate;