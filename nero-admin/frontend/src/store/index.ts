import thunk from 'redux-thunk';
import {applyMiddleware, combineReducers, createStore} from "redux";
import reducers from "./reducers";
import {useDispatch} from "react-redux";

const rootReducer = combineReducers(reducers)

export const store = createStore(rootReducer, applyMiddleware(thunk))

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch;

export const useAppDispatch = () => useDispatch<AppDispatch>()
