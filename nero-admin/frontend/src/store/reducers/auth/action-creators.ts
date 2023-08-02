import {AuthActionEnum, SetAuthAction, SetErrorAction, SetIsLoadingAction, SetUserAction} from "./types";
import IUser from "../../../models/IUser";
import {AppDispatch} from "../../index";
import axios from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../api";

export const AuthActionCreators = {
    setUser: (user: IUser): SetUserAction => ({type: AuthActionEnum.SET_USER, payload: user}),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: AuthActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error: string): SetErrorAction => ({type: AuthActionEnum.SET_ERROR, payload: error}),
    setIsAuth: (isAuth: boolean): SetAuthAction => ({type: AuthActionEnum.SET_AUTH, payload: isAuth}),
    login: (username: string, password: string) => async (dispatch: AppDispatch) => {
        try {
            dispatch(AuthActionCreators.setIsLoading(true));

            await axios.post(BASE_ADDRESS + ApiUrls.LOGIN, {username: username, password: password});
        } catch (e) {
            dispatch(AuthActionCreators.setError('Error login'))
        }
    },
    logout: () => async (dispatch: AppDispatch) => {
        try {

        } catch (e) {

        }
    }
}