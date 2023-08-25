import React, { useCallback } from "react";
import { IconClockHour4, IconTrash } from "@tabler/icons-react";
import { ActionIcon, Group, Text } from "@mantine/core";
import { PrescriptionAuthResponse } from "../../../../../declarations/main/main.did";
import { prescriptionAuthGetKind } from "../../../libs/prescription_auths";
import TimeFromNow from "../../../components/TimeFromNow";
import { useThirdPartyFindById } from "../../../hooks/thirdparty";
import { thirdPartyGetKind } from "../../../libs/thirdparties";

interface Props {
    item: PrescriptionAuthResponse;
    onDelete: (item: PrescriptionAuthResponse) => void;
}

const Item = (props: Props) => {
    const thirdparty = useThirdPartyFindById(props.item.to);
    
    const handleDelete = useCallback(async () => {
        props.onDelete(props.item);
    }, [props.item]);

    const {item} = props;

    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{thirdparty.data?.name}</Text>
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
                <Text size="xs" color="dimmed">
                    {thirdparty.data && thirdPartyGetKind(thirdparty.data.kind).label} /&nbsp;
                    {prescriptionAuthGetKind(item.kind).label} /&nbsp;
                    {item.expires_at.length > 0? `Expires at: ${new Date(Number((item.expires_at[0] as bigint) / 1000000n)).toISOString()}`: 'Never expires'}
                </Text>
            </div>
            <ActionIcon
                variant="filled"
                color="red"
                title="Delete"
                onClick={handleDelete}
            >
                <IconTrash size="1rem" />
            </ActionIcon>
        </Group>
    )
};

export default Item;