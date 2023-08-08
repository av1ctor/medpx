import { useContext } from "react";
import { ActorContext } from "../stores/actor";
import { _SERVICE as Main } from "../../../declarations/main/main.did";

interface Actors {
    main: Main;
};

export const useActors = (
): Actors => {
    const [actors, ] = useContext(ActorContext);
    return {
        main: actors.main,
    };
};