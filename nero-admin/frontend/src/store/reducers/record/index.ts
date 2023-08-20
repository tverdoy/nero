import {RecordAction, RecordActionEnum, RecordState} from "./types.ts";


const initialState: RecordState = {
    record: undefined,
    isLoading: false,
    error: undefined,
}

export default function recordReducer(state = initialState, action: RecordAction): RecordState {
    switch (action.type) {
        case RecordActionEnum.SET_RECORD:
            return {...state, apps: action.payload}
        case RecordActionEnum.SET_ERROR:
            return {...state, error: action.payload}
        case RecordActionEnum.SET_IS_LOADING:
            return {...state, isLoading: action.payload}
        default:
            return state;
    }
}