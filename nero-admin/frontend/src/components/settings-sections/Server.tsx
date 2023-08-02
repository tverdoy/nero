import IServerConf from "../../models/IServerConf.ts";
import {Descriptions, DescriptionsProps} from "antd";

export type ServerProps = {
    server: IServerConf
}

const Server = ({server}: ServerProps) => {
    const items: DescriptionsProps['items'] = [
        {
            key: '1',
            label: 'Address',
            children: server.addr,
        },
        {
            key: '2',
            label: 'Maximum of size HTTP head',
            children: server.max_head_size,
        },
        {
            key: '3',
            label: 'Maximum of size HTTP body',
            children: server.max_body_size,
        },
    ];

    return (
        <Descriptions title="Server" layout="vertical" items={items} />
    );
};

export default Server;