import {FC, useState} from 'react';
import {Button, Checkbox, Form, Layout, Input} from "antd";
import {useTypedSelector} from "../hooks/useTypedSelector.ts";
import {useActions} from "../hooks/useAction.ts";

const Login: FC = () => {
    const {isLoading, error} = useTypedSelector(state => state.authReducer);
    const [username, setUsername] = useState('')
    const [password, setPassword] = useState('')
    const {login} = useActions()

    const submit = () => {
        login(username, password)
    }

    return (
        <Layout className={"grid min-h-full h-screen place-items-center px-6 py-24 sm:py-32 lg:px-8"}>
            <div className={"px-11 py-24 bg-white shadow-2xl rounded-lg w-auto sm:w-4/5 md:w-96"}>
                <Form
                    name="basic"
                    initialValues={{remember: true}}
                    autoComplete="off"
                    layout={"vertical"}
                    onFinish={submit}
                >
                    {error &&
                    <div className={"text-red-600 text-center mb-10 text-base"}>{error}</div>
                    }
                    <Form.Item
                        label="Username"
                        name="username"
                        rules={[{required: true, message: 'Please input your username!'}]}
                        className={"mb-10"}
                    >
                        <Input
                            value={username}
                            onChange={e => setUsername(e.target.value)}
                        />
                    </Form.Item>

                    <Form.Item
                        label="Password"
                        name="password"
                        rules={[{required: true, message: 'Please input your password!'}]}
                        className={"mb-6"}
                    >
                        <Input
                            value={password}
                            onChange={e => setPassword(e.target.value)}
                            type={"password"}
                        />
                    </Form.Item>

                    <Form.Item
                        name="remember"
                        valuePropName="checked"
                    >
                        <Checkbox>Remember me</Checkbox>
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" className={"w-full"} loading={isLoading}>
                            Submit
                        </Button>
                    </Form.Item>
                </Form>
            </div>
        </Layout>
    );
};

export default Login;