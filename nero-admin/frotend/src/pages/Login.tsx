import React, {FC} from 'react';
import {Button, Card, Checkbox, Form, Input, Layout, Row} from "antd";
import {useActions} from "../hooks/useAction";

const Login: FC = () => {
    const {login} = useActions()

    const onFinish = (values: any) => {
        console.log(values);
        login(values.username, values.password)
    };

    const onFinishFailed = (errorInfo: any) => {
        console.log('Failed:', errorInfo);
    };

    return (
        <Layout>
            <Row justify={"center"} align={"middle"}
                 style={{width: "100vw", height: "100vh", minHeight: "300px", minWidth: "400px"}}>
                <Card bodyStyle={{height: "100%", width: "100%"}} style={{
                    width: "32vw",
                    height: "70vh",
                    minHeight: "300px",
                    minWidth: "400px",
                    margin: "calc(var(--index) * 3)"
                }}>
                    <Row justify={"center"} align={"middle"} style={{height: "100%", width: "100%"}}>
                        <Form
                            name="basic"
                            wrapperCol={{span: 20}}
                            style={{maxWidth: 600, width: "100%"}}
                            initialValues={{remember: true}}
                            onFinish={onFinish}
                            onFinishFailed={onFinishFailed}
                            autoComplete="off"
                        >
                            <Form.Item
                                label="Username"
                                name="username"
                                rules={[{required: true, message: 'Please input your username!'}]}
                            >
                                <Input/>
                            </Form.Item>

                            <Form.Item
                                label="Password"
                                name="password"
                                rules={[{required: true, message: 'Please input your password!'}]}
                            >
                                <Input.Password/>
                            </Form.Item>

                            <Form.Item name="remember" valuePropName="checked" wrapperCol={{offset: 0, span: 24}}>
                                <Checkbox>Remember me</Checkbox>
                            </Form.Item>

                            <Form.Item wrapperCol={{offset: 0, span: 24}}>
                                <Button type="primary" htmlType="submit" size={"large"} style={{width: "100%"}}>
                                    Login
                                </Button>
                            </Form.Item>
                        </Form>
                    </Row>
                </Card>
            </Row>
        </Layout>
    );
};

export default Login;