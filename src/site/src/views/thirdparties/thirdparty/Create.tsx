import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Container, Select, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { findMe } from "../../../libs/users";
import { kinds } from "../../../libs/thirdparties";

const schema = yup.object().shape({
    kind: yup.string().required(),
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const ThirdPartyCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {update} = useAuth();
    
    const form = useForm({
        initialValues: {
          kind: '',
          name: '',
          email: '',
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            kind: {[values.kind]: null},
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            if(!main) {
                throw Error('Main actor undefined');
            }
            
            const res = await main.thirdparty_create(values);
            if('Ok' in res) {
                let user = await findMe(main);
                update(user);
                props.onSuccess('Third party registered!');
            }
            else {
                showError(res.Err);
            }
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
                <Select
                    label="Kind"
                    placeholder="Kind"
                    data={kinds}
                    {...form.getInputProps('kind')}
                />
                <TextInput
                    label="Name"
                    placeholder="Your name"
                    {...form.getInputProps('name')}
                />
                <TextInput
                    label="Email"
                    placeholder="Your e-mail"
                    {...form.getInputProps('email')}
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

export default ThirdPartyCreate;