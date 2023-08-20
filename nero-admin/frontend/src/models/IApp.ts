export default interface IApp {
    name: string,
    models: IModel[]
}
export interface IModel {
    scheme: IScheme
}

export interface IScheme {
    name: string,
    fields: IField[]
}

export interface IField {
    name: string,
    tp: FieldTypeEnum,
    is_option: boolean,
    attrs: IAttributes,
}

export interface IAttributes {
    max_length?: number,
    default?: string
}

export enum FieldTypeEnum {
    U8 = "u8",
    U16 = "u16",
    U32 = "u32",
    U64 = "u64",
    I8 = "i8",
    I16 = "i16",
    I32 = "i32",
    I64 = "i64",
    F32 = "f32",
    F64 = "f64",
    STRING = "String",
    THING = "Thing"
}