import React from "react";
import { Global, rem } from "@mantine/core";

export const GlobalStyles = () => {
    return (
        <Global
            styles={(theme) => ({
                '.list-item': {
                    '& + &': {
                        paddingTop: theme.spacing.sm,
                        marginTop: theme.spacing.sm,
                        borderTop: `${rem(1)} solid ${
                            theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
                        }`,
                    },
                },
                '.list-create-button': {
                    position: 'absolute',
                    top: '4px',
                    right: '4px',
                },
                '.card': {
                    backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
                },
                '.card-title': {
                    lineHeight: 1,
                },
            })}
        />
    );
  }