# Database-Introduction-Project-
# Sql 
I use sql to design a database to use in atm machine.
![image](https://user-images.githubusercontent.com/92064764/204880114-458ffd65-e28d-4f82-aab1-ae64acb39a5f.png)

# SQLX 
I use sqlx to update the database.
```Rust
pub async fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
    where for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin
{
    let result: Vec<T> = sqlx::query_as::< _,T> (query.as_ref())
        .fetch_all(connection)
        .await ?;

    if result.is_empty() {
        Err(anyhow!("No se encontró ningún dato"))
    } else {
        Ok(result)
    }
}


pub async fn make_query_expect_empty<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
    where for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin
{
    let result: Vec<T> = sqlx::query_as::< _,T> (query.as_ref())
        .fetch_all(connection)
        .await ?;

    if !result.is_empty() {
        Err(anyhow!("Se encontró uno o más datos"))
    } else {
        Ok(result)
    }
}
```

# How to run ?
* To run the ddl use any sql compiler 
* To compile the user app use `cargo run --bin usuarios`
* To compile the admin app use `cargo run --bin admins`
