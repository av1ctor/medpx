import React, { useCallback, useMemo, useState } from "react";
import { Button, Grid, Select, Stack, TextInput, Text, Box } from "@mantine/core";
import { useForm } from "@mantine/form";
import { userFindByKey, userGetName, userGetPrincipal } from "../../../libs/users";
import { keyGetKindIndex, keyGetKindUniqueness, keyStringTokind, kinds } from "../../../libs/keys";
import { Uniqueness } from "../../../libs/keys";
import { useActors } from "../../../hooks/actors";
import { useUI } from "../../../hooks/ui";
import { UserResponse } from "../../../../../declarations/main/main.did";
import countries from "../../../libs/countries";
import { UserAvatar } from "../../../components/UserAvatar";

interface Props {
    onChange: (user: UserResponse|undefined) => void;
}

export const UserLookup = (props: Props) => {
    const {main} = useActors();
    const {showError} = useUI();
    const [isVerifing, setIsVerifing] = useState(false);
    const [user, setUser] = useState<UserResponse|undefined>();

    const form = useForm({
        initialValues: {
            kind: '',
            key: '',
            country: '',
        },
    });
    
    const handleLookup = useCallback(async (e: any) => {
        e.preventDefault();
        
        try {
            setIsVerifing(true);

            if(form.validate().hasErrors) {
                return;
            }

            let user = await userFindByKey(
                main, 
                keyStringTokind(form.values.kind), 
                keyGetKindUniqueness(form.values.kind) === Uniqueness.Worldwide? 
                    []: 
                    [form.values.country], 
                form.values.key
            );
            setUser(user);
            props.onChange(user);
        }
        catch(e) {
            setUser(undefined);
            props.onChange(undefined);
            showError(e);
        }
        finally {
            setIsVerifing(false);
        }
    }, [main, form.values, props.onChange, setIsVerifing]);

    const _countries = useMemo(() => {
        return countries.map(c => ({label: c.name, value: c.code}))
    }, []);
    
    return (
        <div>
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
                <UserAvatar user={user} />
                <Button
                    variant="filled" 
                    color="green"
                    disabled={!form.values.key}
                    loading={isVerifing}
                    fullWidth
                    onClick={handleLookup}
                >
                    Look up
                </Button>
            </Stack>
        </div>
    );
}