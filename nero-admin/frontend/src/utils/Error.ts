import INeroError from "../models/INeroError.ts";

export default interface IError {
    neroError?: INeroError,
    code: number
}

