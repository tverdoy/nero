import IUser from "../models/IUser.ts";

export interface IAuth {
    token: string,
    user: IUser
}