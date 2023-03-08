//一个模拟的数据连接
pub struct Datasource {
    //
}

pub trait Connection {
    fn query<T>(&self, data: T) -> Vec<T>;
}

pub struct ConnectionImpl {}

impl Connection for ConnectionImpl {
    //模拟查询返回一个列表
    fn query<T>(&self, data: T) -> Vec<T> {
        vec![data]
    }
}

pub fn get_connection() -> ConnectionImpl {
    //模拟获取到一个数据库连接的操作
    // HikariDatasource {}

    ConnectionImpl {}
}
