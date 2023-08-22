import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Container, Select, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../../hooks/ui";
import { useActors } from "../../../../hooks/actors";
import { kinds } from "../../../../libs/prescription_auths";
import { usePrescriptionAuth } from "../../../../hooks/prescription_auths";
import { PrescriptionResponse } from "../../../../../../declarations/main/main.did";
import { Principal } from "@dfinity/principal";

const schema = yup.object().shape({
    prescription_id: yup.string().required(),
    kind: yup.string().required(),
    value: yup.string().min(3).max(64),
});

interface Props {
    item: PrescriptionResponse,
    onSuccess: (msg: string) => void;
}

const PrescriptionAuthCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = usePrescriptionAuth();
    
    const form = useForm({
        initialValues: {
            prescription_id: props.item.id,
            kind: '',
            to: '',
            expires_at: []
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            kind: {[values.kind]: null},
            to: Principal.fromText(values.to),
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await create(values);
            props.onSuccess('Prescription shared!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    return (
        <Container>
            <form onSubmit={form.onSubmit(handleCreate)}>
                <TextInput
                    label="User"
                    placeholder="User id to share with"
                    required
                    {...form.getInputProps('to')}
                />
                <Select
                    label="Kind"
                    placeholder="Sharing kind"
                    data={kinds}
                    required
                    {...form.getInputProps('kind')}
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

export default PrescriptionAuthCreate;