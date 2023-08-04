import {Button, Result} from "antd";
import {useNavigate} from "react-router-dom";
import {RouteNames} from "../route.ts";

type ServerErrorProps = {
    title?: string,
    subTitle?: string
}

const ServerError = ({title, subTitle}: ServerErrorProps) => {
    const navigate = useNavigate();

    const onClick = () => {
        navigate(RouteNames.HOME)
    }

    return (
        <Result
            status="500"
            title={title ? title : "500"}
            subTitle={subTitle ? subTitle : "Sorry, something went wrong."}
            extra={<Button type="primary" onClick={onClick}>Back Home</Button>}
        />
    );
};

export default ServerError;