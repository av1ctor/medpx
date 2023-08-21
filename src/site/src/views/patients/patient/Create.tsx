import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Space, TextInput } from "@mantine/core";
import { DateInput } from "@mantine/dates";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { userFindMe } from "../../../libs/users";
import { usePatient } from "../../../hooks/patients";

const schema = yup.object().shape({
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
    birth_date: yup.date().required(),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const PatientCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {update} = useAuth();
    const {create} = usePatient();
    
    const form = useForm({
        initialValues: {
          name: '',
          email: '',
          birth_date: new Date('2000-06-15'),
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

            await create(values);
            props.onSuccess('Patient registered!');

            let user = await userFindMe(main);
            update(user);
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

export default PatientCreate;