import {useEffect} from 'react';
import {useParams} from "react-router-dom";
import {useActionsRecord} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import RequestWrap from "../components/RequestWrap.tsx";

const Record = () => {
    const {appName, modelName, recordId} = useParams();
    const {requestOne} = useActionsRecord();
    const {record, isLoading, error} = useTypedSelector(state => state.recordReducer);
    const {token} = useTypedSelector(state => state.authReducer);

    useEffect(() => {
        requestOne(token, appName || "", modelName || "", recordId || "")
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