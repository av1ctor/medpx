import React, { ReactElement, useEffect, useMemo } from "react";
import { Anchor, Badge, Center, Container, Divider, Grid, Skeleton, Space, Text } from "@mantine/core";
import QRCode from "react-qr-code";
import { PrescriptionResponse } from "../../../../../declarations/main/main.did";
import { useUserFindById } from "../../../hooks/users";
import { config } from "../../../config";
import { useDecrypt } from "../../../hooks/crypto";
import { useUI } from "../../../hooks/ui";
import { userGetDoctor } from "../../../libs/users";
import { UserAvatar } from "../../../components/UserAvatar";
import { principalToString } from "../../../libs/icp";
import { Link } from "react-router-dom";

interface Props {
    item: PrescriptionResponse;
    isEncrypted: boolean;
}

const PrescriptionView = (props: Props) => {
    const {showError} = useUI()
    const doctorq = useUserFindById(props.item.doctor);
    const patientq = useUserFindById(props.item.patient);
    
    const dec = useDecrypt(
        (props.item?.contents as Uint8Array) || new Uint8Array(), 
        props.item.patient, 
        Number(props.isEncrypted)
    );

    useEffect(() => {
        if(dec.Err) {
            showError(dec.Err);
        }
    }, [dec.Err]);
    
    const {item} = props;

    const url = `${config.APP_URL}/#/p/${item.id}`;

    const rowsSkeleton = useMemo(() => {
        const rows = new Array<ReactElement>();

        for(let i = 0; i < 14; i++)
            rows.push(<Skeleton key={i} h="1rem" w="100%" mb="1rem" />);

        return rows;
    }, []);

    const doctor = userGetDoctor(doctorq.data);

    return (
        <div>
            <Center>
                <Text size="1.5rem" fw={500} color="blue" >
                    {doctorq.data?.name}
                </Text>
            </Center>
            <Center>
                <Text size=".75rem" color="blue" >
                    License number: {doctor?.license_num}
                </Text>
            </Center>
            <Center>
                <Text size=".75rem" color="blue" >
                    Id:&nbsp;
                    <Badge size="sm">
                        <Link target="blank" to={`/user/${doctorq.data?.id}`}>
                            {principalToString(doctorq.data?.id)}
                        </Link>
                    </Badge>
                </Text>
            </Center>

            <Divider mt="1rem" />

            <Container p="1rem">
                <div>
                    <Text><u>Patient</u></Text>
                    <UserAvatar user={patientq.data} />
                </div>

                <Space h="2rem" />

                <div className="prescription-contents">
                    <Text><u>Prescription</u></Text>
                    {dec.Ok?
                        <div>
                            {dec.Ok}
                        </div>
                    :
                        <div>
                            {rowsSkeleton}
                            <Skeleton h="1rem" w="100%" />
                        </div>
                    }
                </div>

                <Space h="2rem" />

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
                                Digitally created and signed by <b>{doctorq.data?.name}</b>, license <b>{doctor?.license_num}</b>, at {new Date(Number(item.created_at / 1000000n)).toISOString()}<br/>
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