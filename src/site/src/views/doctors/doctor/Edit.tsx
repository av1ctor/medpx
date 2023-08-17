import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Container, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { userFindMe, userGetPrincipal } from "../../../libs/users";
import { useDoctor } from "../../../hooks/doctors";
import { DoctorResponse } from "../../../../../declarations/main/main.did";

const schema = yup.object().shape({
    license_num: yup.string().required().min(3).max(32),
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
    prescription_template: yup.string().optional(),
});

interface Props {
    entity: DoctorResponse,
    onSuccess: (msg: string) => void;
}

const DoctorEdit = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {user, update: userUpdate} = useAuth();
    const {update} = useDoctor();
    
    const form = useForm({
        initialValues: {
            ...props.entity,
            prescription_template: props.entity.prescription_template.length > 0?
                props.entity.prescription_template[0]:
                '',
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            prescription_template: values.prescription_template !== ''? 
                [values.prescription_template]:
                [],
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await update(userGetPrincipal(user), values);
            props.onSuccess('Doctor updated!');

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
        <Container>
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
                <TextInput
                    label="License num"
                    placeholder="Your license number"
                    {...form.getInputProps('license_num')}
                />
                <TextInput
                    label="Prescription Template"
                    placeholder="Your prescription template"
                    {...form.getInputProps('prescription_template')}
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

export default DoctorEdit;