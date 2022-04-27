use std::convert::TryFrom;

use log::debug;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, MySql};
use futures::TryStreamExt;

macro_rules! json_error {
    ($e:expr) => {
        Err ( serde_json::json!({ "error": $e.to_string() }) )
    };
}

macro_rules! resolve_err {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return json_error!(err.to_string()),
        }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i64>,
    pub content: String
}

#[derive(Clone)]
pub struct SqlItemService;

// query in each fn is executed or fetched depending on the method
// handling is solved via macro resolve_err or individually for each fn
impl SqlItemService {
    /// retrieves all entries of self.table
    pub async fn get_items(
        pool: &Pool<MySql>,
        table: String
    ) -> Result<Vec<Item>, serde_json::Value> {
        let query = format!("SELECT * FROM {};", table);
        let mut rows = sqlx::query_as::<_, Item>(query.as_str())
            .fetch(pool);
        let mut items: Vec<Item> = Vec::new();
        // resolve the next row until next is None, on error returns QueryError (see resolve_err)
        while let Some(row) = resolve_err!(rows.try_next().await) {
            items.push(row);
        }
        // returns a vector of all items in the database
        Ok(items)
    }

    /// adds an entry in self.table with the provided context
    pub async fn add_item(
        pool: &Pool<MySql>,
        table: String,
        content: String
    ) -> Result<Item, serde_json::Value> {
        let query = format!("INSERT INTO {} (content) VALUES ('{}');", table, content);
        // execute query and get last inserted id to return it
        let id = resolve_err!(sqlx::query(query.as_str())
            .execute(pool)
            .await)
            .last_insert_id();
        // conversion of u64 to i64, on fail throw error
        match i64::try_from(id) {
            Ok(id) => Ok( Item { id: Some(id), content } ),
            Err(err) => json_error!(err.to_string())
        }
    }

    /// retrieves item by provided id from self.table
    pub async fn get_item_by_id(
        pool: &Pool<MySql>,
        table: String,
        id: String
    ) -> Result<Item, serde_json::Value> {
        let query = format!("SELECT * FROM {} WHERE ID = {};", table, id);
        let mut rows =
            sqlx::query_as::<_, Item>(query.as_str())
                .fetch(pool);

        // gets first matching row if query was successful
        match rows.try_next().await {
            Ok(op_item) => {
                debug!("got item [{:?}] for id [{}]", op_item, id);
                // checks if item was available
                match op_item {
                    Some(item) => Ok(item),
                    None => json_error!("No SQL Error but no item was found to the given id".to_string())
                }
            },
            Err(err) => json_error!(err.to_string())
        }
    }

    /// deletes item by provided id from self.table
    pub async fn delete_item_by_id(
        pool: &Pool<MySql>,
        table: String,
        id: String
    ) -> Result<serde_json::Value, serde_json::Value> {
        let query = format!("DELETE FROM {} WHERE ID = {};", table, id);
        let affected = resolve_err!(sqlx::query(query.as_str())
                .execute(pool)
                .await)
                .rows_affected();
        debug!("deleted [{}] rows", affected);

        if affected == 1 {
            Ok( serde_json::json!({}) )
        } else {
            json_error!("No SQL Error but no row was effected".to_string())
        }
    }
}
