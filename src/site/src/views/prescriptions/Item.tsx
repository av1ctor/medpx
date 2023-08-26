import React, { useCallback } from "react";
import { ActionIcon, Group, Skeleton, Text } from "@mantine/core";
import { IconClockHour4, IconShare, IconStethoscope, IconTrash, IconVaccine } from "@tabler/icons-react";
import { PrescriptionResponse, UserResponse } from "../../../../declarations/main/main.did";
import { useAuth } from "../../hooks/auth";
import { userIsKind } from "../../libs/users";
import { useUserFindById } from "../../hooks/users";
import TimeFromNow from "../../components/TimeFromNow";

interface Props {
    item: PrescriptionResponse
    onView: (item: PrescriptionResponse) => void;
    onShare: (item: PrescriptionResponse) => void;
    onDelete: (item: PrescriptionResponse) => void;
}

const Patient = (props: {user: UserResponse|undefined}) => 
    <span>
        {!props.user? 
            <Skeleton h="1rem" w="10rem"></Skeleton>
        :
            <span><IconVaccine size="0.75rem"/> Patient: {props.user.name}</span>
        }
    </span>;

const Doctor = (props: {user: UserResponse|undefined}) => 
    <span>
        {!props.user? 
            <Skeleton h="1rem" w="10rem"></Skeleton>
        :
            <span><IconStethoscope size="0.75rem"/> Doctor: {props.user.name}</span>
        }
    </span>;

const Item = (props: Props) => {
    const {user} = useAuth();
    const doctorq = useUserFindById(props.item.doctor);
    const patientq = useUserFindById(props.item.patient);

    const handleView = useCallback(() => {
        props.onView(props.item);
    }, [props.item]);

    const handleShare = useCallback(() => {
        props.onShare(props.item);
    }, [props.item]);

    const handleDelete = useCallback(async () => {
        props.onDelete(props.item);
    }, [props.item]);
    
    const {item} = props;
    const isDoctor = userIsKind(user, 'Doctor');
    const isPatient = userIsKind(user, 'Patient');
    const isThirdParty = userIsKind(user, 'ThirdParty');
    
    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div className="clickable" onClick={handleView}>
                <Text>{item.id}</Text>
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
                <Text size="xs" color="dimmed">
                    {isThirdParty?
                        <>
                            <Doctor user={doctorq.data} /><br/>
                            <Patient user={patientq.data} />
                        </>
                        :
                        isDoctor?
                            <Patient user={patientq.data} />
                        :
                            <Doctor user={doctorq.data} />
                    }
                </Text>
            </div>
            {isPatient &&
                <>
                    <ActionIcon
                        variant="filled"
                        color="blue"
                        title="Share"
                        onClick={handleShare}
                    >
                        <IconShare size="1rem" />
                    </ActionIcon>
                    <ActionIcon
                        variant="filled"
                        color="red"
                        title="Delete"
                        onClick={handleDelete}
                    >
                        <IconTrash size="1rem" />
                    </ActionIcon>
                </>
            }
        </Group>
    )
};

export default Item;