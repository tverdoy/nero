export default interface IApp {
    name: string,
    schemes: IScheme[]
}
export interface IScheme {
    name: string,
    fields: IField[]
}

export interface IField {
    name: string,
    type: FieldTypeEnum,
    args: IFieldArg[]
}

export enum FieldTypeEnum {
    INT = "Int",
    STRING = "String",
    BOOL = "Bool",
    LINK_TO = "LinkTo"
}

export interface IFieldArg {
    max_length?: number,
    default?: string
}