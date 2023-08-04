import {FC, useEffect, useState} from 'react';
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import {useActionsApps} from "../hooks/useAction.ts";
import {Col, Row} from "antd";
import AppCard from "../components/AppCard.tsx";
import RequestWrap from "../components/RequestWrap.tsx";

const Apps: FC = () => {
    const {apps, isLoading, error} = useTypedSelector(state => state.appsReducer)
    const {token} = useTypedSelector(state => state.authReducer)
    const {request} = useActionsApps()

    const findValidApps = () => apps ? apps.filter(app => app.schemes.length) : undefined

    const [validApps, validAppsSet] = useState(findValidApps());


    useEffect(() => {
        request(token)
    }, [])

    useEffect(() => {
        validAppsSet(findValidApps())
    }, [apps])


    return (
        <RequestWrap isLoading={isLoading} error={error} object={apps}>
            <Row className={"fade-in"} gutter={[16, 16]}>
                {validApps && validApps.map(app => {
                        let span = 8;
                        if (validApps.length < 3) {
                            span = validApps.length % 2 ? 24 : 12

                        }
                        return <Col key={app.name} span={span}><AppCard app={app}/></Col>
                    }
                )}
            </Row>
        </RequestWrap>
    );
};

export default Apps;