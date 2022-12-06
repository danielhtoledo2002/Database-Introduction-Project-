use std::io::Write;
use anyhow::{Result, Error, anyhow};
use sqlx::mysql::MySqlPool;
use sqlx::MySql;

#[derive(Clone)]
#[derive(Debug, sqlx::FromRow)]
pub struct Banco {
    pub id: u32,
    pub name: String
}

#[derive(Clone)]
#[derive(Debug, sqlx::FromRow, PartialEq, PartialOrd)]
pub struct Card {
    pub number:String,
    pub bank_id:u32,
    pub cvv:u32,
    pub nip:i32,
    pub expiration_date: sqlx::types::time::Date,
    pub balance:f64,
    pub r#type:String,
    pub expired:bool,
    pub r#try:u32,
}
#[derive(Clone)]
#[derive(Debug, sqlx::FromRow, PartialEq, PartialOrd)]
pub struct Deuda {
    pub id: u32,
    pub number:String,
    pub r#type:String,
    pub deuda:f64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Atm {
    pub name:String,
    pub address:String,
    pub bank_id:u32,
    pub money:f64,
}

pub fn input(msg: &str) -> Result<String> {
    let mut h = std::io::stdout();
    write!(h, "{}", msg).unwrap();
    h.flush().unwrap();

    let mut campos = String::new();
    let _ = std::io::stdin().read_line(&mut campos)?;
    Ok(campos.trim().to_string())
}

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


pub async fn get_atm(connection: &sqlx::Pool<MySql>) -> Result<Atm> {
    if let Some(addr) = mac_address::get_mac_address()? {

        let q = format!("select * from atms where name = '{addr}'");
        let mut result = make_query::<Atm>(&q, connection).await;

        let result = if result.is_err() {
            println!("Error: {:?}", result.unwrap_err());
            let insert_q = format!("insert into atms (name, address, bank_id , money)
values ('{addr}', 'Oso 81, Col del Valle Centro, Benito Juárez, 03100 Ciudad de México, CDMX', 2
, 200000.0)");
            sqlx::query(&insert_q)
                .fetch_all(connection)
                .await?;

            make_query::<Atm>(&q, connection).await?
        } else {
            result?
        };

        if result.len() == 1 {
            Ok(result.into_iter().next().unwrap())
        } else {
            Err(anyhow!("Se encontraron varios ATMs"))
        }

    } else {
        Err(anyhow!("No se pudo obtener la MAC"))
    }
}