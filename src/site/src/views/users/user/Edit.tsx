import React, { useCallback } from "react";
import { useAuth } from "../../../hooks/auth";
import { userGetKind } from "../../../libs/users";
import PatientEdit from "../../patients/patient/Edit";
import { useUI } from "../../../hooks/ui";
import DoctorEdit from "../../doctors/doctor/Edit";
import ThirdPartyEdit from "../../thirdparties/thirdparty/Edit";

interface Props {
}

const UserEdit = (props: Props) => {
    const {isLogged, user} = useAuth();
    const {showSuccess} = useUI();

    const handleEdited = useCallback((msg: string) => {
        showSuccess(msg);
    }, []);

    if(!isLogged) {
        return null;
    }

    const kind = userGetKind(user);

    return (
        user?.kind? 
            'Patient' in user?.kind?
                <PatientEdit 
                    entity={user?.kind.Patient}
                    onSuccess={handleEdited} 
                />
            :
            'Doctor' in user?.kind?
                <DoctorEdit 
                    entity={user?.kind.Doctor}
                    onSuccess={handleEdited} 
                />
            :
            'ThirdParty' in user?.kind && 
                <ThirdPartyEdit 
                    entity={user?.kind.ThirdParty}
                    onSuccess={handleEdited} 
                />
        :
            null
    );
};

export default UserEdit;