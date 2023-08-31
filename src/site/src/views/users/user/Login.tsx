import React, { useCallback, useEffect } from "react";
import { Text, Center, Group, Stack, ThemeIcon, Title, UnstyledButton, rem, createStyles, Container, Grid } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { useAuth } from "../../../../../site/src/hooks/auth";
import { useUI } from "../../../../../site/src/hooks/ui";
import { ICProviderType } from "../../../../../site/src/interfaces/icprovider";
import { useBrowser } from "../../../hooks/browser";

const useStyles = createStyles((theme) => ({
    subLink: {
        width: '100%',
        padding: `${theme.spacing.xs} ${theme.spacing.md}`,
        borderRadius: theme.radius.md,
    
        ...theme.fn.hover({
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[0],
        }),
    
        '&:active': theme.activeStyles,
    },

    url: {
        color: 'inherit',
        textDecoration: 'none',
    },
}));

interface Props {
    authenticateOnly: boolean;
    onAuthenticated?: () => void;
}

const Login = (props: Props) => {
    const {login, isLogged} = useAuth();
    const {showError, showSuccess, toggleLoading} = useUI();
    const {returnToLastPage} = useBrowser();
    const { classes, theme } = useStyles();

    const handleLogin = useCallback(async (providerType: ICProviderType) => {
        try {
            toggleLoading(true);
            const res = await login(providerType, props.authenticateOnly);
            if(res.Err) {
                showError(res.Err);
            }
            else {
                if(props.onAuthenticated) {
                    props.onAuthenticated();
                    return;
                }
            }
        }
        catch(e) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
    }, [login, isLogged, props.authenticateOnly, props.onAuthenticated]);

    const handleAuthenticateII = useCallback(async () => {
        handleLogin(ICProviderType.InternetIdentity);
    }, [handleLogin]);

    const handleAuthenticatePlug = useCallback(async () => {
        handleLogin(ICProviderType.Plug);
    }, [handleLogin]);
    
    const handleAuthenticateStoic = useCallback(async () => {
        handleLogin(ICProviderType.Stoic);
    }, [handleLogin]);

    useEffect(() => {
        if(isLogged) {
            showSuccess('Welcome back!');
            returnToLastPage();
        }
    }, [isLogged]);

    const data = [
        {
            logo: 'ii.svg',
            w: 64,
            h: undefined,
            color: 'yellow',
            title: 'Internet Identity',
            description: 'Authenticate with Internet Identity',
            onClick: handleAuthenticateII
        },
        {
            logo: 'plug.svg',
            w: undefined,
            h: 64,
            color: 'blue',
            title: 'Plug Wallet',
            description: 'Authenticate with Plug Wallet',
            onClick: handleAuthenticatePlug
        },
    ];

    return (
        <Stack>
            <Center>
                <Title order={4}>
                    <FormattedMessage defaultMessage="Please choose a provider"/>
                </Title>
            </Center>
            <Grid>
                {data.map((item) => 
                    <Grid.Col md={6} key={item.title}>
                        <UnstyledButton 
                            className={classes.subLink} 
                            onClick={item.onClick}
                        >
                            <Group noWrap align="flex-start">
                                <ThemeIcon size={64} variant="filled" color={item.color}  radius="md">
                                    <img src={`/providers/${item.logo}`} height={item.h} width={item.w} />
                                </ThemeIcon>
                                <div>
                                    <Text size="sm" fw={500}>
                                        {item.title}
                                    </Text>
                                    <Text size="xs" color="dimmed">
                                        {item.description}
                                    </Text>
                                </div>
                            </Group>
                        </UnstyledButton>
                    </Grid.Col>
                )}                
            </Grid>
        </Stack>
    );
};

export default Login;