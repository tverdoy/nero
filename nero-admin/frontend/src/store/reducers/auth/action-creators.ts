import {
    AuthActionEnum,
    SetAuthAction,
    SetErrorAction,
    SetIsLoadingAction,
    SetTokenAction,
    SetUserAction
} from "./types";
import IUser from "../../../models/IUser";
import {AppDispatch} from "../../index";
import axios, {AxiosError} from "axios";
import {ApiUrls, BASE_ADDRESS} from "../../../api";
import {IAuth} from "../../../api/IAuthApi.ts";
import {RouteNames} from "../../../router";

export const AuthActionCreators = {
    setUser: (user: IUser): SetUserAction => ({type: AuthActionEnum.SET_USER, payload: user}),
    setToken: (token: string): SetTokenAction => ({type: AuthActionEnum.SET_TOKEN, payload: token}),
    setIsLoading: (isLoading: boolean): SetIsLoadingAction => ({
        type: AuthActionEnum.SET_IS_LOADING,
        payload: isLoading
    }),
    setError: (error: string): SetErrorAction => ({type: AuthActionEnum.SET_ERROR, payload: error}),
    setIsAuth: (isAuth: boolean): SetAuthAction => ({type: AuthActionEnum.SET_AUTH, payload: isAuth}),
    login: (username: string, password: string) => async (dispatch: AppDispatch) => {
        dispatch(AuthActionCreators.setIsLoading(true));

        try {
            const response = await axios.post<IAuth>(BASE_ADDRESS + ApiUrls.LOGIN, {username: username, password: password});
            const auth = response.data;
            if (auth.token) {
                localStorage.setItem("nero-admin-token", auth.token)
                localStorage.setItem("nero-admin-id", auth.user.id)
                localStorage.setItem("nero-admin-username", auth.user.username)
                localStorage.setItem('nero-admin-last-username', auth.user.username)

                dispatch(AuthActionCreators.setIsAuth(true))
                dispatch(AuthActionCreators.setToken(auth.token))
                dispatch(AuthActionCreators.setUser(auth.user))
            } else {
                dispatch(AuthActionCreators.setError('Invalid username or password'))
            }
        } catch (e) {
            let errorIsSet = false;
            if (e instanceof AxiosError) {
                if (e.request.status == 401) {
                    dispatch(AuthActionCreators.setError('Invalid username or password'))
                    errorIsSet = true
                }
            }

            if (!errorIsSet) {
                dispatch(AuthActionCreators.setError('Server return error'))
            }
        }

        dispatch(AuthActionCreators.setIsLoading(false));
    },
    logout: () => async (_: AppDispatch) => {
        localStorage.removeItem('nero-admin-token')
        localStorage.removeItem('nero-admin-id')
        localStorage.removeItem('nero-admin-username')

        window.location.pathname = RouteNames.LOGIN
    }
}