import React, { Fragment } from "react";
import { Button, Card, Center, Divider, Group, Text } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { IconClipboardList, IconRefresh } from "@tabler/icons-react";
import { useAuth } from "../../hooks/auth";
import { usePrescriptionsFind } from "../../hooks/prescriptions";
import Item from "./Item";

interface Props {
}

const Prescriptions = (props: Props) => {
    const {user} = useAuth();
    
    const query = usePrescriptionsFind(user, 10);

    return (
        <Card withBorder radius="md" p="xl" className="card">
            <Group position="apart" noWrap spacing="xl">
                <div>
                    <Text fz="lg" className="card-title" fw={500}>
                        <IconClipboardList size="1rem" /> Prescriptions
                    </Text>
                    <Text fz="xs" c="dimmed" mt={3} mb="xl">
                        View your prescriptions
                    </Text>
                </div>
            </Group>
            
            <Divider pb="md" />

            {query.status === 'success' && query.data && 
                query.data.pages.map((page, index) => 
                    <Fragment key={index}>
                        {page.map(item =>
                            <Item key={item.id} item={item} />
                        )}
                    </Fragment>
                )
            }
             
            <Center>
                <Button
                    disabled={!query.hasNextPage || query.isFetchingNextPage}
                    onClick={() => query.fetchNextPage()}
                >
                    <IconRefresh />&nbsp;{<FormattedMessage id={query.hasNextPage? 'Load more': 'All loaded'} defaultMessage={query.hasNextPage? 'Load more': 'All loaded'}/>}
                </Button>
            </Center>
        </Card>
    );
};

export default Prescriptions;