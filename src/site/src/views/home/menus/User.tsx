import React from 'react';
import { Group, UnstyledButton, createStyles, rem, Text, ThemeIcon } from '@mantine/core';
import { IconUser, IconKey, IconClipboardList, IconUsersGroup } from '@tabler/icons-react';
import { Link } from 'react-router-dom';

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

const data = [
    {
        icon: IconClipboardList,
        title: 'Prescriptions',
        description: 'Prescriptions you have access',
        to: "/prescriptions"
    },
    {
        icon: IconKey,
        title: 'Keys',
        description: 'Your keys',
        to: "/keys"
    },
    {
        icon: IconUsersGroup,
        title: 'Groups',
        description: 'Your groups',
        to: "/groups"
    },
    {
        icon: IconUser,
        title: 'Profile',
        description: 'Your profile',
        to: "/user/profile"
    },
];

interface Props {
    onClick?: () => void;
}

export const UserMenu = (props: Props) => {
    const { classes, theme } = useStyles();

    return data.map((item) => (
        <UnstyledButton 
            className={classes.subLink} 
            key={item.title}
        >
            <Link to={item.to} className={classes.url} onClick={props.onClick}>
                <Group noWrap align="flex-start">
                    <ThemeIcon size={34} variant="default" radius="md">
                        <item.icon size={rem(22)} color={theme.fn.primaryColor()} />
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
            </Link>
        </UnstyledButton>
    ));
};
