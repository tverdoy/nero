import React from "react";
import Login from "../pages/Login";
import Home from "../pages/Home";
import Settings from "../pages/Settings.tsx";

export interface IRoute {
    path: string,
    element: React.ComponentType
}

export enum RouteNames {
    LOGIN = '/login',
    HOME = '/home',
    SETTINGS = '/settings'
}

export const publicRoutes: IRoute[] = [
    {path: RouteNames.LOGIN, element: Login}
]

export const privateRoutes: IRoute[] = [
    {path: RouteNames.HOME, element: Home},
    {path: RouteNames.SETTINGS, element: Settings}
]