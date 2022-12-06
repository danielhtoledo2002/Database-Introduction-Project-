use sqlx::mysql::MySqlPool;
use sqlx::MySql;
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};
use tokio::time::error::Elapsed;
use dioxus::prelude::*;
use axum::{Router, Server, routing::{get, post}, response::Html, Form};
use std::net::SocketAddr;
use lib::*;
use tower_http::services::ServeDir;
use axum::error_handling::HandleError;
use axum::http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use axum::response::IntoResponse;

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
#[derive(Deserialize)]
struct SignIn {
    user:String,
    password:String,
}

async fn verification_login(Form(datos) : Form<SignIn>, estado: Arc<Estado>) -> Html<String> {
    Html(format!("{}:{}", datos.user, datos.password))
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

async fn map(atm: &mut Atm, connection: &sqlx::Pool<MySql>) -> Result<()>  {

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

fn Main(cx: Scope) -> Element{
    let script = r#"
    function post(path, params, method='post') {

  // The rest of this code assumes you are not using a library.
  // It can be made less verbose if you use one.
  const form = document.createElement('form');
  form.method = method;
  form.action = path;

  for (const key in params) {
    if (params.hasOwnProperty(key)) {
      const hiddenField = document.createElement('input');
      hiddenField.type = 'hidden';
      hiddenField.name = key;
      hiddenField.value = params[key];

      form.appendChild(hiddenField);
    }
  }

  document.body.appendChild(form);
  form.submit();
}

document.addEventListener("DOMContentLoaded", function(event) {
   var form = document.getElementById("myForm");
   function handleForm(event) {
        event.preventDefault();

        var user = document.getElementById("user").value;
        var password = document.getElementById("password").value;

        post("/login", {user: user, password: password});
    }
   form.addEventListener('submit', handleForm);
});
    "#;
    cx.render(rsx! {
        link {
            rel:"stylesheet",
            href:"/static/tailwindcss.css"
        },
        script {
            dangerous_inner_html: "{script}"
        },
        section { class: "h-screen",
        div { class: "px-6 h-full text-gray-800",
            div { class: "flex xl:justify-center lg:justify-between justify-center items-center flex-wrap h-full g-6",
                div { class: "grow-0 shrink-1 md:shrink-0 basis-auto xl:w-6/12 lg:w-6/12 md:w-9/12 mb-12 md:mb-0",
                    img { class: "w-full",
                        src: "./static/bank_image.jpg",
                        alt: "Sample image",
                    }
                }
                div { class: "xl:ml-20 xl:w-5/12 lg:w-5/12 md:w-8/12 mb-12 md:mb-0",
                    form {
                        id: "myForm",
                        action: "/login",
                        method: "post",
                        /* Email input */
                        div { class: "mb-6",
                            input { class: "form-control block w-full px-4 py-2 text-xl font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                                id: "user",
                                name: "user",
                                r#type: "text",
                                placeholder: "Email address",
                            }
                        }
                        /* Password input */
                        div { class: "mb-6",
                            input { class: "form-control block w-full px-4 py-2 text-xl font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                                id: "password",
                                r#type: "password",
                                name: "password",
                                placeholder: "Password",
                            }
                        }
                        div { class: "flex justify-between items-center mb-6",
                            div { class: "form-group form-check",
                                input { class: "form-check-input appearance-none h-4 w-4 border border-gray-300 rounded-sm bg-white checked:bg-blue-600 checked:border-blue-600 focus:outline-none transition duration-200 mt-1 align-top bg-no-repeat bg-center bg-contain float-left mr-2 cursor-pointer",
                                    id: "exampleCheck2",
                                    r#type: "checkbox",
                                }
                                label { class: "form-check-label inline-block text-gray-800",
                                    r#for: "exampleCheck2",
                                    "Remember me"
                                }
                            }
                            a { class: "text-gray-800",
                                href: "#!",
                                "Forgot password?"
                            }
                        }
                        div { class: "text-center lg:text-left",
                            button { class: "inline-block px-7 py-3 bg-blue-600 text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out",
                                r#type: "submit",
                                "Login"
                            }
                            p { class: "text-sm font-semibold mt-2 pt-1 mb-0",
                                "Don't have an account?"
                                a { class: "text-red-600 hover:text-red-700 focus:text-red-700 transition duration-200 ease-in-out",
                                    href: "#!",
                                    "Register"
                                    }
                                }
                             }
                        }
                    }
                }
            }
        }
    })
}
async fn render_login() -> Html<String> {

    let mut vdom = VirtualDom::new(Main);
    let _ = vdom.rebuild();
    let string = dioxus::ssr::render_vdom(&vdom);
    Html(string)
}



struct Estado {
    connection: sqlx::Pool<MySql>,
    atm: Atm
}

#[tokio::main]
async fn main() {

    let connection = MySqlPool::connect("mysql://daniel:1234@localhost/banco").await.unwrap();
    let atm = get_atm(&connection).await.unwrap();

    let estado_compartido = Arc::new(Estado {
        connection,
        atm
    });

    let servicio_archivos = HandleError::new(ServeDir::new("./static"), not_found);

    async fn not_found(err: std::io::Error) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )
    }

    let app = Router::new()
        .nest_service("/static", servicio_archivos)
        .route("/login",get(render_login).post({
            let estado = Arc::clone(&estado_compartido);
            move |datos| verification_login(datos, estado)
        }))
    ;
    let adrr = SocketAddr::from(([127, 0, 0, 1], 80));
    axum::Server::bind(&adrr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
