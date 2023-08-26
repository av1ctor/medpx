import React, { useCallback, useEffect, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Grid, Space, Textarea, Flex, Modal, Stepper } from "@mantine/core";
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

const schema = yup.object().shape({
    contents: yup.string().min(16).max(4096),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const PrescriptionCreate = (props: Props) => {
    const {principal, aes_gcm} = useAuth();
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = usePrescription();
    const {isMobile} = useBrowser()
    const [opened, { open, close }] = useDisclosure(false);
    const [previewItem, setPreviewItem] = useState<PrescriptionResponse|undefined>();
    const [patient, setPatient] = useState<UserResponse|undefined>();
    const [active, setActive] = useState(0);
    
    const form = useForm({
        initialValues: {
            contents: '',
        },
    
        validate: yupResolver(schema),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            if(!aes_gcm) {
                throw Error("AES-GCM undefined");
            }

            const principal = userGetPrincipal(patient);
            
            const rawKey = await aes_gcm.genRawKey('prescriptions', principal);
            if('Err' in rawKey || !rawKey.Ok) {
                throw new Error('Raw key generation failed');
            }
            
            const contents = await aes_gcm.encrypt(values.contents, rawKey.Ok);

            await create({
                patient: principal,
                contents,
            });

            props.onSuccess('Prescription created!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, aes_gcm, patient]);

    const handlePreview = useCallback(() => {
        if(!principal) {
            return
        }
        setPreviewItem({
            id: 'temp',
            created_at: BigInt(Date.now()) * 1000000n,
            doctor: principal,
            patient: userGetPrincipal(patient),
            contents: new TextEncoder().encode(form.values.contents),
        })
        open()
    }, [open, principal, form.values, patient]);

    useEffect(() => {
        setActive(patient? 1: 0);
    }, [patient]);

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
                        description="Lookup patient"
                    >
                        <UserLookup 
                            onChange={setPatient}
                        />
                    </Stepper.Step>
                    <Stepper.Step 
                        label="Contents" 
                        description="Prescription contents"
                        allowStepSelect={!!patient}
                    >
                        <Flex direction="column">
                            <div><b>Name:</b> {patient?.name}</div>
                            <div><b>Id:</b> {patient?.id.toString()}</div>
                        </Flex>

                        <Space h="1rem" />

                        <form onSubmit={form.onSubmit(handleCreate)}>
                            <Textarea
                                label="Contents"
                                placeholder="Contents"
                                minRows={20}
                                disabled={!patient}
                                {...form.getInputProps('contents')}
                            />
                            
                            <Space h="lg"/>
                            
                            <Grid>
                                <Grid.Col md={6} sm={12}>
                                    <Button
                                        color="blue"
                                        fullWidth
                                        disabled={!patient || form.values.contents.length < 3}
                                        onClick={handlePreview}
                                    >
                                        Preview
                                    </Button>
                                </Grid.Col>
                                <Grid.Col md={6} sm={12}>
                                    <Button
                                        color="red"
                                        fullWidth
                                        disabled={!patient || form.values.contents.length < 3}
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