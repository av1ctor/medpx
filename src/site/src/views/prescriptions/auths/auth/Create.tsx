import React, { useCallback, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Flex, Select, Space } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../../hooks/ui";
import { useActors } from "../../../../hooks/actors";
import { kinds } from "../../../../libs/prescription_auths";
import { usePrescriptionAuth } from "../../../../hooks/prescription_auths";
import { UserLookup } from "../../../users/user/Lookup";
import { UserResponse } from "../../../../../../declarations/main/main.did";
import { userGetPrincipal } from "../../../../libs/users";

const schema = yup.object().shape({
    prescription_id: yup.string().required(),
    kind: yup.string().required(),
    value: yup.string().min(3).max(64),
});

interface Props {
    prescriptionId: string,
    onSuccess: (msg: string) => void;
}

const PrescriptionAuthCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = usePrescriptionAuth();
    const [thirdParty, setThirdParty] = useState<UserResponse|undefined>()
    
    const form = useForm({
        initialValues: {
            prescription_id: props.prescriptionId,
            kind: '',
            expires_at: []
        },
    
        validate: yupResolver(schema),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await create({
                ...values,
                kind: {[values.kind]: null},
                to: userGetPrincipal(thirdParty),
            });

            props.onSuccess('Prescription shared!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, thirdParty]);

    const data = thirdParty && 'ThirdParty' in thirdParty.kind?
        thirdParty.kind.ThirdParty:
        null;

    return (
        <Container>
            <UserLookup 
                setUser={setThirdParty}
            />                    

            <Space h="xl" />

            <Flex direction="column">
                <div><b>Name:</b> {data?.name}</div>
                <div><b>Id:</b> {data?.id.toString()}</div>
            </Flex>

            <Space h="1rem" />

            <form onSubmit={form.onSubmit(handleCreate)}>
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
                    disabled={!thirdParty}
                >
                    Submit
                </Button>
            </form>
        </Container>
    );
};

export default PrescriptionAuthCreate;