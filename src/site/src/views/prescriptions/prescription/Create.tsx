import React, { useCallback, useMemo, useState } from "react";
import * as yup from 'yup';
import { Button, Card, Container, Grid, Select, Space, Stack, TextInput, Textarea, Text } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { IconCircleCheck } from "@tabler/icons-react";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { usePrescription } from "../../../hooks/prescriptions";
import { UserResponse } from "../../../../../declarations/main/main.did";
import { userFindByKey, userGetPrincipal } from "../../../libs/users";
import { keyBuildKind as keyStringToKind, kinds } from "../../../libs/keys";
import countries from "../../../libs/countries";

const schema = yup.object().shape({
    patient: yup.string().required(),
    contents: yup.string().min(2).max(4096),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const PrescriptionCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = usePrescription();
    const [isVerifing, setIsVerifing] = useState(false);
    const [patient, setPatient] = useState<UserResponse|undefined>();
    
    const form = useForm({
        initialValues: {
            country: '',
            kind: '',
            patient: '',
            contents: '',
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            patient: userGetPrincipal(patient),
            contents: new TextEncoder().encode(values.contents),
        }),
    });

    const handleVerify = useCallback(async () => {
        try {
            setIsVerifing(true);
            let pat = await userFindByKey(
                main, 
                form.values.country, 
                keyStringToKind(form.values.kind), 
                form.values.patient
            );
            setPatient(pat);
        }
        catch(e) {
            setPatient(undefined);
            showError(e);
        }
        finally {
            setIsVerifing(false);
        }
    }, [form.values.patient, main, setPatient, setIsVerifing]);

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await create(values);
            props.onSuccess('Prescription created!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    const _countries = useMemo(() => {
        return countries.map(c => ({label: c.name, value: c.code}))
    }, []);

    return (
        <Container>
            <form onSubmit={form.onSubmit(handleCreate)}>
                <Card radius="sm" withBorder>
                    <Text weight={500}>Patient</Text>
                    <Stack>
                        <Grid>
                            <Grid.Col md={4} xs={12}>
                                <Select
                                    label="Country"
                                    placeholder="Patient's country"
                                    data={_countries}
                                    searchable
                                    {...form.getInputProps('country')}
                                />
                            </Grid.Col>
                            <Grid.Col md={4} xs={12}>
                                <Select
                                    label="Kind"
                                    placeholder="Patient's key kind"
                                    data={kinds}
                                    {...form.getInputProps('kind')}
                                />
                            </Grid.Col>
                            <Grid.Col md={4} xs={12}>
                                <TextInput
                                    label="Key"
                                    placeholder="Patient's key"
                                    {...form.getInputProps('patient')}
                                />
                            </Grid.Col>
                        </Grid>
                        <Button
                            variant="filled" 
                            color="blue"
                            disabled={!form.values.patient}
                            loading={isVerifing}
                            fullWidth
                            onClick={handleVerify}
                        >
                            <IconCircleCheck size="1rem" /> Verify
                        </Button>
                    </Stack>
                </Card>

                <Space h="xl" />

                <Textarea
                    label="Contents"
                    placeholder="Contents"
                    minRows={20}
                    disabled={!patient}
                    {...form.getInputProps('contents')}
                />
                <Space h="lg"/>
                <Button
                    color="red"
                    fullWidth
                    type="submit"
                >
                    Submit
                </Button>
            </form>
        </Container>
    );
};

export default PrescriptionCreate;