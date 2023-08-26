import React, { forwardRef, useCallback, useMemo, useState } from "react";
import { Button, Group, Select, SelectItem, Space, Text } from "@mantine/core";
import { GroupResponse } from "../../../../../declarations/main/main.did";
import { useAuth } from "../../../hooks/auth";
import { useGroupFindByUser } from "../../../hooks/groups";
import { GroupMembers } from "../Item";
import { principalToString } from "../../../libs/icp";

interface Props {
    onChange: (user: GroupResponse|undefined) => void;
}

interface ItemProps extends React.ComponentPropsWithoutRef<'div'> {
    value: string;
    label: string;
    item: GroupResponse;
};
  
const GroupItem = forwardRef<HTMLDivElement, ItemProps>(
    ({ value, label, item, ...others }: ItemProps, ref) => (
        <div ref={ref} {...others}>
            <Group noWrap>
                <div>
                    <Text size="sm">{item.id}</Text>
                    <Text size="xs" opacity={0.65}>
                        <GroupMembers members={item.members} />
                    </Text>
                </div>
            </Group>
        </div>
    )
);

const ChooseGroup = (props: Props) => {
    const {user} = useAuth();
    const groups = useGroupFindByUser(user, 32);
    const [group, setGroup] = useState<GroupResponse|undefined>();

    const data = useMemo(() => {
        if(!groups.data) {
            return [];
        }
        return groups.data.pages.flatMap(g => g).map(g => ({
            value: g.id,
            label: g.members.map(id => principalToString(id)).join(', '),
            item: g,
        }))
    }, [groups.data]);
    
    const handleChange = useCallback((value: string) => {
        setGroup(data.find(g => g.value === value)?.item);
    }, [data]);
    
    const handleSelect = useCallback(() => {
        props.onChange(group)
    }, [props.onChange, group]);
    
    const handleFilter = useCallback((value: string, item: SelectItem) => {
        return item.value.toLowerCase().includes(value.toLowerCase().trim());
    }, []);

    return (
        <>
            <Select
                label="Choose a group"
                placeholder="Group"
                itemComponent={GroupItem}
                data={data}
                value={group?.id}
                searchable
                maxDropdownHeight={400}
                nothingFound="Not found"
                filter={handleFilter}
                onChange={handleChange}
            />

            <Space h="md" />

            <Button
                variant="filled" 
                color="green"
                disabled={!group}
                fullWidth
                onClick={handleSelect}
            >
                Select
            </Button>
        </>
    );
};

export default ChooseGroup;