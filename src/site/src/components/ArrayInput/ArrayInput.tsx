import React, { forwardRef, useCallback } from "react";
import { Container } from "@mantine/core";
import { DefaultProps } from '@mantine/styles';
import { ForwardRefWithStaticComponents } from '@mantine/utils';
import { FormattedMessage } from "react-intl";
import { ArrayInputProvider } from "./Context";
import { Create } from "./Create";
import { Item } from "./Item";

interface ArrayInputProps extends DefaultProps, React.ComponentPropsWithoutRef<'div'> {
    label?: string;
    disabled?: boolean;
    value: any;
    children: any;
    onChange?: (value: any) => void;
}

type ArrayInputComponent = ForwardRefWithStaticComponents<ArrayInputProps, {
    Item: typeof Item;
    Create: typeof Create;
}>;

export const ArrayInput: ArrayInputComponent = forwardRef<HTMLDivElement, ArrayInputProps>((props, ref) => {
    const handleDelete = useCallback((index: number) => {
        if(props.onChange) {
            const arr = Array.from(props.value);
            arr.splice(index, 1);
            props.onChange(arr);
        }
    }, [props.onChange, props.value]);

    const handleCreate = useCallback(() => {
        if(props.onChange) {
            const arr = Array.from(props.value);
            arr.push(null);
            props.onChange(arr);
        }
    }, [props.onChange, props.value]);
    
    return (
        <ArrayInputProvider
            value={{ 
                onCreate: handleCreate,
                onDelete: handleDelete,
            }}
        >
            <Container ref={ref}>
                {props.label &&
                    <label className="label">
                        <FormattedMessage id={props.label} defaultMessage={props.label} />
                    </label>
                }
                {props.children}
            </Container>
        </ArrayInputProvider>
    );
}) as any;

ArrayInput.Item = Item;
ArrayInput.Create = Create;