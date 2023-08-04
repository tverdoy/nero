import {useEffect, useState} from 'react';
import {useParams} from "react-router-dom";
import {useActionsApps} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import RequestWrap from "../components/RequestWrap.tsx";

const Model = () => {
    const {appName, modelName} = useParams();
    const {request} = useActionsApps();
    const {apps, isLoading, error} = useTypedSelector(state => state.appsReducer);
    const {token} = useTypedSelector(state => state.authReducer);

    const findModel = () => {
        const app = apps ? apps.find(a => a.name === appName) : undefined;
        return app ? app.schemes.find(scheme => scheme.name === modelName) : undefined
    }

    const [model, modelSet] = useState(findModel())

    useEffect(() => {
        request(token)
    }, [])

    useEffect(() => {
        modelSet(findModel())
    }, [apps])

    return (
        <RequestWrap isLoading={isLoading} error={error} object={model}>
            <>{model?.name}</>
        </RequestWrap>
    );
};

export default Model;