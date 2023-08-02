import {Layout} from "antd";
import AppRouter from "./components/AppRouter.tsx";
import {FC} from "react";
import {useActions} from "./hooks/useAction.ts";
import IUser from "./models/IUser.ts";

const App: FC = () => {
    const {setUser, setIsAuth} = useActions();

    if(localStorage.getItem('nero-admin-token')) {
        console.log("fdsf");
        setUser({ id: localStorage.getItem('nero-admin-id'),username: localStorage.getItem('nero-admin-username')} as IUser)
        setIsAuth(true);
    }

    return (
        <Layout>
            <AppRouter />
        </Layout>
    );
};

export default App;