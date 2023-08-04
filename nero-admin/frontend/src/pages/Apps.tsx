import {FC, useEffect, useRef} from 'react';
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import {useActionsApps, useActionsAuth, useActionsSettings} from "../hooks/useAction.ts";
import {Col, message, Row} from "antd";
import {ErrorKindEnum} from "../utils/error.ts";
import AppCard from "../components/AppCard.tsx";

const Apps: FC = () => {
    const {apps, isLoading, error} = useTypedSelector(state => state.appsReducer)
    const {token} = useTypedSelector(state => state.authReducer)
    const {request} = useActionsApps()
    const {logout} = useActionsAuth()
    const [messageApi, contextHolder] = message.useMessage();

    const shouldLog = useRef(true)

    const validApps = apps.filter(app => app.schemes.length );

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

    return (
        <div>
            {contextHolder}
            { isLoading
                ?
                <div>LOADDING</div>
                :
                <Row gutter={[16, 16]}>
                    {validApps.map(app =>
                        {
                            let span = 8;
                            if (validApps.length < 3) {
                                span = validApps.length % 2 ? 24 : 12

                            }
                            return <Col span={span}><AppCard app={app}/></Col>
                        }
                    )}
                </Row>
            }
        </div>
    );
};

export default Apps;