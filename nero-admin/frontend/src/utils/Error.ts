
export default interface IError {
    message: string,
    code?: number,
    kind: string
}

export enum ErrorKindEnum {
    AUTH = "Auth",
    RESPONSE_EMPTY = "ResponseEmpty",
    FRONTEND = "Frontend"
}