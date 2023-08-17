import { Anchor, Center, Container, Divider, Footer, Grid, Space, Stack } from '@mantine/core';
import { IconBrandGithubFilled, IconBrandTwitterFilled } from '@tabler/icons-react';
import React from 'react';
import { FormattedMessage } from 'react-intl';
import { Link } from 'react-router-dom';

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
        </Container>
    );
}

export default AppFooter;