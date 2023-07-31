import React, {FC} from 'react';
import {Layout, Menu, MenuProps, Space} from "antd";
import {HomeOutlined, UserOutlined} from "@ant-design/icons";
import Title from "antd/es/typography/Title";

const items: MenuProps['items'] = [
    {
        key: "home",
        icon: <HomeOutlined/>,
        label: `Home`,
    },
    {
        key: "users",
        icon: <UserOutlined />,
        label: `Users`,
    }
];

const Sider: FC = () => {
    return (
        <Layout.Sider className="Sider" style={{
            overflow: 'auto',
            height: '100vh',
            position: 'fixed',
            left: 0,
            top: 0,
            bottom: 0,
            width: '100px'
        }}>
           <Space align={"center"} style={{ width: "100%", justifyContent: "center" }}>
               <Title className={"sider--title"} level={2}>Nero</Title>
           </Space>
            <Menu theme="dark" mode="inline" defaultSelectedKeys={['home']} items={items}/>
        </Layout.Sider>

    );
};

export default Sider;