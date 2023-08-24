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
                    '.clickable': {
                        width: '100%',
                    },
                    '.clickable:hover': {
                        cursor: 'pointer',
                        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.blue[0], 
                    },
                },
                '.list-create-button': {
                    position: 'absolute',
                    top: '4px',
                    right: '4px',
                },
                '.card': {
                    backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
                    border: '0.0625rem solid #dee2e6',
                    borderRadius: '0.5rem',
                    padding: '1rem',
                },
                '.main-card': {
                    minHeight: '40vh',
                },
                '.card-title': {
                    lineHeight: 1,
                },
                '.prescription-contents': {
                    minHeight: '30rem',
                },
            })}
        />
    );
  }