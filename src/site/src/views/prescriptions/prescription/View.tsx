import React, { ReactElement, useCallback, useEffect, useMemo, useState } from "react";
import { Anchor, Badge, Center, Container, Divider, Grid, Skeleton, Space, Text } from "@mantine/core";
import QRCode from "react-qr-code";
import { Link } from "react-router-dom";
import { PrescriptionResponse } from "../../../../../declarations/main/main.did";
import { useUserFindById } from "../../../hooks/users";
import { config } from "../../../config";
import { userGetDoctor } from "../../../libs/users";
import { UserAvatar } from "../../../components/UserAvatar";
import { principalToString } from "../../../libs/icp";
import { useAuth } from "../../../hooks/auth";

interface Props {
    item: PrescriptionResponse;
    isEncrypted: boolean;
}

const PrescriptionView = (props: Props) => {
    const {aes_gcm} = useAuth();
    const doctorq = useUserFindById(props.item.doctor);
    const patientq = useUserFindById(props.item.patient);
    const [text, setText] = useState<string|undefined>();
    const [err, setErr] = useState<string|undefined>();
        
    const decrypt = useCallback(async () => {
        const contents = props.item.contents as Uint8Array;

        if(!props.isEncrypted) {
            setText(new TextDecoder().decode(contents));
            return;
        }

        if(!aes_gcm) {
            return;
        }
    
        const rawKey = await aes_gcm.genRawKey(props.item);
        if('Err' in rawKey || !rawKey.Ok) {
            setErr('Raw key generation failed');
            return;
        }
        
        try {
            setText(await aes_gcm.decrypt(contents, rawKey.Ok));
        }
        catch(e: any) {
            setErr(e.message || "Call to AES GCM decrypt failed");
        }
    }, [aes_gcm]);
    
    useEffect(() => {
        decrypt();
    }, [props.item]);

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
                    
                    {err?
                        <div>
                            Error: {err}
                        </div>
                    :   
                        text?
                            <div>
                                {text}
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
                                Digitally created and signed by <b>{doctorq.data?.name}</b>, license <b>{doctor?.license_num}</b>, at <b>{new Date(Number(item.created_at / 1000000n)).toISOString()}</b>, with hash <small><b>{Buffer.from(item.hash).toString('hex')}</b></small><br/>
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