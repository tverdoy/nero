import {SettingsAction, SettingsActionEnum, SettingsState} from "./types.ts";
import ISettings from "../../../models/ISettings.ts";


const initialState: SettingsState = {
    settings: {} as ISettings,
    isLoading: true,
    error: undefined,
}

export default function settingsReducer(state = initialState, action: SettingsAction): SettingsState {
    switch (action.type) {
        case SettingsActionEnum.SET_SETTINGS:
            return {...state, settings: action.payload}
        case SettingsActionEnum.SET_ERROR:
            return {...state, error: action.payload}
        case SettingsActionEnum.SET_IS_LOADING:
            return {...state, isLoading: action.payload}
        default:
            return state;
    }
}