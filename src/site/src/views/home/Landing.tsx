import React, { useCallback } from "react";
import { createStyles, Image, Container, Title, Button, Group, Text, List, ThemeIcon, rem, Flex } from '@mantine/core';
import { IconBrandGithub, IconCheck, IconUserPlus } from '@tabler/icons-react';
import { Link } from "react-router-dom";
import { useBrowser } from "../../hooks/browser";
  
const useStyles = createStyles((theme) => ({
    inner: {
        display: 'flex',
        flexWrap: 'wrap'
    },

    content: {
        maxWidth: rem(480),
        marginRight: `calc(${theme.spacing.xl} * 3)`,

        [theme.fn.smallerThan('md')]: {
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
  
    imageContainer: {
        flex: 1,
        display: 'flex',
        justifyContent: 'center',
    },

    image: {
        margin: 'auto',
    },
  
    highlight: {
        position: 'relative',
        backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
        borderRadius: theme.radius.md,
        padding: '0.5rem 1rem',
        lineHeight: '4rem',
    },
}));

const Landing = () => {
    const { classes } = useStyles();
    const {redirectToSignup} = useBrowser();

    const handleViewSourceCode = useCallback(() => {
        window.open("https://github.com/av1ctor/medpx", "blank");
    }, []);

    const handleSignup = useCallback(() => {
        redirectToSignup();
    }, []);

    return (
        <Container>
            <div className={classes.inner}>
                <div className={classes.content}>
                    <Title className={classes.title}>
                        Keep your <span className={classes.highlight}>medical prescriptions</span> safe, under your control
                    </Title>
                    <Text color="dimmed" mt="md" className={classes.subtitle}>
                        Medpx stores your medical prescriptions in eletronic form, on the blockchain, using cryptography to ensure nobody can access them – unless you allow it!
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
                            <b>Encrypted</b> – leveraging the <Link to="https://internetcomputer.org/blog/features/vetkey-primer" target="blank"><b>VetKeys</b></Link> technology, present only on the Internet Computer, nobody can access your prescriptions, not even the host provider
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Shareable</b> – You can share your prescriptions with other users, for example a Hospital, Drug store or another doctor, or create groups, allowing any users that are members of the group to access your data 
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Time-locked</b> – When sharing your prescriptions, you can define a date limit to cut the access to your data
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Easy to find</b> – You can create keys, like your e-mail, your phone number, etc, so you can be found more easily by other users (no need to keep tracking of long and cumbersome wallet addresses)
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>On-chain</b> – Medpx is a decentralized web3 app, running 100% on-chain on the Internet Computer, giving you full control of your prescriptions
                        </List.Item>
                        <List.Item className={classes.item}>
                            <b>Open-source</b> – released under MIT license, anyone can verify the source code and contribute to the app
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
                <div className={classes.imageContainer}>
                    <Image src="logo.svg" className={classes.image} />
                </div>
            </div>
        </Container>
    );
};

export default Landing;