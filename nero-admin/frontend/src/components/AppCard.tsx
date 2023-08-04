import IApp from "../models/IApp.ts";
import {Card, Menu, MenuProps} from "antd";

type AppCardProps = {
    app: IApp
}

const AppCard = ({app}: AppCardProps) => {
    const items: MenuProps['items'] = app.schemes.map(scheme => {
        return {
            key: scheme.name,
            label: scheme.name
        }
    });


    const onClick: MenuProps['onClick'] = (item) => {

    }

    return (
        <Card title={app.name} className={"shadow-xl"}>
            <Menu
                onClick={onClick}
                mode="inline"
                items={items}
                className={"w-full"}
                style={{borderInlineEnd: 0}}
            />
        </Card>
    );
};

export default AppCard;