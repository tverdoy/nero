import {useEffect, useState} from 'react';
import {useNavigate, useParams} from "react-router-dom";
import {useActionsApps} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import RequestWrap from "../components/RequestWrap.tsx";
import ModelTable from "../components/ModelTable.tsx";
import {RouteNames} from "../route.ts";

const Model = () => {
    const {appName, modelName} = useParams();
    const {request} = useActionsApps();
    const {apps, isLoading, error} = useTypedSelector(state => state.appsReducer);
    const {token} = useTypedSelector(state => state.authReducer);
    const navigate = useNavigate();

    const findModel = () => {
        const app = apps ? apps.find(a => a.name === appName) : undefined;
        return app ? app.models.find(model => model.scheme.name === modelName) : undefined
    }

    const [model, modelSet] = useState(findModel())

    useEffect(() => {
        request(token)
    }, [])

    useEffect(() => {
        modelSet(findModel())
    }, [apps])

    const onClickRecord = (recordId: string) => {
        navigate(RouteNames.RECORD.replace(':appName', appName || "").replace(':modelName', modelName || "").replace(':recordId', recordId))
    }

    return (
        <RequestWrap isLoading={isLoading} error={error} object={model}>
            { model ?
                <div style={{ height: "75vh" }} className={"overflow-hidden "}>
                    <ModelTable model={model} onClickRecord={onClickRecord}/>
                </div>
                :
                <div>Error</div>

            }
        </RequestWrap>
    );
};

export default Model;