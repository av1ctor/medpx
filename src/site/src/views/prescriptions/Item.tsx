import React from "react";
import { Group, Text, createStyles, rem } from "@mantine/core";
import { PrescriptionResponse } from "../../../../declarations/main/main.did";
import { useAuth } from "../../hooks/auth";
import { userIsKind } from "../../libs/users";
import { useDoctorFindById } from "../../hooks/doctors";
import { usePatientFindById } from "../../hooks/patients";

interface Props {
    item: PrescriptionResponse
}

const Item = (props: Props) => {
    const {user} = useAuth();
    const doctor = useDoctorFindById(props.item.doctor);
    const patient = usePatientFindById(props.item.patient);
    
    const {item} = props;
    const isDoctor = userIsKind(user, 'Doctor');
    
    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{item.id}</Text>
                <Text size="xs" color="dimmed">
                    {isDoctor?
                        patient.data?.name:
                        doctor.data?.name
                    }
                </Text>
            </div>
        </Group>
    )
};

export default Item;