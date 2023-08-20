import {useEffect, useState} from 'react';
import {useNavigate, useParams} from "react-router-dom";
import {useActionsApps, useActionsRecord} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import RequestWrap from "../components/RequestWrap.tsx";
import ModelTable from "../components/ModelTable.tsx";
import {RouteNames} from "../route.ts";

const Model = () => {
    const {appName, modelName} = useParams();
    const navigate = useNavigate();


    const actionsApps = useActionsApps();
    const actionsRecord = useActionsRecord();

    const stateApps = useTypedSelector(state => state.appsReducer);
    const stateRecord = useTypedSelector(state => state.recordReducer);
    const {token} = useTypedSelector(state => state.authReducer);

    const findModel = () => {
        const app = stateApps.apps ? stateApps.apps.find(a => a.name === appName) : undefined;
        return app ? app.models.find(model => model.scheme.name === modelName) : undefined
    }

    const [model, modelSet] = useState(findModel())

    useEffect(() => {
        actionsApps.request(token)
        actionsRecord.requestAll(token, appName || "", modelName || "")
    }, [])

    useEffect(() => {
        modelSet(findModel())
    }, [stateApps.apps])

    const onClickRecord = (recordId: string) => {
        navigate(RouteNames.RECORD.replace(':appName', appName || "").replace(':modelName', modelName || "").replace(':recordId', recordId))
    }

    return (
        <RequestWrap isLoading={stateApps.isLoading} error={stateApps.error} object={model}>
            <RequestWrap isLoading={stateApps.isLoading} error={stateApps.error} object={model}>
                { model && stateRecord.allRecords ?
                    <div style={{ height: "75vh" }} className={"overflow-hidden "}>
                        <ModelTable model={model} records={stateRecord.allRecords} onClickRecord={onClickRecord}/>
                    </div>
                    :
                    <div>Error</div>

                }
            </RequestWrap>
        </RequestWrap>
    );
};

export default Model;