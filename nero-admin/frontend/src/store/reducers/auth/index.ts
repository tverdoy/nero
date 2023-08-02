import {AuthAction, AuthActionEnum, AuthState} from "./types";
import IUser from "../../../models/IUser";

const initialState: AuthState = {
    isAuth: false,
    user: {} as IUser,
    token: '',
    isLoading: false,
    error: "",
}

export default function authReducer(state = initialState, action: AuthAction): AuthState {
    switch (action.type) {
        case AuthActionEnum.SET_AUTH:
            return {...state, isAuth: action.payload}
        case AuthActionEnum.SET_USER:
            return {...state, user: action.payload}
        case AuthActionEnum.SET_TOKEN:
            return {...state, token: action.payload}
        case AuthActionEnum.SET_ERROR:
            return {...state, error: action.payload}
        case AuthActionEnum.SET_IS_LOADING:
            return {...state, isLoading: action.payload}

        default:
            return state;
    }
}