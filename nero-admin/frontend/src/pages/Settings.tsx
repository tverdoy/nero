import {FC, useEffect, useRef, useState} from 'react';
import {Col, Layout, Menu, MenuProps, message, Row} from "antd";
import {DatabaseOutlined, SafetyCertificateOutlined, SettingOutlined} from "@ant-design/icons";
import DataBase from "../components/settings-sections/DataBase.tsx";
import Cors from "../components/settings-sections/Cors.tsx";
import Server from "../components/settings-sections/Server.tsx";
import {useActionsAuth, useActionsSettings} from "../hooks/useAction.ts";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import ServerError from "../components/ServerError.tsx";
import {ErrorKindEnum} from "../utils/error.ts";


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


const Settings: FC = () => {
    const [section, setSection] = useState(SettingsSection.SERVER);
    const {settings, isLoading, error} = useTypedSelector(state => state.settingsReducer)
    const {token} = useTypedSelector(state => state.authReducer)
    const {request} = useActionsSettings()
    const {logout} = useActionsAuth()
    const [messageApi, contextHolder] = message.useMessage();

    const shouldLog = useRef(true)

    useEffect(() => {
        request(token)
    }, [])

    useEffect(() => {
        if (error) {
            if (error.kind == ErrorKindEnum.AUTH) {
                logout()
            }

            if (shouldLog) {
                shouldLog.current = false

                // noinspection JSIgnoredPromiseFromCall
                messageApi.open({
                    type: 'error',
                    content: error.message,
                });
            }
        }
    }, [error])


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
                return <Server isLoading={isLoading} server={settings.server}/>
            case SettingsSection.DATABASE:
                return <DataBase isLoading={isLoading} database={settings.db}/>
            case SettingsSection.CORS:
                return <Cors isLoading={isLoading} cors={settings.cors}/>
        }
    }

    if (!error) {
        return (
            <div className={"fade-in bg-white shadow-2xl rounded-lg p-6 h-4/6 pt-12"}>
                {contextHolder}
                <Row>
                    <Col span={16}>
                        {sectionComponent()}
                    </Col>
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
        )
    } else {
        return <div>
            {contextHolder}
            <ServerError/>
        </div>
    }
};
export default Settings;