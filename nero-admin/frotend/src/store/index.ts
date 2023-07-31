import thunk from 'redux-thunk';
import {applyMiddleware, combineReducers, createStore} from "redux";

const rootReducer = combineReducers({

})

export const store = createStore(rootReducer, applyMiddleware(thunk))

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch;