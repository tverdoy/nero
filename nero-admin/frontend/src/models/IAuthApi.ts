import IUser from "./IUser.ts";

export interface IAuth {
    token: string,
    user: IUser
}