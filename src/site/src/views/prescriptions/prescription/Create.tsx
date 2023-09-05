import React, { useCallback, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Grid, Space, Textarea, Modal, Stepper } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { usePrescription } from "../../../hooks/prescriptions";
import { PrescriptionResponse, UserResponse } from "../../../../../declarations/main/main.did";
import { userGetPrincipal } from "../../../libs/users";
import PrescriptionView from "./View";
import { useBrowser } from "../../../hooks/browser";
import { useDisclosure } from "@mantine/hooks";
import { useAuth } from "../../../hooks/auth";
import { UserLookup } from "../../users/user/Lookup";
import { UserAvatar } from "../../../components/UserAvatar";
import CertSigner from "../../../components/CertSigner/CertSigner";

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
    const {isMobile} = useBrowser()
    const [opened, { open, close }] = useDisclosure(false);
    const [previewItem, setPreviewItem] = useState<PrescriptionResponse|undefined>();
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

    const handlePreview = useCallback(() => {
        if(!principal) {
            return
        }
        setPreviewItem({
            id: 'temp',
            created_at: BigInt(Date.now()) * 1000000n,
            doctor: principal,
            patient: userGetPrincipal(patient),
            hash: [],
            signature: [],
            cert: '',
            contents: new TextEncoder().encode(form.values.contents),
        })
        open()
    }, [open, principal, form.values, patient]);

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
                            <CertSigner 
                                hash={hash}
                                onSuccess={handleSigned}
                            />
                        </div>

                        <Space h="lg"/>

                        <form onSubmit={form.onSubmit(handleCreate)}>
                            <Grid>
                                <Grid.Col md={6} sm={12}>
                                    <Button
                                        color="blue"
                                        fullWidth
                                        onClick={handlePreview}
                                    >
                                        Preview
                                    </Button>
                                </Grid.Col>
                                <Grid.Col md={6} sm={12}>
                                    <Button
                                        color="red"
                                        fullWidth
                                        disabled={!signature}
                                        type="submit"
                                    >
                                        Submit
                                    </Button>
                                </Grid.Col>
                            </Grid>
                        </form>
                    </Stepper.Step>
                </Stepper>
                
                
            </Container>

            <Modal
                opened={opened}
                size="xl"
                fullScreen={isMobile}
                centered
                onClose={close}
            >
                {previewItem && 
                    <PrescriptionView 
                        item={previewItem}
                        isEncrypted={false}
                    />}
            </Modal>
        </>
    );
};

export default PrescriptionCreate;