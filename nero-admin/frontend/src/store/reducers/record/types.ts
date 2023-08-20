import IError from "../../../utils/error.ts";

export interface RecordState {
    record?: any,
    allRecords: any[]
    isLoading: boolean,
    error?: IError,
}

export enum RecordActionEnum {
    SET_RECORD = "SET_RECORD",
    SET_ALL_RECORD = "SET_ALL_RECORD",
    SET_IS_LOADING = "SET_IS_LOADING",
    SET_ERROR = "SET_ERROR",
}

export interface SetRecordAction {
    type: RecordActionEnum.SET_RECORD;
    payload: any
}

export interface SetAllRecordAction {
    type: RecordActionEnum.SET_ALL_RECORD;
    payload: any[]
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
    SetAllRecordAction |
    SetErrorAction |
    SetIsLoadingAction