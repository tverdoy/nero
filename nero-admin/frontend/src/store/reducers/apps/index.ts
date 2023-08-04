import {AppsAction, AppsActionEnum, AppsState} from "./types.ts";


const initialState: AppsState = {
    apps: undefined,
    isLoading: false,
    error: undefined,
}

export default function appsReducer(state = initialState, action: AppsAction): AppsState {
    switch (action.type) {
        case AppsActionEnum.SET_APPS:
            return {...state, apps: action.payload}
        case AppsActionEnum.SET_ERROR:
            return {...state, error: action.payload}
        case AppsActionEnum.SET_IS_LOADING:
            return {...state, isLoading: action.payload}
        default:
            return state;
    }
}