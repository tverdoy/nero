import {useDispatch} from "react-redux";
import {bindActionCreators} from "redux";
import {AuthActionCreators} from "../store/reducers/auth/action-creators.ts";
import {SettingsActionCreators} from "../store/reducers/settings/action-creators.ts";
import {AppsActionCreators} from "../store/reducers/apps/action-creators.ts";

export const useActionsAuth = () => {
    const dispatch = useDispatch();
    return bindActionCreators(AuthActionCreators, dispatch);
}

export const useActionsSettings = () => {
    const dispatch = useDispatch();
    return bindActionCreators(SettingsActionCreators, dispatch);
}

export const useActionsApps = () => {
    const dispatch = useDispatch();
    return bindActionCreators(AppsActionCreators, dispatch);
}