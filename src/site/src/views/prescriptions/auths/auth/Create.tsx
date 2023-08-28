import React, { useCallback, useEffect, useMemo, useState } from "react";
import * as yup from 'yup';
import { Box, Button, Center, Container, Flex, SegmentedControl, Select, Space, Stepper } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { DateInput } from "@mantine/dates";
import { IconUser, IconUsersGroup } from "@tabler/icons-react";
import { useUI } from "../../../../hooks/ui";
import { useActors } from "../../../../hooks/actors";
import { AuthTarget, kinds, prescriptionAuthStringToTarget } from "../../../../libs/prescription_auths";
import { usePrescriptionAuth } from "../../../../hooks/prescription_auths";
import { UserLookup } from "../../../users/user/Lookup";
import { GroupResponse, UserResponse } from "../../../../../../declarations/main/main.did";
import { useBrowser } from "../../../../hooks/browser";
import { GroupMembers } from "../../../groups/Item";
import ChooseGroup from "../../../groups/group/Choose";
import { UserAvatar } from "../../../../components/UserAvatar";

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
    const {isMobile} = useBrowser();
    const {create} = usePrescriptionAuth();
    const [target, setTarget] = useState(AuthTarget.User);
    const [user, setUser] = useState<UserResponse|undefined>()
    const [group, setGroup] = useState<GroupResponse|undefined>()
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

    const handleChangeTarget = useCallback((value: string) => {
        setUser(undefined);
        setGroup(undefined);
        setTarget(prescriptionAuthStringToTarget(value));
    }, [setTarget]);

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
                to: target === AuthTarget.User? 
                    {User: user?.id}
                :
                    {Group: group?.id},
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
    }, [main, user, group, target]);

    useEffect(() => {
        if(target === AuthTarget.User)
            setActive(user? 1: 0);
        else
            setActive(group? 1: 0);
    }, [user, group, target]);

    const targets = useMemo(() => {
        return [
            {
                value: AuthTarget[AuthTarget.User],
                label: (
                    <Center>
                        <IconUser />
                        <Box ml={10}>User</Box>
                    </Center>
                ),
            },
            {
                value: AuthTarget[AuthTarget.Group],
                label: (
                    <Center>
                        <IconUsersGroup />
                        <Box ml={10}>Group</Box>
                    </Center>
                ),
            },
        ];
    }, []);

    return (
        <Container>
            <Stepper 
                active={active} 
                onStepClick={setActive}
                breakpoint="sm"
            >
                <Stepper.Step 
                    label="Target" 
                    description="Lookup target"
                >
                    <Container>
                        <SegmentedControl
                            size="md"
                            orientation={isMobile? "vertical": "horizontal"}
                            fullWidth
                            color="blue"
                            value={AuthTarget[target]}
                            data={targets}
                            onChange={handleChangeTarget}
                        />

                        <Space h="md"/>

                        {target === AuthTarget.User &&
                            <UserLookup 
                                onChange={setUser}
                            />
                        }
                        {target === AuthTarget.Group &&
                            <ChooseGroup
                                onChange={setGroup}
                            />
                        }
                    </Container>
                </Stepper.Step>
                <Stepper.Step 
                    label="Options" 
                    description="Authorization options"
                    allowStepSelect={!!user}
                >
                    {target === AuthTarget.User &&
                        <UserAvatar user={user} />
                    }
                    {target === AuthTarget.Group && 
                        <Flex direction="column">
                            <div>
                                Members: <GroupMembers members={group?.members} />
                            </div>
                            <div><b>Id:</b> {group?.id.toString()}</div>
                        </Flex>
                    }

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
                            disabled={!user && !group}
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