import {Layout} from "antd";
import AppRouter from "./components/AppRouter.tsx";
import {FC} from "react";
import {useActionsAuth} from "./hooks/useAction.ts";
import IUser from "./models/IUser.ts";

const App: FC = () => {
    const {setUser, setIsAuth, setToken} = useActionsAuth();

    if(localStorage.getItem('nero-admin-token')) {
        setUser({ id: localStorage.getItem('nero-admin-id'), username: localStorage.getItem('nero-admin-username')} as IUser)
        setIsAuth(true);
        setToken(localStorage.getItem('nero-admin-token') || '')
    }

    return (
        <Layout>
            <AppRouter />
        </Layout>
    );
};

export default App;