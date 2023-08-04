import IError from "../../../utils/error.ts";
import IApp from "../../../models/IApp.ts";

export interface AppsState {
    apps?: IApp[],
    isLoading: boolean,
    error?: IError,
}

export enum AppsActionEnum {
    SET_APPS = "SET_APPS",
    SET_IS_LOADING = "SET_IS_LOADING",
    SET_ERROR = "SET_ERROR",
}

export interface SetAppsAction {
    type: AppsActionEnum.SET_APPS;
    payload: IApp[] | undefined
}

export interface SetIsLoadingAction {
    type: AppsActionEnum.SET_IS_LOADING;
    payload: boolean
}

export interface SetErrorAction {
    type: AppsActionEnum.SET_ERROR;
    payload: IError | undefined
}


export type AppsAction =
    SetAppsAction |
    SetErrorAction |
    SetIsLoadingAction