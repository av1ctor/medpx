import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Select, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { userFindMe } from "../../../libs/users";
import { kinds } from "../../../libs/thirdparties";
import { useThirdParty } from "../../../hooks/thirdparty";

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
    const {create} = useThirdParty();
    
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

            await create(values);
            props.onSuccess('Third party registered!');

            await update(main, await userFindMe(main));
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    return (
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
    );
};

export default ThirdPartyCreate;