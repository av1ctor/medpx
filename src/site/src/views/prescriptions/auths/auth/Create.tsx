import React, { useCallback, useEffect, useState } from "react";
import * as yup from 'yup';
import { Button, Container, Flex, Grid, Select, Space, Stepper, Switch } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../../hooks/ui";
import { useActors } from "../../../../hooks/actors";
import { kinds } from "../../../../libs/prescription_auths";
import { usePrescriptionAuth } from "../../../../hooks/prescription_auths";
import { UserLookup } from "../../../users/user/Lookup";
import { UserResponse } from "../../../../../../declarations/main/main.did";
import { userGetPrincipal } from "../../../../libs/users";
import { DateInput } from "@mantine/dates";

const schema = yup.object().shape({
    prescription_id: yup.string().required(),
    kind: yup.string().required(),
    value: yup.string().min(3).max(64),
    expires_at: yup.date().optional(),
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
    const [active, setActive] = useState(0);
    
    const form = useForm({
        initialValues: {
            prescription_id: props.prescriptionId,
            kind: '',
            expires: false,
            expires_at: undefined,
        },
    
        validate: yupResolver(schema),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            if(values.expires_at) {
                values.expires_at.setHours(23);
                values.expires_at.setMinutes(59);
                values.expires_at.setSeconds(59);
            }

            await create({
                ...values,
                kind: {[values.kind]: null},
                to: userGetPrincipal(thirdParty),
                expires_at: values.expires_at?
                    [BigInt(values.expires_at.valueOf()) * 1000000n]:
                    [],
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

    useEffect(() => {
        setActive(thirdParty? 1: 0);
    }, [thirdParty]);

    const data = thirdParty && 'ThirdParty' in thirdParty.kind?
        thirdParty.kind.ThirdParty:
        null;

    return (
        <Container>
            <Stepper 
                active={active} 
                onStepClick={setActive}
                breakpoint="sm"
            >
                <Stepper.Step 
                    label="Third party" 
                    description="Lookup third party"
                >
                    <UserLookup 
                        setUser={setThirdParty}
                    />                    
                </Stepper.Step>
                <Stepper.Step 
                    label="Options" 
                    description="Authorization options"
                    allowStepSelect={!!thirdParty}
                >
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
                        <DateInput
                            label="Expiration date"
                            placeholder="Expirates at"
                            valueFormat="YYYY-MM-DD"
                            preserveTime={false}
                            {...form.getInputProps('expires_at')}
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
                </Stepper.Step>
            </Stepper>
        </Container>
    );
};

export default PrescriptionAuthCreate;