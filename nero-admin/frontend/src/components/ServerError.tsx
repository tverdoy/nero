import {Button, Result} from "antd";
import {useNavigate} from "react-router-dom";
import {RouteNames} from "../router";

const ServerError = () => {
    const navigate = useNavigate();

    const onClick = () => {
        navigate(RouteNames.HOME)
    }

    return (
        <Result
            status="500"
            title="500"
            subTitle="Sorry, something went wrong."
            extra={<Button type="primary" onClick={onClick}>Back Home</Button>}
        />
    );
};

export default ServerError;