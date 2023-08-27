import React, { useCallback } from "react";
import { createStyles, Image, Container, Title, Button, Group, Text, List, ThemeIcon, rem } from '@mantine/core';
import { IconBrandGithub, IconCheck, IconUserPlus } from '@tabler/icons-react';
import { Link } from "react-router-dom";
import { useBrowser } from "../../hooks/browser";
  
const useStyles = createStyles((theme) => ({
    inner: {
        display: 'flex',
        justifyContent: 'space-between',
    },

    content: {
        maxWidth: rem(480),
        marginRight: `calc(${theme.spacing.xl} * 3)`,

        [theme.fn.smallerThan('md')]: {
            maxWidth: '100%',
            marginRight: 0,
        },
    },
  
    title: {
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
        fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        fontSize: rem(42),
        lineHeight: 1.2,
        fontWeight: 900,
        textAlign: 'justify',

        [theme.fn.smallerThan('xs')]: {
            fontSize: rem(28),
        },
    },

    subtitle: {
        textAlign: 'justify',
    },

    item: {
        textAlign: 'justify',
    },
  
    control: {
        [theme.fn.smallerThan('xs')]: {
            flex: 1,
        },
    },
  
    image: {
        flex: 1,
        [theme.fn.smallerThan('md')]: {
            display: 'none',
        },
    },
  
    highlight: {
        position: 'relative',
        backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
        borderRadius: theme.radius.md,
        padding: `${rem(4)} ${rem(12)}`,
    },
}));

const Landing = () => {
    const { classes } = useStyles();
    const {redirectToSignup, navigateTo} = useBrowser();

    const handleViewSourceCode = useCallback(() => {
        navigateTo("https://github.com/av1ctor/medpx");
    }, []);

    const handleSignup = useCallback(() => {
        redirectToSignup();
    }, []);

    return (
        <Container>
            <div className={classes.inner}>
                <div className={classes.content}>
                    <Title className={classes.title}>
                        Your <span className={classes.highlight}>health data</span> safe, under your control
                    </Title>
                    <Text color="dimmed" mt="md" className={classes.subtitle}>
                        Medpx keeps your medical prescriptions on the blockchain using
                        cryptography to ensure nobody can access them – unless you allow it!
                    </Text>

                    <List
                        mt={30}
                        spacing="sm"
                        size="sm"
                        icon={
                            <ThemeIcon size={20} radius="xl">
                                <IconCheck size={rem(12)} stroke={1.5} />
                            </ThemeIcon>
                        }
                    >
                        <List.Item className={classes.item}>
                            <b>Cryptography</b> – leveraging the <Link to="https://internetcomputer.org/blog/features/vetkey-primer" target="blanck">VetKD</Link> technology, present only on the Internet Computer, nobody can access your prescriptions, not even the host provider
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Shareable</b> – You can share your prescriptions with other users, for example a Hospital, Drug store or another doctor, or create groups, allowing any users that are part of them to access your data 
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Time-locked</b> – When sharing your prescriptions, you can define a date limit to cut the access to your data
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>On-chain</b> – Medpx is a decentralized web3 app, running 100% on-chain on the Internet Computer, giving you full control of your prescriptions
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Open-source</b> – released under MIT license, anyone can check the source code and contribute to the app
                        </List.Item>
                    </List>

                    <Group mt={30}>
                        <Button 
                            radius="xl" 
                            size="md" 
                            className={classes.control}
                            onClick={handleSignup}
                        >
                            <IconUserPlus size="1rem" />&nbsp;Create an account
                        </Button>
                        <Button 
                            variant="default" 
                            radius="xl" 
                            size="md" 
                            className={classes.control}
                            onClick={handleViewSourceCode}
                        >
                            <IconBrandGithub size="1rem" />&nbsp;View source code
                        </Button>
                    </Group>
                </div>
                <div className={classes.image}>
                    <Image src="medpx-logo.svg" />
                </div>
            </div>
        </Container>
    );
};

export default Landing;