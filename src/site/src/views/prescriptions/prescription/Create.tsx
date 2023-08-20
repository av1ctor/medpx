import React, { useCallback, useMemo, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Grid, Select, Space, Stack, TextInput, Textarea, Text } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { usePrescription } from "../../../hooks/prescriptions";
import { UserResponse } from "../../../../../declarations/main/main.did";
import { userFindByKey, userGetPrincipal } from "../../../libs/users";
import { Uniqueness, keyGetKind, keyGetKindIndex, keyGetKindUniqueness, keyStringTokind as keyStringToKind, kinds } from "../../../libs/keys";
import countries from "../../../libs/countries";

const schema = yup.object().shape({
    kind: yup.string().required(),
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
                keyStringToKind(form.values.kind), 
                keyGetKindUniqueness(form.values.kind) === Uniqueness.Worldwide? 
                    []: 
                    [form.values.country], 
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

    const handlePreview = useCallback(() => {

    }, []);

    const _countries = useMemo(() => {
        return countries.map(c => ({label: c.name, value: c.code}))
    }, []);

    return (
        <Container>
            <form onSubmit={form.onSubmit(handleCreate)}>
                <div className="card">
                    <Text weight={500}>
                        Patient
                    </Text>
                    <Stack>
                        <Grid>
                            <Grid.Col md={3} xs={12}>
                                <Select
                                    label="Key kind"
                                    placeholder="Patient's key kind"
                                    data={kinds}
                                    {...form.getInputProps('kind')}
                                />
                            </Grid.Col>
                            <Grid.Col md={6} xs={12}>
                                <TextInput
                                    label="Key"
                                    placeholder="Patient's key"
                                    {...form.getInputProps('patient')}
                                />
                            </Grid.Col>
                            <Grid.Col md={3} xs={12}>
                                <Select
                                    label="Country"
                                    placeholder="Patient's country"
                                    data={_countries}
                                    searchable
                                    disabled={form.values.kind === '' || kinds[keyGetKindIndex(form.values.kind)].uniqueness === Uniqueness.Worldwide}
                                    {...form.getInputProps('country')}
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
                            Verify
                        </Button>
                    </Stack>
                </div>

                <Space h="xl" />

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
                            disabled={!patient}
                            onClick={handlePreview}
                        >
                            Preview
                        </Button>
                    </Grid.Col>
                    <Grid.Col md={6} sm={12}>
                        <Button
                            color="red"
                            fullWidth
                            type="submit"
                            disabled={!patient}
                        >
                            Submit
                        </Button>
                    </Grid.Col>
                </Grid>
                
            </form>
        </Container>
    );
};

export default PrescriptionCreate;