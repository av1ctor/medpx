import React from 'react';
import { Anchor, Center, Container, Divider, Grid, Space, Stack, Text } from '@mantine/core';
import { IconBrandGithubFilled, IconBrandTwitterFilled } from '@tabler/icons-react';
import { FormattedMessage } from 'react-intl';

interface Props {
}

const AppFooter = (props: Props) => {
    
    const year = new Date().getFullYear();
    
    return (
        <Container pt="10rem">
            <Divider pb="xl" />
            <Grid>
                <Grid.Col span={12}>
                    <Center>
                        <a href="https://dfinity.org" target="_blank">
                            <img src="/powered-by.svg" />
                        </a>
                    </Center>
                </Grid.Col>
            </Grid>
            <Grid pt="md">
                <Grid.Col span={4}>
                    <Stack>
                        <Anchor href="#/about">
                            <FormattedMessage id="Aboutus" defaultMessage="About us"/>
                        </Anchor>
                        <Anchor href="#/jobs">
                            <FormattedMessage id="Jobs" defaultMessage="Jobs" />
                        </Anchor>
                    </Stack>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Stack>
                        <Anchor href="#/policies">
                            <FormattedMessage id="Policies" defaultMessage="Policies" />
                        </Anchor>
                        <Anchor href="#/privacy">
                            <FormattedMessage id="Privacy" defaultMessage="Privacy" />
                        </Anchor>
                    </Stack>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Grid>
                        <Grid.Col span={6}>
                            <Anchor 
                                href="https://twitter.com/medpx_app"
                                target="_blank"
                            >
                                <IconBrandTwitterFilled size="2rem" />
                            </Anchor>
                        </Grid.Col>
                        <Grid.Col span={6}>
                            <Anchor
                                href="https://github.com/av1ctor/medpx"
                                target="_blank"
                            >
                                <IconBrandGithubFilled size="2rem" />
                            </Anchor>
                        </Grid.Col>
                    </Grid>
                </Grid.Col>
            </Grid>
            <Space />
            <Center>
                © 2023-{year} medpx
            </Center>
            <Center>
                <Text color='pink' size="xs">Notice: this is a demo site and shouldn't not be used to store real/private data</Text>
            </Center>
        </Container>
    );
}

export default AppFooter;