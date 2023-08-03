import IServerConf from "./IServerConf.ts";
import IDataBaseConf from "./IDataBaseConf.ts";
import ICorsConf from "./ICorsConf.ts";
import IAuthTokenConf from "./IAuthTokenConf.ts";

export default interface ISettings {
    server: IServerConf,
    db: IDataBaseConf,
    cors: ICorsConf,
    admin_auth: IAuthTokenConf
}