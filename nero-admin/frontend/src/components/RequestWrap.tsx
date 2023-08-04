import {JSX} from 'react';
import IError, {ErrorKindEnum} from "../utils/error.ts";
import {Result, Spin} from "antd";
import ServerError from "./ServerError.tsx";
import {useActionsAuth} from "../hooks/useAction.ts";

type RequestWrapProps = {
    isLoading: boolean,
    error?: IError,
    object: any
    children: JSX.Element
}

const RequestWrap = ({isLoading, error, object, children}: RequestWrapProps) => {
    const {logout} = useActionsAuth()

    console.log(isLoading)
    if (isLoading) {
        return <div className={"grid place-items-center h-full"}>
            <Spin/>
        </div>
    } else if (error) {
        if (error.kind == ErrorKindEnum.AUTH) {
            logout()
        }

        return (
            <ServerError title={error.kind} subTitle={error.message}/>

        );
    } else if (object == undefined) {
        return <Result
            status="404"
            title="404"
            subTitle="Sorry, the page you visited does not exist."
        />
    } else {
        return (
            <>{children}</>
        );
    }
};

export default RequestWrap;