export default interface IError {
    message: string,
    kind: string
}

export enum ErrorKindEnum {
    AUTH = "Auth",
    RESPONSE_EMPTY = "ResponseEmpty",
    FRONTEND = "Frontend",
    Other = "Other"
}
