import React, { useCallback, useMemo } from "react";
import * as yup from 'yup';
import { Button, Container, Select, Space, TextInput } from "@mantine/core";
import { useForm, yupResolver } from "@mantine/form";
import { useUI } from "../../../hooks/ui";
import { useActors } from "../../../hooks/actors";
import { Uniqueness, keyGetKindIndex, keyGetKindUniqueness, kinds } from "../../../libs/keys";
import { useKey } from "../../../hooks/keys";
import countries from "../../../libs/countries";

const schema = yup.object().shape({
    kind: yup.string().required(),
    country: yup.string().when(["kind"], (values, schema) => {
        if(values[0] !== '' && keyGetKindUniqueness(values[0]) !== Uniqueness.Worldwide)
            return schema.required().length(2);
        return schema;
    }),
    value: yup.string().min(3).max(64),
});

interface Props {
    onSuccess: (msg: string) => void;
}

const KeyCreate = (props: Props) => {
    const {main} = useActors();
    const {toggleLoading, showError} = useUI();
    const {create} = useKey();
    
    const form = useForm({
        initialValues: {
          kind: '',
          country: '',
          value: '',
        },
    
        validate: yupResolver(schema),

        transformValues: (values) => ({
            ...values,
            kind: {[values.kind]: null},
            country: keyGetKindUniqueness(values.kind) === Uniqueness.Worldwide? 
                []: 
                [values.country], 
        }),
    });

    const handleCreate = useCallback(async (values: any) => {
        try {
            toggleLoading(true);

            await create(values);
            props.onSuccess('Key created!');
        }
        catch(e: any) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [main]);

    const _countries = useMemo(() => {
        return countries.map(c => ({label: c.name, value: c.code}))
    }, []);

    return (
        <Container>
            <form onSubmit={form.onSubmit(handleCreate)}>
                <Select
                    label="Kind"
                    placeholder="Key kind"
                    data={kinds}
                    {...form.getInputProps('kind')}
                />
                <TextInput
                    label="Value"
                    placeholder="Key value"
                    {...form.getInputProps('value')}
                />
                <Select
                    label="Country"
                    placeholder="Your country"
                    data={_countries}
                    searchable
                    disabled={form.values.kind === '' || kinds[keyGetKindIndex(form.values.kind)].uniqueness === Uniqueness.Worldwide}
                    {...form.getInputProps('country')}
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

export default KeyCreate;