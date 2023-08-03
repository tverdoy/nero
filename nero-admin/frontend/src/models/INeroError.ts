export default interface INeroError {
    kind: NeroKindEnum,
    error: string
}

export enum NeroKindEnum {
    Auth = "Auth",
    Other = "Other"
}