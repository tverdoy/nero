import ISettings from "../../../models/ISettings.ts";
import IError from "../../../utils/error.ts";

export interface SettingsState {
    settings: ISettings,
    isLoading: boolean,
    error?: IError,
}

export enum SettingsActionEnum {
    SET_SETTINGS = "SET_SETTINGS",
    SET_IS_LOADING = "SET_IS_LOADING",
    SET_ERROR = "SET_ERROR",
}

export interface SetSettingsAction {
    type: SettingsActionEnum.SET_SETTINGS;
    payload: ISettings
}

export interface SetIsLoadingAction {
    type: SettingsActionEnum.SET_IS_LOADING;
    payload: boolean
}

export interface SetErrorAction {
    type: SettingsActionEnum.SET_ERROR;
    payload: IError | undefined
}


export type SettingsAction =
    SetSettingsAction |
    SetErrorAction |
    SetIsLoadingAction