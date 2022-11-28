use sqlx::mysql::MySqlPool;
use sqlx::MySql;
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};
use tokio::time::error::Elapsed;

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
                if card.r#type == "Debit" {
                    println!("El dinero en la cuenta es {} $ pesos", card.balance);
                } else {
                    let deudas = make_query::<Deuda>(
                        format!("Select * from deudas where number = {}",card.number),
                        connection
                    ).await?;

                    let deuda = if deudas.len() != 1 {
                        println!("Ocurrió un error al leer la deuda");
                        continue;
                    } else {
                        deudas.into_iter().next().unwrap()
                    };

                    println!("Su crédito restante es {}-{} = {}", card.balance, deuda.deuda, card.balance-deuda.deuda);
                }
            },
            Opcion::RetirarEfectivo => {
                let opcion = elegir_dinero()? as i32 as f64;

                let dinero = card.balance - opcion;

                let atm_dinero = atm.money - opcion;

                if !(dinero >= 0.  && atm_dinero >= 0.) {
                    println!("No tienes suficiente dinero");
                    continue
                }

                if card.r#type == "Debit" {
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
                    let deudas = make_query::<Deuda>(
                        format!("Select * from deudas where number = {}",card.number),
                        connection
                    ).await?;

                    if deudas.len() != 1 {
                        println!("No encontró la tarjeta");
                        continue
                    }

                    let dinero_credito = deudas[0].deuda + opcion + (opcion*0.03);
                    if dinero_credito >= 0. && dinero_credito <= card.balance {
                        let _ = sqlx::query(
                            &format!(r#"UPDATE deudas SET deuda = {dinero_credito} WHERE number = "{}""#, card.number),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"UPDATE atms SET money = {atm_dinero} WHERE name = "{}""#, atm.name),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"INSERT INTO withdrawals (amount, atm_name, card_number) VALUES ({},"{}","{}")"#, opcion as i32, atm.name, card.number),
                        ).execute(connection).await?;

                    } else {
                        println!("No tienes deuda o saldo mayor al que hay que pagar");
                        break
                    }
                }

            },
            Opcion::DepositarEfectivo => {
                let opcion = elegir_dinero()?;
                let dinero = card.balance + opcion as i32 as f64;
                let atm_dinero = atm.money + opcion as i32 as f64;
                if card.r#type == "Debit" {
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
                }else {
                    let mut deudas = make_query::<Deuda>(
                        format!("Select * from deudas where number = {}",card.number),
                        connection
                    ).await?;
                    if deudas.len() == 1 {
                        let dinero_deuda = deudas[0].deuda - opcion as i32 as f64;
                        if dinero_deuda >= 0. {
                            let _ = sqlx::query(
                                &format!(r#"UPDATE deudas SET deuda = {dinero_deuda} WHERE number = "{}""#, card.number),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                &format!(r#"UPDATE atms SET money = {atm_dinero} WHERE name = "{}""#, atm.name),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                &format!(r#"INSERT INTO deposits (amount, card_number, atm_name) VALUES ({}, "{}","{}")"#, opcion as i32,card.number ,atm.name),
                            ).execute(connection).await?;
                            deudas[0].deuda = dinero_deuda;

                        }
                        else {
                            println!("No tienes deuda o saldo mayor al que hay que pagar");
                            break
                        }
                    }else {
                        println!("No encontró la tarjeta");
                    }
                }
            },

            Opcion::TransferirEfectivo => {
                loop {
                    if card.r#type == "Credit" {
                        println!("No puedes realizar esta operación con una tarjeta de crédito");
                        continue;
                    }

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

                    let target = if target.len() != 1  {
                        println!("No existe la tarjeta");
                        continue;
                    } else {
                        target.into_iter().next().unwrap()
                    };

                    let opcion = elegir_dinero()? as i32 as f64;
                    let dinero = card.balance - opcion;

                    if target.r#type == "Debit" {
                        let _ = sqlx::query(
                            &format!(r#"UPDATE cards SET balance = {dinero} WHERE number = {}"#, card.number),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"UPDATE cards SET balance = {} WHERE number = {}"#, target.balance + opcion, target.number),
                        ).execute(connection).await?;
                        let _ = sqlx::query(
                            &format!(r#"INSERT INTO transfers (amount, sent_money, received_money) VALUES ({}, "{}","{}")"#, opcion, card.number, target.number),
                        ).execute(connection).await?;
                        card.balance = dinero;
                        println!("Transferencia exitosa {} ", card.balance);
                        break;
                    } else {
                        let deudas = make_query::<Deuda>(
                            format!("Select * from deudas where number = {}", target.number),
                            connection
                        ).await?;

                        let mut deuda = if deudas.len() != 1 {
                            println!("No encontró la tarjeta");
                            continue;
                        } else {
                            deudas.into_iter().next().unwrap()
                        };

                        let dinero_deuda = deuda.deuda - opcion;
                        println!("{} | {dinero_deuda} : {} - {}", target.balance, deuda.deuda, opcion);

                        if dinero_deuda >= 0. && dinero_deuda <= target.balance {
                            let _ = sqlx::query(
                                &format!(r#"UPDATE cards SET balance = {dinero} WHERE number = {}"#, card.number),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                &format!(r#"UPDATE deudas SET deuda = {dinero_deuda} WHERE number = "{}""#, target.number),
                            ).execute(connection).await?;
                            let _ = sqlx::query(
                                &format!(r#"INSERT INTO transfers (amount, sent_money, received_money) VALUES ({}, "{}","{}")"#, opcion, card.number, target.number),
                            ).execute(connection).await?;
                            deuda.deuda = dinero_deuda;
                            println!("Transferencia exitosa {} ", card.balance);
                            break;
                        }
                        else {
                            println!("No tienes deuda o saldo mayor al que hay que pagar");
                            break
                        }
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
    let connection = MySqlPool::connect("mysql://daniel:1234@localhost/banco").await.unwrap();
    let mut atm = get_atm(&connection).await.unwrap();

    loop {
        match app(&mut atm, &connection).await {
            Err(error) => {
                println!("Error: {}", error);
            },
            Ok(_) => {}
        }
    }
}
