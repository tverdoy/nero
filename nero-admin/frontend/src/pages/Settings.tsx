import {FC, useState} from 'react';
import {Col, Layout, Menu, MenuProps, Row} from "antd";
import {DatabaseOutlined, SafetyCertificateOutlined, SettingOutlined} from "@ant-design/icons";
import IDataBaseConf from "../models/IDataBaseConf.ts";
import DataBase from "../components/settings-sections/DataBase.tsx";
import ICorsConf from "../models/ICorsConf.ts";
import Cors from "../components/settings-sections/Cors.tsx";
import Server from "../components/settings-sections/Server.tsx";

enum SettingsSection {
    SERVER = "SERVER",
    DATABASE = "DATABASE",
    CORS = "CORS"
}

const items: MenuProps['items'] = [
    {
        key: SettingsSection.SERVER,
        icon: <SettingOutlined/>,
        label: `Server`,
    },
    {
        key: SettingsSection.DATABASE,
        icon: <DatabaseOutlined/>,
        label: `Database`,
    },
    {
        key: SettingsSection.CORS,
        icon: <SafetyCertificateOutlined/>,
        label: `Cors`,
    }
];


const testServer = {addr: "127.0.0.1:8080", max_body_size: 10903, max_head_size: 4096};
const testDataBase: IDataBaseConf = {
    connect: false,
    db_addr: "127.0.0.1:8000",
    db_user: "root",
    db_password: "root",
    db_db: "nero",
    db_ns: "nero"
}

const testCors: ICorsConf = {
    is_allow_cors: true,
    allow_origin: "127.0.0.1:3000",
    allow_headers: ["Content-Length"],
    allow_methods: ["GET", "POST"]
}

const Settings: FC = () => {
    const [section, setSection] = useState(SettingsSection.SERVER);

    const onClick: MenuProps['onClick'] = (item) => {
        if (item.key === "SERVER") {
            setSection(SettingsSection.SERVER)
        } else if (item.key === "DATABASE") {
            setSection(SettingsSection.DATABASE)
        } else {
            setSection(SettingsSection.CORS)
        }
    }

    const sectionComponent = () => {
        switch (section) {
            case SettingsSection.SERVER:
                return <Server server={testServer}/>
            case SettingsSection.DATABASE:
                return <DataBase database={testDataBase}/>
            case SettingsSection.CORS:
                return <Cors cors={testCors}/>
        }
    }

    return (
        <div className={"fade-in bg-white shadow-2xl rounded-lg p-6"}>
            <Row>
                <Col span={16}>{sectionComponent()}</Col>
                <Col span={8} className={"p-3"} style={{borderInlineStart: "1px solid rgba(5, 5, 5, 0.06)"}}>
                    <Layout>
                        <Menu
                            onClick={onClick}
                            defaultSelectedKeys={[SettingsSection.SERVER]}
                            mode="inline"
                            items={items}
                            className={"w-full"}
                            style={{borderInlineEnd: 0}}
                        />
                    </Layout>
                </Col>
            </Row>
        </div>
    );
};
export default Settings;