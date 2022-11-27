use std::io::Write;
use std::os::raw::c_int;
use sqlx::mysql::MySqlPool;
use std::ptr::null;
use sqlx::MySql;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};
use crate::MoneyOptions::{Q100, Q1000, Q200, Q2000};
use crate::Opcion::{ConsultarSaldo, DepositarEfectivo, RetirarEfectivo, TransferirEfectivo};

#[derive(Clone, Copy)]
enum Opcion {
    ConsultarSaldo,
    RetirarEfectivo,
    DepositarEfectivo,
    TransferirEfectivo,
    Salir
}

#[derive(Clone, Copy)]
enum MoneyOptions {
    Q100    = 100,
    Q200    = 200,
    Q500    = 500,
    Q1000   = 1000,
    Q2000   = 2000,
    Q4000   = 4000,
}

#[derive(Clone)]
#[derive(Debug, sqlx::FromRow)]
struct Card {
    number:String,
    bank_id:u32,
    cvv:u32,
    nip:i32,
    expiration_date: sqlx::types::time::Date,
    balance:f64,
    r#type:String,
    expired:bool,
    r#try:u32,
}

macro_rules! debug {
    ( $($t:tt)* ) => {
        {
            #[cfg(debug_assertions)]
            println!($($t)*);
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct Atm {
    name:String,
    address:String,
    bank_id:u32,
    money:f64,
}

fn input(msg: &str) -> Result<String> {
    let mut h = std::io::stdout();
    write!(h, "{}", msg).unwrap();
    h.flush().unwrap();

    let mut campos = String::new();
    let _ = std::io::stdin().read_line(&mut campos)?;
    Ok(campos.trim().to_string())
}

async fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
    where
        for<'a, > T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin
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

async fn get_atm(connection: &sqlx::Pool<MySql>) -> Result<Atm> {
    if let Some(addr) = mac_address::get_mac_address()? {

        let q = format!("select * from atms where name = '{addr}'");
        let mut result = make_query::<Atm>(&q, connection).await;

        let result = if result.is_err() {
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

async fn iniciar_sesion(connection: &sqlx::Pool<MySql>) -> Result<Card> {
    let tarjeta = input("Ingrese el número de tarjeta: ")?;
    let nip = input("Ingrese el nip: ")?;

    let query = format!(r#"select * from cards where number = "{tarjeta}" and nip = "{nip}" "# );
    let cartas =  make_query::<Card>(query, connection).await?;
    if cartas.len() == 1 {
        Ok(cartas.into_iter().next().unwrap())
    } else {
        Err(anyhow!("Se encontró mas de un dato"))
    }
}


impl TryFrom<u8> for MoneyOptions {
    type Error = Error;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use MoneyOptions::*;
        let tipo = match value {
            x if x == 1 => Q100,
            x if x == 2 => Q200,
            x if x == 3 => Q500,
            x if x == 4 => Q1000,
            x if x == 5 => Q2000,
            x if x == 6 => Q4000,
            _ => {
                return Err(anyhow!("Opción no valida"));
            }
        };
        Ok(tipo)
    }
}



impl TryFrom<u8> for Opcion {
    type Error = Error;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use Opcion::*;
        let tipo = match value {
            x if x == 1 => ConsultarSaldo,
            x if x == 2 => RetirarEfectivo,
            x if x == 3 => DepositarEfectivo,
            x if x == 4 => TransferirEfectivo,
            x if x == 5 => Salir,
            _ => {
                return Err(anyhow!("Opción no valida"));
            }
        };
        Ok(tipo)
    }
}

fn menu() -> Result<Opcion> {
    println!("1. Consultar saldo");
    println!("2. Retirar efectivo");
    println!("3. Depositar efectivo");
    println!("4. Transferir efectivo");
    println!("5. Salir");

    let opcion: u8 = input("Elige una opción: ")?.parse()?;
    Opcion::try_from(opcion)
}

fn elegir_dinero() -> Result<MoneyOptions> {
    let opcion = loop {
        println!("1. 100 $ pesos");
        println!("2. 200 $ pesos");
        println!("3. 500 $ pesos");
        println!("4. 1000 $ pesos");
        println!("5. 2000 $ pesos");
        println!("6. 4000 $ pesos");
        let opcion: u8 = input("Elige una opción: ")?.parse()?;

        if let Ok(money) = MoneyOptions::try_from(opcion) {
            break money;
        } else {
            println!("Opción no válida");
        }
    };
    Ok(opcion)
}
async fn app(atm: &mut Atm, connection: &sqlx::Pool<MySql>) -> Result<()>  {

    let mut card = iniciar_sesion(connection).await?;
    println!("Result2: {:#?}", card);
    println!("Bienvenido");
    loop {
        match  menu(){
             Ok(opcion) => {
                match opcion {
                    ConsultarSaldo => {
                        println!("El dinero en la cuenta es {} $ pesos", card.balance);
                    },
                    RetirarEfectivo => {
                        let opcion = elegir_dinero()?;

                        let dinero = card.balance - opcion as i32 as f64;
                        let atm_dinero = atm.money - opcion as i32 as f64;
                        if dinero >= 0.  && atm_dinero >= 0. {
                            let _ = sqlx::query(
                                &format!(r#"UPDATE cards SET balance = {dinero} WHERE number = {}"#, card.number),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                &format!(r#"UPDATE atms SET money = {atm_dinero} WHERE name = "{}""#, atm.name),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                 &format!(r#"INSERT INTO withdrawals (amount, atm_name, card_number) VALUES ({},"{}","{}")"#, opcion as i32, atm.name, card.number),
                            ).execute(connection).await?;

                            card.balance = dinero;
                            atm.money = atm_dinero;
                            println!("Retiro exitoso {}", card.balance);
                        } else {
                            println!("No tienes suficiente dinero");
                        }

                    },
                    DepositarEfectivo => {
                        let opcion = elegir_dinero()?;
                        let dinero = card.balance + opcion as i32 as f64;
                        let atm_dinero = atm.money + opcion as i32 as f64;
                        let _ = sqlx::query(
                            &format!(r#"UPDATE cards SET balance = {dinero} WHERE number = {}"#, card.number),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"UPDATE atms SET money = {atm_dinero} WHERE name = "{}""#, atm.name),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"INSERT INTO deposits (amount, card_number, atm_name) VALUES ({}, "{}","{}")"#, opcion as i32,card.number ,atm.name),
                        ).execute(connection).await?;
                        card.balance = dinero;
                        atm.money = atm_dinero;
                        println!("Deposito exitoso {} ", card.balance);
                        debug!("ATM dinero: {}", atm.money);
                    },
                    TransferirEfectivo => {
                        loop {
                            let numero_tarjeta = input("Ingresa el número de la tarjeta: ")?;

                            if numero_tarjeta == card.number {
                                println!("No puedes transferirte a ti mismo");
                                continue;
                            }

                            let q = format!("select * from cards where number = '{numero_tarjeta}'");
                            let mut target = make_query::<Card>(&q, connection).await;

                            let target = if let Ok(target) = target {
                                target
                            } else {
                                println!("No existe la tarjeta");
                                continue;
                            };

                            if target.len() == 1  {
                                let opcion = elegir_dinero()?;
                                let dinero = card.balance - opcion as i32 as f64;
                                if dinero > 0. && card.r#type == "Debit"{
                                    let _ = sqlx::query(
                                        &format!(r#"UPDATE cards SET balance = {dinero} WHERE number = {}"#, card.number),
                                    ).execute(connection).await?;
                                    let _ = sqlx::query(
                                        &format!(r#"UPDATE cards SET balance = {} WHERE number = {}"#, target[0].balance + opcion as i32 as f64, target[0].number),
                                    ).execute(connection).await?;
                                    let _ = sqlx::query(
                                        &format!(r#"INSERT INTO transfers (amount, sent_money, received_money) VALUES ({}, "{}","{}")"#, opcion as i32, card.number, target[0].number),
                                    ).execute(connection).await?;
                                    card.balance = dinero;
                                    println!("Transferencia exitosa {} ", card.balance);
                                    break;
                                }else{
                                    println!("No tienes suficiente dinero");
                                }
                            }else {
                                println!("No existe la tarjeta");
                            }
                        }
                    },
                    _ => {
                        println!("Salir");
                        break;
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}



#[tokio::main]
async fn main() {
    let connection = MySqlPool::connect("mysql://root:1234@localhost/Banco").await.unwrap();
    let mut atm = get_atm(&connection).await.unwrap();

    println!("{atm:?}");

    loop {
        match app(&mut atm, &connection).await {
            Err(error) => {
                println!("Error: {}", error);
            },
            Ok(_) => {}
        }
    }
}
