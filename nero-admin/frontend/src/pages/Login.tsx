import {FC} from 'react';
import {Button, Checkbox, Form, Layout, Input} from "antd";

const Login: FC = () => {
    return (
        <Layout className={"grid min-h-full h-screen place-items-center px-6 py-24 sm:py-32 lg:px-8"}>
            <div className={"px-11 py-24 bg-white shadow-2xl rounded-lg w-auto sm:w-4/5 md:w-96"}>
                <Form
                    name="basic"
                    initialValues={{ remember: true }}
                    autoComplete="off"
                    layout={"vertical"}
                >
                    <Form.Item
                        label="Username"
                        name="username"
                        rules={[{ required: true, message: 'Please input your username!' }]}
                        className={"mb-10"}
                    >
                        <Input />
                    </Form.Item>

                    <Form.Item
                        label="Password"
                        name="password"
                        rules={[{ required: true, message: 'Please input your password!' }]}
                        className={"mb-6"}
                    >
                        <Input.Password />
                    </Form.Item>

                    <Form.Item
                        name="remember"
                        valuePropName="checked"
                    >
                        <Checkbox>Remember me</Checkbox>
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" className={"bg-blue-600 w-full"}>
                            Submit
                        </Button>
                    </Form.Item>
                </Form>
            </div>
        </Layout>
    );
};

export default Login;