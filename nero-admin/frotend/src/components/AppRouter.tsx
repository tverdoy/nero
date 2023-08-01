import React, {FC} from 'react';
import {Navigate, Route, Routes} from "react-router-dom";
import {privateRoutes, publicRoutes, RouteNames} from "../router";
import {Layout} from "antd";
import Sider from "./Sider";
import {useTypedSelector} from "../hooks/useTypedSelector";

const AppRouter: FC = () => {
    const {isAuth} = useTypedSelector(state => state.authReducer);

    return (
        isAuth
            ?
            <Layout hasSider>
                <Sider/>
                <Layout className="Main">
                    <Layout.Content style={{overflow: 'initial'}}>
                        <Routes>
                            {privateRoutes.map(route =>
                                <Route path={route.path} element={<route.element/>} key={route.path}/>
                            )}
                            <Route
                                path="*"
                                element={<Navigate to={RouteNames.HOME} replace/>}
                            />
                        </Routes>
                    </Layout.Content>
                </Layout>
            </Layout>
            :
            <Routes>
                {publicRoutes.map(route =>
                    <Route path={route.path} element={<route.element/>} key={route.path}/>
                )}
                <Route
                    path="*"
                    element={<Navigate to={RouteNames.LOGIN} replace/>}
                />
            </Routes>
    );
};

export default AppRouter;