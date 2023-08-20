import IApp from "../models/IApp.ts";
import {Card, Menu, MenuProps} from "antd";
import {useNavigate} from "react-router-dom";
import {RouteNames} from "../route.ts";

type AppCardProps = {
    app: IApp
}

const AppCard = ({app}: AppCardProps) => {
    const navigate = useNavigate()

    const items: MenuProps['items'] = app.models.map(model => {
        return {
            key: `${app.name}/${model.scheme.name}`,
            label: model.scheme.name
        }
    });


    const onClick: MenuProps['onClick'] = (item) => {
        navigate(RouteNames.MODEL.replace(":appName/:modelName", item.key))
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

export default AppCard