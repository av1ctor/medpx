import React from 'react';
import { ActionIcon, Avatar, Box, Burger, Center, Collapse, Divider, Drawer, Group, Header, HoverCard, ScrollArea, UnstyledButton, createStyles, rem, useMantineColorScheme, Text } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IconChevronDown, IconHome2, IconMoonStars, IconSun } from '@tabler/icons-react';
import { AuthMenu } from './menus/Auth';
import { UserMenu } from './menus/User';
import { useAuth } from '../../hooks/auth';

const useStyles = createStyles((theme) => ({
    link: {
        display: 'flex',
        alignItems: 'center',
        height: '100%',
        paddingLeft: theme.spacing.md,
        paddingRight: theme.spacing.md,
        textDecoration: 'none',
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
        fontWeight: 500,
        fontSize: theme.fontSizes.sm,
    
        [theme.fn.smallerThan('sm')]: {
            height: rem(42),
            display: 'flex',
            alignItems: 'center',
            width: '100%',
        },
    
        ...theme.fn.hover({
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
        }),
    },
  
    hiddenMobile: {
        [theme.fn.smallerThan('sm')]: {
            display: 'none',
        },
    },
  
    hiddenDesktop: {
        [theme.fn.largerThan('sm')]: {
            display: 'none',
        },
    },
}));
  
interface Props {
}

const ToggleThemeButton = () => {
    const { colorScheme, toggleColorScheme } = useMantineColorScheme();
    const dark = colorScheme === 'dark';

    return (
        <ActionIcon
            variant="outline"
            color={dark ? 'yellow' : 'blue'}
            onClick={() => toggleColorScheme()}
            title="Toggle color scheme"
            >
            {dark ? <IconSun size="1.1rem" /> : <IconMoonStars size="1.1rem" />}
        </ActionIcon>
    );
};

const AppHeader = (props: Props) => {
    const {isLogged, user} = useAuth();
    const [drawerOpened, { toggle: toggleDrawer, close: closeDrawer }] = useDisclosure(false);
    const [linksOpened, { toggle: toggleLinks }] = useDisclosure(false);
    const { classes, theme } = useStyles();

    return (
        <Box pb={120}>
            <Header height={60} px="md">
                <Group position="apart" sx={{ height: '100%' }}>
                    <Group sx={{ height: '100%' }} spacing={0} className={classes.hiddenMobile}>
                        <img src="/medpx-logo.svg" />

                        <a href="#" className={classes.link}>
                            <IconHome2 size="1rem" />&nbsp;Home
                        </a>
                        
                        {user && 
                            <HoverCard width={600} position="bottom" radius="md" shadow="md" withinPortal>
                                <HoverCard.Target>
                                    <a href="#" className={classes.link}>
                                        <Center inline>
                                            <Avatar color="cyan" radius="xl" size="sm">
                                                {user.name.substr(0, 1).toUpperCase()}
                                            </Avatar>
                                            &nbsp;<Text fw={500} size="xs">{user.name}</Text>&nbsp;
                                            <IconChevronDown size={16} color={theme.fn.primaryColor()} />
                                        </Center>
                                    </a>
                                </HoverCard.Target>

                                <HoverCard.Dropdown sx={{ overflow: 'hidden' }}>
                                    <UserMenu />
                                </HoverCard.Dropdown>
                            </HoverCard>
                        }
                    </Group>

                    <Group className={classes.hiddenMobile}>
                        <ToggleThemeButton />
                        <AuthMenu />
                    </Group>
                    
                    <Burger opened={drawerOpened} onClick={toggleDrawer} className={classes.hiddenDesktop} />
                </Group>
            </Header>

            <Drawer
                opened={drawerOpened}
                onClose={closeDrawer}
                size="100%"
                padding="md"
                title={<img src="/medpx-logo.svg" />}
                className={classes.hiddenDesktop}
                zIndex={1000000}
            >
                <ScrollArea h={`calc(100vh - ${rem(60)})`} mx="-md">
                    <Divider my="sm" color={theme.colorScheme === 'dark' ? 'dark.5' : 'gray.1'} />
        
                    <a href="#" className={classes.link} onClick={closeDrawer}>
                        <IconHome2 size="1rem" />&nbsp;Home
                    </a>
                    
                    {user && 
                        <>
                            <UnstyledButton className={classes.link} onClick={toggleLinks}>
                                <Center inline>
                                    <Avatar color="cyan" radius="xl" size="sm">
                                        {user.name.substr(0, 1).toUpperCase()}
                                    </Avatar>
                                    &nbsp;<Text fw={500} size="xs">{user.name}</Text>&nbsp;
                                    <IconChevronDown size={16} color={theme.fn.primaryColor()} />
                                </Center>
                            </UnstyledButton>
                            <Collapse in={linksOpened}>
                                <UserMenu onClick={closeDrawer} />
                            </Collapse>
                        </>
                    }
        
                    <Divider my="sm" color={theme.colorScheme === 'dark' ? 'dark.5' : 'gray.1'} />
        
                    <Group className={classes.hiddenDesktop} position="center" grow pb="xl" px="md">
                        <div>
                            <ToggleThemeButton />
                        </div>
                        <AuthMenu onClick={closeDrawer} />
                    </Group>
                </ScrollArea>
            </Drawer>
        </Box>
    );
}

export default AppHeader;