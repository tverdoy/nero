import {Layout} from "antd";
import AppRouter from "./components/AppRouter.tsx";
import {FC, useEffect} from "react";
import {useActions} from "./hooks/useAction.ts";
import IUser from "./models/IUser.ts";

const App: FC = () => {
    const {setUser, setIsAuth} = useActions();

    useEffect(() => {
        if(localStorage.getItem('nero-admin-token')) {
            setUser({ id: localStorage.getItem('nero-admin-id'),username: localStorage.getItem('nero-admin-username')} as IUser)
            setIsAuth(true);
        }
    }, [])

    return (
        <Layout>
            <AppRouter />
        </Layout>
    );
};

export default App;