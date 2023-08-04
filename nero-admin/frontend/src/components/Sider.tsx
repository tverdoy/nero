import {FC, useEffect, useState} from 'react';
import {Layout, Menu, MenuProps} from "antd";
import {DeploymentUnitOutlined, HomeOutlined, SettingOutlined} from "@ant-design/icons";
import {useLocation, useNavigate} from 'react-router-dom';
import {RouteNames} from "../route.ts";


const items: MenuProps['items'] = [
    {
        key: RouteNames.HOME,
        icon: <HomeOutlined/>,
        label: `Home`,
    },
    {
        key: RouteNames.APPS,
        icon: <DeploymentUnitOutlined/>,
        label: `Apps`,
    },
    {
        key: RouteNames.SETTINGS,
        icon: <SettingOutlined/>,
        label: `Settings`,
    },
];

const Sider: FC = () => {
    const navigate = useNavigate();
    let location = useLocation();
    const [currentItem, setCurrentItem] = useState([RouteNames.HOME as string]);

    useEffect(() => {
        setCurrentItem([location.pathname])
    }, [location])

    const onClick: MenuProps['onClick'] = (item) => {
        setCurrentItem([location.pathname])
        navigate(item.key)
    }


    return (
        <Layout.Sider
            className={"overflow-auto h-full"}
            width={"20vw"}
        >
            <h2 className={"text-center text-white w-full text-2xl"}>Nero</h2>
            <Menu theme="dark" mode="inline" selectedKeys={currentItem} items={items} onClick={onClick}/>


        </Layout.Sider>

    );
};

export default Sider;