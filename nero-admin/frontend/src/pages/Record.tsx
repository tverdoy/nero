import {useEffect, useState} from 'react';
import {useParams} from "react-router-dom";
import {useActionsApps, useActionsRecord} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import RequestWrap from "../components/RequestWrap.tsx";
import ModelTable from "../components/ModelTable.tsx";

const Record = () => {
    const {appName, modelName, recordId} = useParams();
    const {requestOne} = useActionsRecord();
    const {record, isLoading, error} = useTypedSelector(state => state.recordReducer);
    const {token} = useTypedSelector(state => state.authReducer);

    useEffect(() => {
        requestOne(token, recordId)
    }, [])


    return (
        <RequestWrap isLoading={isLoading} error={error} object={record}>
            { record ?
                <h1>Ok</h1>
                :
                <div>Error</div>

            }
        </RequestWrap>
    );
};

export default Record;