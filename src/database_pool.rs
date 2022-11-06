use diesel::mysql::MysqlConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;

#[derive(Debug)]
pub struct DatabasePool{
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

impl DatabasePool {
    pub fn connect_mysql(url: &str) ->Self{
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        let pool = Pool::builder()
        .build(manager)
        .unwrap();
        Self{
            pool
        }
    }
}
