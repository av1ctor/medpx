import React, { useCallback } from "react";
import * as yup from 'yup';
import { Button, Container, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useAuth } from "../../../hooks/auth";
import { findMe } from "../../../libs/users";

const schema = yup.object().shape({
    license_num: yup.string().required().min(3).max(32),
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
    prescription_template: yup.string().optional(),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const DoctorCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {update} = useAuth();
    
    const form = useForm({
        initialValues: {
            license_num: '',
            name: '',
            email: '',
            prescription_template: '',
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

            if(!main) {
                throw Error('Main actor undefined');
            }
            
            const res = await main.doctor_create(values);
            if('Ok' in res) {
                let user = await findMe(main);
                update(user);
                props.onSuccess('Doctor registered!');
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

export default DoctorCreate;