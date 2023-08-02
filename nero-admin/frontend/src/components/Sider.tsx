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
        console.log(item.key)
        navigate(item.key)
    }

    return (
        <Layout.Sider
            className={"overflow-auto h-full"}
            width={"20vw"}
        >
            <h2 className={"text-center text-white w-full text-2xl"}>Nero</h2>
            <Menu theme="dark" mode="inline" defaultSelectedKeys={[location.pathname]} items={items} onClick={onClick}/>


        </Layout.Sider>

    );
};

export default Sider;