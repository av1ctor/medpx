import React, { useCallback, useMemo, useState } from "react";
import * as yup from 'yup';
import { ActionIcon, Box, Button, Container, Group, Space, TextInput, Text, Center } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { randomId, useDisclosure } from '@mantine/hooks';
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { useGroup } from "../../../hooks/groups";
import { UserLookup } from "../../users/user/Lookup";
import { UserResponse } from "../../../../../declarations/main/main.did";
import { userGetPrincipal } from "../../../libs/users";
import { Principal } from "@dfinity/principal";

const schema = yup.object().shape({
    members: yup.array().min(1).max(16),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const GroupCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = useGroup();
    const [opened, { open, close }] = useDisclosure(true);
    const [user, setUser] = useState<UserResponse|undefined>();
    
    const form = useForm({
        initialValues: {
            members: new Array<{key: string, value: Principal}>(),
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            members: values.members.map(m => m.value)
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await create(values);
            props.onSuccess('Group created!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    const handleAddMember = useCallback(() => {
        close();
        form.insertListItem('members', {key: randomId(), value: userGetPrincipal(user)})
    }, [form, user, close]);

    const fields = useMemo(() => form.values.members.map((item, index) => (
        <Group key={item.key} mt="xs">
            <TextInput
                placeholder="User principal"
                withAsterisk
                sx={{ flex: 1 }}
                disabled
                {...form.getInputProps(`members.${index}.value`)}
            />
            <ActionIcon color="red" onClick={() => form.removeListItem('members', index)}>
                <IconTrash size="1rem" />
            </ActionIcon>
        </Group>
    )), [form]);

    return (
        <Container>
            <form onSubmit={form.onSubmit(handleCreate)}>
                <div className="card">
                    <Text size="sm"><b>Members</b></Text>
                    {fields.length > 0?
                        fields
                    :
                        <Text color="dimmed" align="center">None</Text>
                    }

                    {form.errors.members &&
                        <Text color="red" size="sm">{form.errors.members}</Text>
                    }

                    {opened &&
                        <>
                            <Box pt="xl" pb="xl">
                                <UserLookup 
                                    onChange={setUser} 
                                />
                            </Box>
                            <Group position="center">
                                <Button
                                    color="green"
                                    onClick={handleAddMember}
                                >
                                    Add
                                </Button>
                                <Button
                                    color="red"
                                    onClick={close}
                                >
                                    Cancel
                                </Button>
                            </Group>
                        </>
                    }

                    <Box display={opened? 'none': 'block'}>
                        <Center pt="md" pb="md">
                            <ActionIcon 
                                variant="filled"
                                color="green"
                                title="New"
                                onClick={open}
                            >
                                <IconPlus size="1rem" />
                            </ActionIcon>
                        </Center>
                    </Box>

                    
                </div>

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

export default GroupCreate;