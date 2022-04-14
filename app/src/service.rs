use std::convert::TryFrom;

use log::debug;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use futures::TryStreamExt;

macro_rules! resolve_err {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return  Err( QueryError { error: err.to_string() } ),
        }
    }
}

#[derive(Serialize)]
pub struct QueryError {
    error: String
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Option<i64>,
    pub content: String
}

#[derive(Clone)]
pub struct ItemService {
    pub pool: MySqlPool,
    pub table: String
}

// query in each fn (except new) is executed or fetched depending on the method
// handling is solved via macro resolve_err or individually for each fn
impl ItemService {
    pub fn new(pool: MySqlPool, table: String) -> ItemService {
        ItemService { pool, table }
    }

    /// retrieves all entries of self.table
    pub async fn get_items(&self) -> Result<Vec<Item>, QueryError> {
        let query = format!("SELECT * FROM {};", self.table);
        let mut rows = sqlx::query_as::<_, Item>(query.as_str())
            .fetch(&self.pool);
        let mut items: Vec<Item> = Vec::new();
        // resolve the next row until next is None, on error returns QueryError (see resolve_err)
        while let Some(row) = resolve_err!(rows.try_next().await) {
            items.push(row);
        }
        // returns a vector of all items in the database
        Ok(items)
    }

    /// adds an entry in self.table with the provided context
    pub async fn add_item(&self, content: String) -> Result<Item, QueryError> {
        let query = format!("INSERT INTO {} (content) VALUES ('{}');", self.table, content);
        // execute query and get last inserted id to return it
        let id = resolve_err!(sqlx::query(query.as_str())
            .execute(&self.pool)
            .await)
            .last_insert_id();
        // conversion of u64 to i64, on fail throw error
        match i64::try_from(id) {
            Ok(id) => Ok( Item { id: Some(id), content } ),
            Err(err) => Err( QueryError { error: err.to_string() } )
        }
    }

    /// retrieves item by provided id from self.table
    pub async fn get_item_by_id(&self, id: String) -> Result<Item, QueryError> {
        let query = format!("SELECT * FROM {} WHERE ID = {};", self.table, id);
        let mut rows =
            sqlx::query_as::<_, Item>(query.as_str())
                .fetch(&self.pool);

        // gets first matching row if query was successful
        match rows.try_next().await {
            Ok(op_item) => {
                debug!("got item [{:?}] for id [{}]", op_item, id);
                // checks if item was available
                match op_item {
                    Some(item) => Ok(item),
                    None => Err( QueryError { error: String::from("No SQL Error but no item was found to the given id") } )
                }
            },
            Err(err) => Err( QueryError { error: err.to_string() } )
        }

    }

    /// deletes item by provided id from self.table
    pub async fn delete_item_by_id(&self, id: String) -> Result<QueryError, QueryError> {
        let query = format!("DELETE FROM {} WHERE ID = {};", self.table, id);
        let affected = resolve_err!(sqlx::query(query.as_str())
                .execute(&self.pool)
                .await)
                .rows_affected();
        debug!("deleted [{}] rows", affected);

        if affected == 1 {
            Ok( QueryError { error: String::new() } )
        } else {
            Err( QueryError { error: String::from("No SQL Error but no row was effected") } )
        }
    }
}
