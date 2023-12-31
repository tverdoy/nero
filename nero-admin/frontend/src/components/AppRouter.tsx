import {FC} from 'react';
import {Navigate, Route, Routes} from "react-router-dom";
import {App, Layout} from "antd";
import Sider from "./Sider";
import {useTypedSelector} from "../hooks/useTypedSelector";
import Header from "./Header.tsx";
import {Content} from "antd/es/layout/layout";
import {privateRoutes, publicRoutes, RouteNames} from "../route.ts";

const AppRouter: FC = () => {
    const {isAuth} = useTypedSelector(state => state.authReducer);

    if (isAuth) {
        return (
            <App message={{maxCount: 1}}>
                <Layout hasSider className={"h-screen"}>
                    <Sider/>
                    <Layout style={{width: "80vw"}} className={"p-3"}>
                        <Header/>
                        <Content className={"overflow-auto pt-14 px-6"}>
                            <Routes>
                                {privateRoutes.map(route =>
                                    <Route path={route.path} element={<route.element/>} key={route.path}/>
                                )}

                                <Route
                                    path="*"
                                    element={<Navigate to={RouteNames.HOME} replace/>}
                                />
                            </Routes>
                        </Content>
                    </Layout>
                </Layout>
            </App>
        );
    } else {
        return (
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
    }
};

export default AppRouter;