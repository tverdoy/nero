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
import IError from "../../../utils/Error.ts";

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
            dispatch(SettingsActionCreators.setIsUnAuth(false))
        } catch (e) {
            if (e instanceof AxiosError) {
                if (e.request.status == 401) {
                    // dispatch(SettingsActionCreators.setIsUnAuth(true))
                }

                if (e.response) {
                    const neroError: INeroError = e.response.data
                    dispatch(SettingsActionCreators.setError({neroError: neroError, code: e.response.status}))
                } else {
                    dispatch(SettingsActionCreators.setError({code: 500}))
                }
            } else {
                dispatch(SettingsActionCreators.setError({code: 0}))
            }
        }

        dispatch(SettingsActionCreators.setIsLoading(false))
    }
}