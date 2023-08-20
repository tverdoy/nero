import React from "react";
import Login from "./pages/Login.tsx";
import Home from "./pages/Home.tsx";
import Settings from "./pages/Settings.tsx";
import Apps from "./pages/Apps.tsx";
import Model from "./pages/Model.tsx";
import Record from "./pages/Record.tsx";

export interface IRoute {
    path: string,
    element: React.ComponentType
}

export enum RouteNames {
    LOGIN = '/login',
    HOME = '/home',
    SETTINGS = '/settings',
    APPS = '/apps',
    MODEL = '/model/:appName/:modelName',
    RECORD = '/model/:appName/:modelName/:recordId'
}

export const publicRoutes: IRoute[] = [
    {path: RouteNames.LOGIN, element: Login}
]

export const privateRoutes: IRoute[] = [
    {path: RouteNames.HOME, element: Home},
    {path: RouteNames.SETTINGS, element: Settings},
    {path: RouteNames.APPS, element: Apps},
    {path: RouteNames.MODEL, element: Model},
    {path: RouteNames.RECORD, element: Record}
]