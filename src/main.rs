use sqlx::mysql::MySqlPool;
use sqlx::MySql;
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};

use lib::*;

#[derive(Debug)]
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

async fn iniciar_sesion(connection: &sqlx::Pool<MySql>) -> Result<Card> {
    let tarjeta = input("Ingrese el número de tarjeta: ")?;
    let nip = input("Ingrese el nip: ")?;

    let query = format!(r#"select * from cards where number = "{tarjeta}" and nip = "{nip}" "# );
    let cartas =  make_query::<Card>(query, connection).await?;

    let tarjeta = if cartas.len() == 1 {
        cartas.into_iter().next().unwrap()
    } else {
        return Err(anyhow!("Se encontró mas de un dato"));
    };

    if tarjeta.expired == true {
        Err(anyhow!("La tarjeta se encuentra bloqueada"))
    } else {
        Ok(tarjeta)
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
        let opcion =  menu();
        let opcion = if let Ok(op) = opcion {
            op
        } else {
            println!("Error: {}", opcion.unwrap_err());
            continue;
        };

        match opcion {
            Opcion::ConsultarSaldo => {
                println!("El dinero en la cuenta es {} $ pesos", card.balance);
            },
            Opcion::RetirarEfectivo => {
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
            Opcion::DepositarEfectivo => {
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
            },

            Opcion::TransferirEfectivo => {
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
            Opcion::Salir => {
                break;
            }
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    let connection = MySqlPool::connect("mysql://daniel:1234@201.145.156.9/banco").await.unwrap();
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
