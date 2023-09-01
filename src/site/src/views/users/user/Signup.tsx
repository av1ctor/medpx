import React, { useEffect, useMemo, useState, useCallback } from "react";
import * as yup from 'yup';
import { Box, Button, Center, Container, SegmentedControl, Select, Space, Stepper, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { DateInput } from "@mantine/dates";
import { IconStethoscope, IconUserHeart, IconUsersGroup } from "@tabler/icons-react";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { useBrowser } from "../../../hooks/browser";
import { useUser } from "../../../hooks/users";
import { useActors } from "../../../hooks/actors";
import { UserKind, thirdPartyKinds, userStringToKind } from "../../../libs/users";
import Login from "./Login";

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

const Signup = (props: Props) => {
    const {isAuthenticated, isLogged} = useAuth();
    const {showSuccess, showError, toggleLoading} = useUI();
    const {main} = useActors();
    const {isMobile, returnToLastPage} = useBrowser();
    const [active, setActive] = useState(0);
    const [kind, setKind] = useState(UserKind.Patient);
    const [options, setOptions] = useState({
        initialValues: {
            name: '',
            email: '',
            license_num: '',
            prescription_template: '',
            birth_date: new Date('2000-06-15'),
            kind: '',
        }
    });

    const handleChangeKind = useCallback((value: string) => {
        setKind(userStringToKind(value));
    }, [setKind]);

    const handleAuthenticated = useCallback(() => {
        setActive(1);
    }, []);

    const {update} = useAuth();
    const {create} = useUser();
    
    const form = useForm(options);

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            const user = await create(values);
            showSuccess('User registered!');
            returnToLastPage();

            await update(main, user);
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    useEffect(() => {
        setOptions({
            ...options,
        
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
        } as any);
    }, [kind]);

    useEffect(() => {
        if(isLogged) {
            returnToLastPage();
        }
        else if(isAuthenticated) {
            if(active === 0) {
                setActive(1);
            }
        }
    }, [isAuthenticated, isLogged, active, setActive]);

    const kinds = useMemo(() => {
        return [
            {
                value: UserKind[UserKind.Patient],
                label: (
                    <Center>
                        <IconUserHeart />
                        <Box ml={10}>Patient</Box>
                    </Center>
                ),
            },
            {
                value: UserKind[UserKind.Doctor],
                label: (
                    <Center>
                        <IconStethoscope />
                        <Box ml={10}>Doctor</Box>
                    </Center>
                ),
            },
            {
                value: UserKind[UserKind.ThirdParty],
                label: (
                    <Center>
                        <IconUsersGroup />
                        <Box ml={10}>Third party</Box>
                    </Center>
                ),
            },
        ];
    }, []);

    return (
        <Stepper 
            active={active} 
            breakpoint="sm"
            color="green"
            onStepClick={setActive}
        >
            <Stepper.Step 
                label="Authentication" 
                description="Authenticate with your provider"
                allowStepSelect={false}
            >
                <Login 
                    authenticateOnly={true}
                    onAuthenticated={handleAuthenticated} 
                />
            </Stepper.Step>
            <Stepper.Step 
                label="Registration" 
                description="Create a new account"
                allowStepSelect={false}
            >
                <Space h="md" />
                <Container>
                    <SegmentedControl
                        size="md"
                        orientation={isMobile? "vertical": "horizontal"}
                        fullWidth
                        color="blue"
                        value={UserKind[kind]}
                        data={kinds}
                        onChange={handleChangeKind}
                    />
                </Container>
                <Space h="md" />

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
            </Stepper.Step>
        </Stepper>
   );
};

export default Signup;