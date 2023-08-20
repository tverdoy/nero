import {IModel} from "../models/IApp.ts";
import {ColumnsType} from "antd/es/table";
import {Table} from "antd";
import record from "../pages/Record.tsx";
import {formatThing} from "../utils";

type ModelTableProps = {
    model: IModel,
    records: any[],
    onClickRecord: (recordId: string) => void;
}


const ModelTable = ({model, records, onClickRecord}: ModelTableProps) => {
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
            render: (text) => {
                let id = formatThing(text);
                return <a onClick={() => onClickRecord(id)}>{id}</a>
            },
            ...columns[0]
        }
    }


    return (
        <Table columns={columns} dataSource={records} scroll={{ y: "60vh" }} pagination={{ pageSize: 20 }}/>
    );
};

export default ModelTable;