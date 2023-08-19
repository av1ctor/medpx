import React, { useMemo } from "react";
import { Anchor, Center, Container, Divider, Flex, Grid, NavLink, Space, Text } from "@mantine/core";
import { PrescriptionResponse } from "../../../../../declarations/main/main.did";
import { useDoctorFindById } from "../../../hooks/doctors";
import { usePatientFindById } from "../../../hooks/patients";
import QRCode from "react-qr-code";
import { config } from "../../../config";

interface Props {
    item: PrescriptionResponse;
}

const PrescriptionView = (props: Props) => {

    const doctor = useDoctorFindById(props.item.doctor);
    const patient = usePatientFindById(props.item.patient);

    const contents = useMemo(() => {
        return new TextDecoder().decode((props.item?.contents as Uint8Array) || new Uint8Array());
    }, [props.item]);

    const {item} = props;

    const url = `${config.APP_URL}/#/p/${item.id}`;
    
    return (
        <div>
            <Center>
                <Text size="1.5rem" fw={500} color="blue" >
                    {doctor.data?.name}
                </Text>
            </Center>
            <Center>
                <Text size=".75rem" color="blue" >
                    License number: {doctor.data?.license_num}
                </Text>
            </Center>
            <Center>
                <Text size=".75rem" color="blue" >
                    Id: {doctor.data?.id.toString()}
                </Text>
            </Center>

            <Divider mt="1rem" />

            <Container p="1rem">
                <Flex direction="column">
                    <div><b>Name:</b> {patient.data?.name}</div>
                    <div><b>Id:</b> {patient.data?.id.toString()}</div>
                </Flex>

                <Space h="2rem" />

                {contents}

                <Space h="30rem" />

                <Divider />

                <div style={{ background: 'white', padding: '16px' }}>
                    <Grid>
                        <Grid.Col md={3} sm={12}>
                            <QRCode 
                                value={url} 
                                size={128}
                            />
                        </Grid.Col>
                        <Grid.Col md={9} sm={12}>
                            <Text size="sm">
                                This prescription can be verified at <Anchor href={url} target="_blank">{url}</Anchor><br/>
                                Digitally created and signed by <b>{doctor.data?.name}</b>, license <b>{doctor.data?.license_num}</b>, at {new Date(Number(item.created_at / 1000000n)).toISOString()}<br/>
                                <img src="/medpx-logo.svg" />
                            </Text>
                        </Grid.Col>
                    </Grid>
                </div>
            </Container>

            
        </div>
    );
};

export default PrescriptionView;