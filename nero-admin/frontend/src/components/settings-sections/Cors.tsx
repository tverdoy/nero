import {Descriptions, DescriptionsProps, Spin} from "antd";
import {CheckCircleOutlined, StopOutlined} from "@ant-design/icons";
import ICorsConf from "../../models/ICorsConf.ts";

export type CorsProps = {
    cors: ICorsConf,
    isLoading: boolean
}

const Cors = ({cors, isLoading}: CorsProps) => {
    if (cors == undefined || isLoading) {
        return <Spin/>
    } else {
        const items: DescriptionsProps['items'] = [
            {
                key: '1',
                label: 'Allow CORS',
                children: cors.is_allow_cors ? <CheckCircleOutlined className={"text-green-500 text-xl"}/> :
                    <StopOutlined className={"text-red-500 text-xl"}/>,
            },
            {
                key: '2',
                label: 'Allow origin',
                children: cors.allow_origin === "*" ? "Any" : cors.allow_origin,
            },
            {
                key: '3',
                label: 'Allow headers',
                children: cors.allow_headers.join(", "),
            },
            {
                key: '4',
                label: 'Allow methods',
                children: cors.allow_methods.join(", "),
            },
        ];

        return <Descriptions title="CORS" layout="vertical" items={items}/>
    }
};

export default Cors;