use std::error;
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};

use lib::*;

#[derive(Clone, Copy, Debug)]
enum Opcion {
    BloquearTarjeta,
    DesbloquearTarjeta,
    RegistrarTarjeta,
    RegistrarBanco,
    AgregarDineroAtm,
    Salir
}

impl TryFrom<u8> for Opcion {
    type Error = Error;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use Opcion::*;
        let tipo = match value {
            x if x == 1 => DesbloquearTarjeta,
            x if x == 2 => BloquearTarjeta,
            x if x == 3 => RegistrarTarjeta,
            x if x == 4 => RegistrarBanco,
            x if x == 5 => AgregarDineroAtm,
            x if x == 6 => Salir,
            _ => {
                return Err(anyhow!("Opción no valida"));
            }
        };
        Ok(tipo)
    }
}

fn menu() -> Result<Opcion> {
    println!("1. Desbloquear tarjeta");
    println!("2. Bloquear tarjeta");
    println!("3. Registrar Tarjeta");
    println!("4. Registrar Banco");
    println!("5. Agregar Dinero al cajero");
    println!("6. Salir");
    let opcion: u8 = input("Elige una opción: ")?.parse()?;
    Opcion::try_from(opcion)
}
async fn block_unlock(connection: &Pool<MySql>, block: bool) -> Result<()> {
    loop {
        let tarjeta = input(&format!("Ingrese la tarjeta que desea {}: ", if block { "bloquear" } else { "desbloquear" }))?;
        let q = format!("select * from cards where number = '{tarjeta}'");
        let mut desbloquear = make_query::<Card>(&q, connection).await;

        let desbloquear = if let Ok(desbloquear) = desbloquear {
            desbloquear
        } else {
            println!("No existe la tarjeta");
            continue;
        };

        if desbloquear.len() == 1  {
            let mut desbloquear = desbloquear.into_iter().next().unwrap(); // para no poner [0]
            println!("{q}");
            let _ = sqlx::query(
                &format!(r#"UPDATE cards SET expired = {} WHERE number = "{}""#, if block { '1' } else { '0' }, desbloquear.number),
            ).execute(connection).await?;
            desbloquear.expired = block;
            println!("{:?}",desbloquear);
            break;
        }else {
            println!("Se encontró más de un resultado");
            continue;
        }
    }
    Ok(())
}

async fn app(atm: &mut Atm, connection: &sqlx::Pool<MySql>) -> Result<()>  {
    loop {
        let opcion =  menu();
        let opcion = if let Ok(op) = opcion {
            op
        } else {
            println!("Error: {}", opcion.unwrap_err().to_string());
            continue;
        };

        match opcion {
            Opcion::DesbloquearTarjeta => {
                block_unlock(connection, false).await?;
            },
            Opcion::BloquearTarjeta => {
                block_unlock(connection, true).await?;
            },
            Opcion::RegistrarTarjeta => {
                let tarjeta = loop {
                    let tarjeta = input("Ingrese la tarjeta que desea registrar: ")?;
                    if tarjeta.len() != 16 {
                        println!("Error: La tarjeta debe ser de 16 dígitos");
                        continue;
                    } else if let Err(_) = tarjeta.parse::<u64>() {
                        println!("Error: La tarjeta debe contener solo numeros");
                        continue;
                    }

                    let q = format!("select * from cards where number = '{tarjeta}'");
                    let mut registrar = make_query_expect_empty::<Card>(&q, connection).await;

                    if let Err(registrar) = registrar {
                        println!("Ya existe la tarjeta");
                        continue;
                    } else {
                         break tarjeta
                    }
                };

                    let banco = loop {
                        let banco = input("Ingrese el banco de la tarjeta: ")?;
                        let q = format!("select * from bancos where name = '{banco}'");
                        let mut registrar = make_query::<Banco>(&q, connection).await;

                        let registrar = if let Ok(registrar) = registrar {
                            registrar
                        } else {
                            println!("No existe el banco");
                            continue;
                        };
                        if registrar.len() ==1  {
                            break registrar[0].id
                        } else {
                            println!("Algo salio mal al obtener el banco");
                            continue;
                        }
                    };

                    let cvv = loop {
                        let cvv = input("Ingrese el cvv de la tarjeta: ")?;
                        let cvv = cvv.parse::<u16>();
                        if let Ok(cvv) = cvv {
                            if cvv < 999 {
                                break cvv;
                            } else {
                                println!("El cvv debe ser de 3 dígitos");
                                continue;
                            }
                        } else {
                            println!("El cvv debe ser un número");
                            continue;
                        }
                    };

                    let nip = loop {
                        let nip = input("Ingrese el nip de la tarjeta: ")?;
                        let nip = nip.parse::<u16>();
                        if let Ok(nip) = nip {
                            if nip < 9999 {
                                break nip;
                            } else {
                                println!("El cvv debe ser de 3 dígitos");
                                continue;
                            }
                        } else {
                            println!("El cvv debe ser un número");
                            continue;
                        }
                    };

                    let expiration_date = loop {
                        let expiration_date = input("Ingrese la fecha de expiración: ")?;
                        if let Ok(_) = sqlx::types::time::Date::parse(&expiration_date, &time::macros::format_description!("[year]-[month]-[day]")) {
                            break expiration_date;
                        } else {
                            println!("Ingresa la fecha con el formato [year]-[month]-[day]");
                            continue
                        }
                    };

                    let balance = loop {
                        let balance: Result<f64, _> = input("Ingrese el dinero: ")?.parse();
                        let balance: f64 = if let Err(e) =  balance {
                            println!("Error: {}", e);
                            continue;
                        } else {
                            balance.unwrap()
                        };

                        if balance > 0. {
                            break balance;
                        } else {
                            println!("Ingresa un número mayor a 0");
                            continue
                        }
                    };

                    let rtype = loop {
                        let rtype = input("Ingrese el tipo de tarjeta: ")?;
                        if rtype == "Debit" || rtype == "Credit" {
                            break rtype;
                        }else {
                            println!("Ingrese un tipo valido");
                            continue;
                        }
                    };
                println!("{}", rtype);
                if  {rtype == "Debit"} {
                    let _ = sqlx::query(
                        &format!(
                            r#"INSERT INTO cards (number,bank_id, cvv, nip, expiration_date, balance, type,try, expired) VALUES ("{tarjeta}",{banco}, {cvv}, {nip},"{expiration_date}",{balance},"{rtype}", 0, 0)"#, ),
                    ).execute(connection).await?;
                }else {
                    let _ = sqlx::query(
                        &format!(
                            r#"INSERT INTO cards (number,bank_id, cvv, nip, expiration_date, balance, type,try, expired) VALUES ("{tarjeta}",{banco}, {cvv}, {nip},"{expiration_date}",{balance},"{rtype}", 0, 0)"#, ),
                    ).execute(connection).await?;
                }




            },
            Opcion::RegistrarBanco => {

                let banco = loop {
                    let banco = input("Ingrese el nombre del banco que desea agregar: ")?;
                    let q = format!("select * from bancos where name = '{banco}'");
                    let mut registrar = make_query_expect_empty::<Banco>(&q, connection).await;
                    if let Err(registrar) = registrar {
                        println!("Ya existe el banco");
                        continue;
                    } else {
                        break banco
                    }
                };

                let _ = sqlx::query(
                    &format!(
                        r#"INSERT INTO bancos (name) VALUES ("{banco}")"#, ),
                ).execute(connection).await?;
            },
            Opcion::AgregarDineroAtm => {
                let atm = loop {
                    let atm = input("Ingrese el nombre del cajero que desea agregarle dinero: ")?;
                    let q = format!("select * from atms where name = '{atm}'");
                    let mut registrar = make_query::<Atm>(&q, connection).await?;
                    if registrar.len() == 1 {
                        break atm;
                    }else {
                        println!("No existe el cajero");
                        continue;
                    }

                };
                let dinero = loop {
                    let dinero: Result<f64, _> = input("Ingrese el dinero: ")?.parse();
                    let dinero: f64 = if let Err(e) =  dinero {
                        println!("Error: {}", e);
                        continue;
                    } else {
                        dinero.unwrap()
                    };

                    if dinero >= 0. && dinero < 200000.  {
                        break dinero;
                    } else {
                        println!("Ingresa un número mayor a 0");
                        continue
                    }

                };
                let _ = sqlx::query(
                    &format!(
                        r#"UPDATE atms SET money = {dinero} WHERE name = "{atm}" "#, ),
                ).execute(connection).await?;

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
    let connection = MySqlPool::connect("mysql://suadmin:1234@localhost/banco").await.unwrap();
    let mut atm = get_atm(&connection).await.unwrap();

    println!("{atm:?}");

    loop {
        match app(&mut atm, &connection).await {
            Err(error) => {
                println!("Error: {}", error);
            },
            Ok(_) => {break;}
        }
    }
}