import {FC} from 'react';
import {Layout, Menu, MenuProps} from "antd";
import {HomeOutlined, UserOutlined} from "@ant-design/icons";

const items: MenuProps['items'] = [
    {
        key: "home",
        icon: <HomeOutlined/>,
        label: `Home`,
    },
    {
        key: "users",
        icon: <UserOutlined/>,
        label: `Users`,
    }
];

const Sider: FC = () => {
    return (
        <Layout.Sider
            className={"overflow-auto h-full"}
            width={"20vw"}
        >
            <h2 className={"text-center text-white w-full text-2xl"}>Nero</h2>
            <Menu theme="dark" mode="inline" defaultSelectedKeys={['home']} items={items}/>


        </Layout.Sider>

    );
};

export default Sider;