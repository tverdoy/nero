import {
    SetErrorAction,
    SetIsLoadingAction, SetIsUnAuthAction,
    SetSettingsAction,
    SettingsActionEnum
} from "./types";
import {AppDispatch} from "../../index";
import ISettings from "../../../models/ISettings.ts";
import axios, {AxiosError} from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../api";
import INeroError from "../../../models/INeroError.ts";
import IError, {ErrorKindEnum} from "../../../utils/Error.ts";

export const SettingsActionCreators = {
    setSettings: (settings: ISettings): SetSettingsAction => ({type: SettingsActionEnum.SET_SETTINGS, payload: settings}),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: SettingsActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error?: IError): SetErrorAction => ({type: SettingsActionEnum.SET_ERROR, payload: error}),
    setIsUnAuth: (isUnAuth: boolean): SetIsUnAuthAction => ({type: SettingsActionEnum.SET_IS_UN_AUTH, payload: isUnAuth}),
    request: (token: string) => async (dispatch: AppDispatch) => {
        dispatch(SettingsActionCreators.setIsLoading(true))

        try {
            const response = await axios.get<ISettings>(BASE_ADDRESS + ApiUrls.SETTINGS, { headers: {"Authorization" : `Bearer ${token}`} })
            dispatch(SettingsActionCreators.setSettings(response.data))
        } catch (e) {
            if (e instanceof AxiosError) {
                if (e.response) {
                    const neroError: INeroError = e.response.data
                    dispatch(SettingsActionCreators.setError({message: neroError.error, kind: neroError.kind, code: e.response.status}))
                } else {
                    dispatch(SettingsActionCreators.setError({message: e.message, kind: ErrorKindEnum.RESPONSE_EMPTY, code: 500}))
                }
            } else {
                dispatch(SettingsActionCreators.setError({message: "Failed request settings", kind: ErrorKindEnum.FRONTEND}))
            }
        }

        dispatch(SettingsActionCreators.setIsLoading(false))
    }
}