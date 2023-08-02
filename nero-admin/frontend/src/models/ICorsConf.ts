export default interface ICorsConf {
    is_allow_cors: boolean,
    allow_origin: string,
    allow_headers: string[],
    allow_methods: string[]
}

