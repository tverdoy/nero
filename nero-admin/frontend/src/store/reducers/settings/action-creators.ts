import {SetErrorAction, SetIsLoadingAction, SetSettingsAction, SettingsActionEnum} from "./types";
import {AppDispatch} from "../../index";
import ISettings from "../../../models/ISettings.ts";
import axios, {AxiosError} from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../utils/api.ts";
import INeroError from "../../../models/INeroError.ts";
import IError, {ErrorKindEnum} from "../../../utils/error.ts";

export const SettingsActionCreators = {
    setSettings: (settings: ISettings): SetSettingsAction => ({
        type: SettingsActionEnum.SET_SETTINGS,
        payload: settings
    }),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: SettingsActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error?: IError): SetErrorAction => ({type: SettingsActionEnum.SET_ERROR, payload: error}),
    request: (token: string) => async (dispatch: AppDispatch) => {
        dispatch(SettingsActionCreators.setIsLoading(true))

        try {
            const response = await axios.get<ISettings>(BASE_ADDRESS + ApiUrls.SETTINGS, {headers: {"Authorization": `Bearer ${token}`}})
            dispatch(SettingsActionCreators.setSettings(response.data))
        } catch (e) {
            let error = {message: "Failed request settings", kind: ErrorKindEnum.FRONTEND as string};

            if (e instanceof AxiosError) {
                if (e.response) {
                    const neroError: INeroError = e.response.data
                    error = {message: neroError.error, kind: neroError.kind}
                } else {
                    error = {message: e.message, kind: ErrorKindEnum.RESPONSE_EMPTY}
                }
            }

            dispatch(SettingsActionCreators.setError(error))
        }

        dispatch(SettingsActionCreators.setIsLoading(false))
    }
}