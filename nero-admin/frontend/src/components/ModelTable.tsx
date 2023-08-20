import {IModel} from "../models/IApp.ts";
import {ColumnsType} from "antd/es/table";
import {Table} from "antd";

type ModelTableProps = {
    model: IModel,
    onClickRecord: (recordId: string) => void;
}


const ModelTable = ({model, onClickRecord}: ModelTableProps) => {
    let columns: ColumnsType<any> = [];

    for (let field of model.scheme.fields) {
        columns.push({
            title: field.name,
            dataIndex: field.name,
            key: field.name,
        })
    }

    if (columns.length > 0) {
        columns[0] = {
            render: (text) => <a onClick={() => onClickRecord(text)}>{text}</a>,
            ...columns[0]
        }
    }

    let testData = []
    for (let i = 0; i < 30; i++) {
        testData.push({
            id: i,
            username: `User ${i}`,
            password: `P@ssw0rd`
        })
    }



    return (
        <Table columns={columns} dataSource={testData} scroll={{ y: "60vh" }} pagination={{ pageSize: 20 }}/>
    );
};

export default ModelTable;