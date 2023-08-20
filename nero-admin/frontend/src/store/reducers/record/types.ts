import IError from "../../../utils/error.ts";
import IApp from "../../../models/IApp.ts";

export interface RecordState {
    record?: any,
    isLoading: boolean,
    error?: IError,
}

export enum RecordActionEnum {
    SET_RECORD = "SET_RECORD",
    SET_IS_LOADING = "SET_IS_LOADING",
    SET_ERROR = "SET_ERROR",
}

export interface SetRecordAction {
    type: RecordActionEnum.SET_RECORD;
    payload: any
}

export interface SetIsLoadingAction {
    type: RecordActionEnum.SET_IS_LOADING;
    payload: boolean
}

export interface SetErrorAction {
    type: RecordActionEnum.SET_ERROR;
    payload: IError | undefined
}


export type RecordAction =
    SetRecordAction |
    SetErrorAction |
    SetIsLoadingAction