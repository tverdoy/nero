import {FC} from 'react';
import {Button, Layout} from "antd";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import {useActions} from "../hooks/useAction.ts";

const Header: FC = () => {
    const {user} = useTypedSelector(state => state.authReducer);
    const {logout} = useActions()

    return (
        <Layout.Header className={"rounded-2xl"}>
            <div className={"flex justify-between items-center"}>
                <p className={"text-center text-white text-base"}>
                    {user.username}
                </p>
                <Button type="primary" danger size={"middle"} onClick={logout}>Logout</Button>
            </div>
        </Layout.Header>
    );
};

export default Header;