import React, { useCallback } from "react";
import { Group, Text } from "@mantine/core";
import { PrescriptionResponse } from "../../../../declarations/main/main.did";
import { useAuth } from "../../hooks/auth";
import { userIsKind } from "../../libs/users";
import { useDoctorFindById } from "../../hooks/doctors";
import { usePatientFindById } from "../../hooks/patients";
import { IconClockHour4, IconStethoscope, IconVaccine } from "@tabler/icons-react";
import TimeFromNow from "../../components/TimeFromNow";

interface Props {
    item: PrescriptionResponse
    onView: (item: PrescriptionResponse) => void;
}

const Item = (props: Props) => {
    const {user} = useAuth();
    const doctor = useDoctorFindById(props.item.doctor);
    const patient = usePatientFindById(props.item.patient);

    const handleView = useCallback(() => {
        props.onView(props.item);
    }, [props.item]);
    
    const {item} = props;
    const isDoctor = userIsKind(user, 'Doctor');
    
    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div className="clickable" onClick={handleView}>
                <Text>{item.id}</Text>
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
                <Text size="xs" color="dimmed">
                    {isDoctor?
                        <span><IconVaccine size="0.75rem"/> {patient.data?.name}</span>:
                        <span><IconStethoscope size="0.75rem"/> {doctor.data?.name}</span>
                    }
                </Text>
            </div>
        </Group>
    )
};

export default Item;