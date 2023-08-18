import React, { useMemo } from "react";
import { Container, Textarea } from "@mantine/core";
import { PrescriptionResponse } from "../../../../../declarations/main/main.did";

interface Props {
    item: PrescriptionResponse | undefined;
}

const PrescriptionView = (props: Props) => {

    const contents = useMemo(() => {
        return new TextDecoder().decode((props.item?.contents as Uint8Array) || new Uint8Array());
    }, [props.item]);
    
    return (
        <Container>
            <Textarea
                label="Contents"
                placeholder="Contents"
                minRows={20}
                value={contents}
            />
        </Container>
    );
};

export default PrescriptionView;