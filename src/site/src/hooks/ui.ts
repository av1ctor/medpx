import { useCallback, useContext } from "react";
import { UIActionType, UIContext } from "../stores/ui";
import { notifications } from '@mantine/notifications';

interface UIProps {
    isLoading: boolean;
    toggleLoading: (to: boolean) => void;
    showSuccess: (text: string) => void
    showError: (e: any) => void;
};

const showError = (e: any) => {
    if(e) {
        const text = typeof e === 'string'? 
            e
        :
            e.constructor === Array?
                e.map((s, i) => `${1+i}. ${s};`) .join('\n')
            :
                typeof e === 'object'?
                    'data' in e?
                        e.data.message
                    :
                        e.message
                :
                    '';
        
        
        notifications.show({
            title: 'Error',
            message: `Error${e.constructor === Array? 's:\n': ': '}${text}`,
            color: 'red',
            autoClose: 10000,
        });
    }
};

const showSuccess = (text: string) => {
    notifications.show({
        title: 'Success',
        message: text,
        color: 'green',
        autoClose: 5000,
    });
}

export const useUI = (): UIProps => {
    const [state, dispatch] = useContext(UIContext);

    const toggleLoading = useCallback((to: boolean) => {
        dispatch({
            type: UIActionType.TOGGLE,
            payload: to
        });
    }, []);
    
    return {
        isLoading: state.isLoading,
        toggleLoading,
        showSuccess,
        showError,
    };
};
