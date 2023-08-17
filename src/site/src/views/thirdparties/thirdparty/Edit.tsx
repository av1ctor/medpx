import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Container, Select, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { userFindMe, userGetPrincipal } from "../../../libs/users";
import { kinds, thirdPartyGetKind } from "../../../libs/thirdparties";
import { useThirdParty } from "../../../hooks/thirdparty";
import { ThirdPartyResponse } from "../../../../../declarations/main/main.did";

const schema = yup.object().shape({
    kind: yup.string().required(),
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
});

interface Props {
    entity: ThirdPartyResponse,
    onSuccess: (msg: string) => void;
}

const ThirdPartyEdit = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {user, update: userUpdate} = useAuth();
    const {update} = useThirdParty();
    
    const form = useForm({
        initialValues: {
            ...props.entity,
            kind: thirdPartyGetKind(props.entity.kind)
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
            
            await update(userGetPrincipal(user), values);
            props.onSuccess('Third party updated!');

            userUpdate(await userFindMe(main));
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
                    label="Id"
                    placeholder="Your id"
                    {...form.getInputProps('id')}
                    readOnly
                />
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

export default ThirdPartyEdit;