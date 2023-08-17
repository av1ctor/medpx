import React, { Fragment } from "react";
import { Button, Card, Center, Text } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { IconKey, IconRefresh } from "@tabler/icons-react";
import { useAuth } from "../../hooks/auth";
import { useKeyFindByUser } from "../../hooks/keys";
import Item from "./Item";

interface Props {
}

const Keys = (props: Props) => {
    const {user} = useAuth();
    
    const query = useKeyFindByUser(user, 10);

    return (
        <Card withBorder radius="md" p="xl" className="card">
            <Text fz="lg" className="card-title" fw={500}>
                <IconKey size="1rem" /> Keys
            </Text>
            <Text fz="xs" c="dimmed" mt={3} mb="xl">
                View your keys
            </Text>
            {query.status === 'success' && query.data && 
                query.data.pages.map((page, index) => 
                    <Fragment key={index}>
                        {page.map(item =>
                            <Item item={item} />
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

export default Keys;