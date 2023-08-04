import {Descriptions, DescriptionsProps, Spin} from "antd";
import IDataBaseConf from "../../models/IDataBaseConf.ts";
import {CheckCircleOutlined, StopOutlined} from "@ant-design/icons";

export type DataBaseProps = {
    database: IDataBaseConf,
    isLoading: boolean
}

const DataBase = ({database, isLoading}: DataBaseProps) => {
    if (database == undefined || isLoading) {
        return <Spin/>
    } else {
        const items: DescriptionsProps['items'] = [
            {
                key: '1',
                label: 'Connect',
                children: database.connect ? <CheckCircleOutlined className={"text-green-500 text-xl"}/> :
                    <StopOutlined className={"text-red-500 text-xl"}/>,
            },
            {
                key: '2',
                label: 'Address',
                children: database.db_addr,
            },
            {
                key: '3',
                label: 'User',
                children: database.db_user,
            },
            {
                key: '4',
                label: 'Password',
                children: database.db_password,
            },
            {
                key: '5',
                label: 'Database',
                children: database.db_db,
            },
            {
                key: '6',
                label: 'Namespace',
                children: database.db_ns,
            },
        ];

        return <Descriptions title="DataBase" layout="vertical" items={items}/>
    }
};

export default DataBase;