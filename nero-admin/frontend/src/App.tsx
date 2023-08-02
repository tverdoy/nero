import {Layout} from "antd";
import AppRouter from "./components/AppRouter.tsx";
import {FC} from "react";

const App: FC = () => {
    return (
        <Layout>
            <AppRouter />
        </Layout>
    );
};

export default App;