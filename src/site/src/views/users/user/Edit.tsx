import React, { useCallback, useEffect } from "react";
import * as yup from 'yup';
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { useForm, yupResolver } from "@mantine/form";
import { Button, Select, Space, TextInput } from "@mantine/core";
import { DateInput } from "@mantine/dates";
import { UserKind, thirdPartyKinds, userGetDoctor, userGetKind, userGetPatient, userGetThirdParty, userGetThirdPartyKind } from "../../../libs/users";
import { useActors } from "../../../hooks/actors";
import { useUser } from "../../../hooks/users";
import { userGetPrincipal } from "../../../libs/users";

const schema = {
    name: yup.string().min(3).max(64),
    email: yup.string().email().min(3).max(128),
};

const doctorSchema = {
    license_num: yup.string().required().min(3).max(32),
    prescription_template: yup.string().optional(),
};

const patientSchema = {
    birth_date: yup.date().required(),
};

const thirdPartySchema = {
    kind: yup.string().required(),
};

interface Props {
}

const UserEdit = (props: Props) => {
    const {main} = useActors();
    const {isLogged, user, update: userUpdate} = useAuth();
    const {showSuccess, showError, toggleLoading} = useUI();
    const {update} = useUser();

    const kind = userGetKind(user);

    const form = useForm({
        initialValues: {
            name: '',
            email: '',
            license_num: '',
            prescription_template: '',
            birth_date: '',
            kind: '',
        },
    
        validate: yupResolver(
            yup.object().shape({...(
                kind === UserKind.Doctor? 
                    doctorSchema
                : kind === UserKind.Patient?
                    patientSchema
                :   
                    thirdPartySchema), 
                ...schema
            })),

        transformValues: (values: any) => ({
            name: values.name,
            email: values.email,
            kind: 
                kind === UserKind.Doctor? 
                    {Doctor: {
                        license_num: values.license_num,
                        prescription_template: !!values.prescription_template? 
                            [values.prescription_template]:
                            [],
                    }}
                : kind === UserKind.Patient?
                    {Patient: {
                        birth_date: values.birth_date.valueOf(),
                    }}
                :   
                    {ThirdParty: {
                        kind: {[values.kind]: null},
                    }},
        }),
    });

    const handleUpdate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            const res = await update(userGetPrincipal(user), values);
            showSuccess('User updated!');

            await userUpdate(main, res);
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main, user]);

    useEffect(() => {
        if(!user) {
            return;
        }

        const kind = userGetKind(user);

        form.setValues({
            name: user.name,
            email: user.email,
            ...(kind === UserKind.Doctor? 
                    {
                        license_num: userGetDoctor(user).license_num,
                        prescription_template: '',
                    }
                : kind === UserKind.Patient?
                    {
                        birth_date: new Date(Number(userGetPatient(user).birth_date))
                    }
                :
                    {
                        kind: userGetThirdPartyKind(userGetThirdParty(user).kind).value,
                    }
            ), 
        } as any);
    }, [user]);

    if(!isLogged) {
        return null;
    }

    return (
        <form onSubmit={form.onSubmit(handleUpdate)}>
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

            {kind === UserKind.Patient && 
                <>
                    <DateInput
                        label="Birth date"
                        placeholder="Your birthday"
                        valueFormat="YYYY-MM-DD"
                        defaultLevel="decade"
                        {...form.getInputProps('birth_date')}
                    />
                </>
            }
            {kind === UserKind.Doctor && 
                <>
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
                </>
            }
            {kind === UserKind.ThirdParty && 
                <>
                    <Select
                        label="Kind"
                        placeholder="Kind"
                        data={thirdPartyKinds}
                        {...form.getInputProps('kind')}
                    />
                </>
            }
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

export default UserEdit;