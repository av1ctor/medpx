import React, { useCallback, useMemo, useState } from "react";
import { Button, Grid, Select, Stack, TextInput, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { userFindByKey } from "../../../libs/users";
import { keyGetKindIndex, keyGetKindUniqueness, keyStringTokind, kinds } from "../../../libs/keys";
import { Uniqueness } from "../../../libs/keys";
import { useActors } from "../../../hooks/actors";
import { useUI } from "../../../hooks/ui";
import { UserResponse } from "../../../../../declarations/main/main.did";
import countries from "../../../libs/countries";

interface Props {
    setUser: (user: UserResponse|undefined) => void;
}

export const UserLookup = (props: Props) => {
    const {main} = useActors();
    const {showError} = useUI();
    const [isVerifing, setIsVerifing] = useState(false);

    const form = useForm({
        initialValues: {
            kind: '',
            key: '',
            country: '',
        },
    });
    
    const handleLookup = useCallback(async () => {
        try {
            setIsVerifing(true);
            let user = await userFindByKey(
                main, 
                keyStringTokind(form.values.kind), 
                keyGetKindUniqueness(form.values.kind) === Uniqueness.Worldwide? 
                    []: 
                    [form.values.country], 
                form.values.key
            );
            props.setUser(user);
        }
        catch(e) {
            props.setUser(undefined);
            showError(e);
        }
        finally {
            setIsVerifing(false);
        }
    }, [main, form.values, props.setUser, setIsVerifing]);

    const _countries = useMemo(() => {
        return countries.map(c => ({label: c.name, value: c.code}))
    }, []);
    
    return (
        <div className="card">
            <Text weight={500}>
                User
            </Text>
            <form onSubmit={form.onSubmit(handleLookup)}>
                <Stack>
                    <Grid>
                        <Grid.Col md={3} xs={12}>
                            <Select
                                label="Key kind"
                                placeholder="User key kind"
                                data={kinds}
                                {...form.getInputProps('kind')}
                            />
                        </Grid.Col>
                        <Grid.Col md={6} xs={12}>
                            <TextInput
                                label="Key"
                                placeholder="User key"
                                {...form.getInputProps('key')}
                            />
                        </Grid.Col>
                        <Grid.Col md={3} xs={12}>
                            <Select
                                label="Country"
                                placeholder="User country"
                                data={_countries}
                                searchable
                                disabled={form.values.kind === '' || kinds[keyGetKindIndex(form.values.kind)].uniqueness === Uniqueness.Worldwide}
                                {...form.getInputProps('country')}
                            />
                        </Grid.Col>
                    </Grid>
                    <Button
                        variant="filled" 
                        color="green"
                        disabled={!form.values.key}
                        loading={isVerifing}
                        fullWidth
                        type="submit"
                    >
                        Look up
                    </Button>
                </Stack>
            </form>
        </div>
    );
}