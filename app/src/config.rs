use std::env;

use log::info;


#[derive(Clone)]
pub struct ApplicationConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db: String,
    pub table: String,
    pub port: u16,
    pub secret: Option<String>
}

impl ApplicationConfig {
    pub fn from_env() -> ApplicationConfig {
        // db config
        let env_var_usr     = "POOL_USER";
        let env_var_pwd     = "POOL_PASSWORD";
        let env_var_host    = "POOL_HOST";
        let env_var_db      = "POOL_DATABASE";
        let env_var_table   = "POOL_TABLE";
        // app config
        let env_var_port    = "APP_PORT"; // has default value 8080
        let env_var_secret  = "APP_SECRET";

        // func to get var or panic with error when environment variable wasn't found
        let get_env_var = | env_var: &str | env::var(env_var)
            .expect(format!("Couldn't get environment variable for [{}]", env_var).as_str());

        let user     = get_env_var(env_var_usr);
        let password = get_env_var(env_var_pwd);
        let host     = get_env_var(env_var_host);
        let db       = get_env_var(env_var_db);
        let table    = get_env_var(env_var_table);
        let port     = match env::var(env_var_port) {
            Ok(value) => value.parse::<u16>().unwrap_or(8080 as u16),
            Err(_) => 8080 as u16
        };
        let secret   = match env::var(env_var_secret) {
            Ok(value) => Some(value),
            Err(_) => None
        };

        info!("Found all required environment variable");
        info!("Will use: User: [{}], Host: [{:?}], DB: [{}], Table: [{}], Port: [{}]", user, host, db, table, port);

        ApplicationConfig { user, password, host,db, table, port, secret }
    }
}
