import {AuthActionCreators} from "./auth/action-creators";
import {SettingsActionCreators} from "./settings/action-creators.ts";

export const allActionCreators = {
    ...AuthActionCreators,
    ...SettingsActionCreators
}