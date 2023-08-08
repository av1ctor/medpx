import React, { useCallback, useContext, useState } from "react";
import { FormattedMessage } from "react-intl";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../../hooks/auth";
import {
    AppShell,
    Navbar,
    Header,
    Footer,
    Text,
    MediaQuery,
    Burger,
    useMantineTheme,
    Button,
  } from '@mantine/core';
import { useUI } from "../../hooks/ui";

interface Props {
}

const Front = (props: Props) => {
    const {showSuccess} = useUI();
    const theme = useMantineTheme();
    
    const [opened, setOpened] = useState(false);
    const navigate = useNavigate();

    const redirectToLogon = useCallback(() => {
        navigate('/user/login');
    }, []);

    const handleClick = () => {
        showSuccess('hey!');
    };

    return (
        <>
            <AppShell
                styles={{
                    main: {
                    background: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
                    },
                }}
                navbarOffsetBreakpoint="sm"
                asideOffsetBreakpoint="sm"
                navbar={
                    <Navbar p="md" hiddenBreakpoint="sm" hidden={!opened} width={{ sm: 200, lg: 300 }}>
                        <Text>Application navbar</Text>
                    </Navbar>
                }
                footer={
                    <Footer height={60} p="md">
                        Application footer
                    </Footer>
                }
                header={
                    <Header height={{ base: 50, md: 70 }} p="md">
                        <div style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                            <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
                                <Burger
                                    opened={opened}
                                    onClick={() => setOpened((o) => !o)}
                                    size="sm"
                                    color={theme.colors.gray[6]}
                                    mr="xl"
                                />
                            </MediaQuery>

                            <Text>Application header</Text>
                        </div>
                    </Header>
                }
            >
                <Button onClick={handleClick}></Button>
                <Text>Resize app to see responsive navbar in action</Text>
            </AppShell>
        </>
    );
};

export default Front;