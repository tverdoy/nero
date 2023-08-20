import {RecordActionEnum, SetRecordAction, SetErrorAction, SetIsLoadingAction,} from "./types";
import {AppDispatch} from "../../index";
import axios, {AxiosError} from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../utils/api.ts";
import INeroError from "../../../models/INeroError.ts";
import IError, {ErrorKindEnum} from "../../../utils/error.ts";
import IApp from "../../../models/IApp.ts";

export const RecordActionCreators = {
    setRecord: (record?: any): SetRecordAction => ({
        type: RecordActionEnum.SET_RECORD,
        payload: record
    }),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: RecordActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error?: IError): SetErrorAction => ({type: RecordActionEnum.SET_ERROR, payload: error}),
    requestOne: (token: string, appName: string, modelName: string, id: string | number) => async (dispatch: AppDispatch) => {
        dispatch(RecordActionCreators.setIsLoading(true))

        try {
            const response = await axios.get<IApp[]>(BASE_ADDRESS + ApiUrls.RECORD, {params: { id: id, app: appName, model: modelName}, headers: {"Authorization": `Bearer ${token}`}})
            dispatch(RecordActionCreators.setRecord(response.data))
        } catch (e) {
            let error = {message: "Failed request record", kind: ErrorKindEnum.FRONTEND as string};

            if (e instanceof AxiosError) {
                if (e.response) {
                    const neroError: INeroError = e.response.data
                    error = {message: neroError.error, kind: neroError.kind}
                } else {
                    error = {message: e.message, kind: ErrorKindEnum.RESPONSE_EMPTY}
                }
            }

            dispatch(RecordActionCreators.setError(error))
        }

        dispatch(RecordActionCreators.setIsLoading(false))
    }
}