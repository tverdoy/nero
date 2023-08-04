import React from "react";
import Login from "./pages/Login.tsx";
import Home from "./pages/Home.tsx";
import Settings from "./pages/Settings.tsx";
import Apps from "./pages/Apps.tsx";

export interface IRoute {
    path: string,
    element: React.ComponentType
}

export enum RouteNames {
    LOGIN = '/login',
    HOME = '/home',
    SETTINGS = '/settings',
    APPS = '/apps'
}

export const publicRoutes: IRoute[] = [
    {path: RouteNames.LOGIN, element: Login}
]

export const privateRoutes: IRoute[] = [
    {path: RouteNames.HOME, element: Home},
    {path: RouteNames.SETTINGS, element: Settings},
    {path: RouteNames.APPS, element: Apps}
]