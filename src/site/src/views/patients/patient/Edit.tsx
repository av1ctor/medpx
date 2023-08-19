import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Space, TextInput } from "@mantine/core";
import { DateInput } from "@mantine/dates";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { userFindMe, userGetPrincipal } from "../../../libs/users";
import { usePatient } from "../../../hooks/patients";
import { PatientResponse } from "../../../../../declarations/main/main.did";

const schema = yup.object().shape({
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
    birth_date: yup.date().required(),
});

interface Props {
    entity: PatientResponse,
    onSuccess: (msg: string) => void;
}

const PatientEdit = (props: Props) => {
    const {user} = useAuth();
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {update: userUpdate} = useAuth();
    const {update} = usePatient();
    
    const form = useForm({
        initialValues: {
            ...props.entity,
            birth_date: new Date(Number(props.entity.birth_date))
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            birth_date: values.birth_date.valueOf(),
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await update(userGetPrincipal(user), values );
            props.onSuccess('Patient updated!');
            
            userUpdate(await userFindMe(main));
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, user]);

    return (
        <form onSubmit={form.onSubmit(handleCreate)}>
            <TextInput
                label="Id"
                placeholder="Your id"
                {...form.getInputProps('id')}
                readOnly
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
            <DateInput
                label="Birth date"
                placeholder="Your birthday"
                valueFormat="YYYY-MM-DD"
                defaultLevel="decade"
                {...form.getInputProps('birth_date')}
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

export default PatientEdit;