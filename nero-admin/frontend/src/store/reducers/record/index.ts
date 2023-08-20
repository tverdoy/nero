import {RecordAction, RecordActionEnum, RecordState} from "./types.ts";


const initialState: RecordState = {
    record: undefined,
    allRecords: [],
    isLoading: false,
    error: undefined,
}

export default function recordReducer(state = initialState, action: RecordAction): RecordState {
    switch (action.type) {
        case RecordActionEnum.SET_RECORD:
            return {...state, record: action.payload}
        case RecordActionEnum.SET_ALL_RECORD:
            return {...state, allRecords: action.payload}
        case RecordActionEnum.SET_ERROR:
            return {...state, error: action.payload}
        case RecordActionEnum.SET_IS_LOADING:
            return {...state, isLoading: action.payload}
        default:
            return state;
    }
}