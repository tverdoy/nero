import {FC} from 'react';
import {Layout, Menu, MenuProps} from "antd";
import {HomeOutlined, SettingOutlined} from "@ant-design/icons";
import {useLocation, useNavigate} from 'react-router-dom';
import {RouteNames} from "../router";


const items: MenuProps['items'] = [
    {
        key: RouteNames.HOME,
        icon: <HomeOutlined/>,
        label: `Home`,
    },
    {
        key: RouteNames.SETTINGS,
        icon: <SettingOutlined/>,
        label: `Settings`,
    }
];

const Sider: FC = () => {
    const navigate = useNavigate();
    let location = useLocation();

    const onClick: MenuProps['onClick'] = (item) => {
        navigate(item.key)
    }

    const selected = () => {
        let path = location.pathname;

        if (items.find((item) => item ? item.key === path : false)) {
            return [location.pathname];
        } else {
            return [RouteNames.HOME]
        }
    }

    return (
        <Layout.Sider
            className={"overflow-auto h-full"}
            width={"20vw"}
        >
            <h2 className={"text-center text-white w-full text-2xl"}>Nero</h2>
            <Menu theme="dark" mode="inline" defaultSelectedKeys={selected()} items={items} onClick={onClick}/>


        </Layout.Sider>

    );
};

export default Sider;