import {AppsActionEnum, SetAppsAction, SetErrorAction, SetIsLoadingAction,} from "./types";
import {AppDispatch} from "../../index";
import axios, {AxiosError} from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../utils/api.ts";
import INeroError from "../../../models/INeroError.ts";
import IError, {ErrorKindEnum} from "../../../utils/error.ts";
import IApp from "../../../models/IApp.ts";

export const AppsActionCreators = {
    setApps: (apps?: IApp[]): SetAppsAction => ({
        type: AppsActionEnum.SET_APPS,
        payload: apps
    }),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: AppsActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error?: IError): SetErrorAction => ({type: AppsActionEnum.SET_ERROR, payload: error}),
    request: (token: string) => async (dispatch: AppDispatch) => {
        dispatch(AppsActionCreators.setIsLoading(true))


        try {
            const response = await axios.get<IApp[]>(BASE_ADDRESS + ApiUrls.APPS, {headers: {"Authorization": `Bearer ${token}`}})
            dispatch(AppsActionCreators.setApps(response.data))
        } catch (e) {
            let error = {message: "Failed request apps", kind: ErrorKindEnum.FRONTEND as string};

            if (e instanceof AxiosError) {
                if (e.response) {
                    const neroError: INeroError = e.response.data
                    error = {message: neroError.error, kind: neroError.kind}
                } else {
                    error = {message: e.message, kind: ErrorKindEnum.RESPONSE_EMPTY}
                }
            }

            dispatch(AppsActionCreators.setError(error))
        }

        dispatch(AppsActionCreators.setIsLoading(false))
    }
}