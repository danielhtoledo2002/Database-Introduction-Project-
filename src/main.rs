use anyhow::{anyhow, Error, Result};
use dioxus::events::onchange;
use dioxus::prelude::*;
use lib::*;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use text_io::try_read;
use tokio::time::error::Elapsed;

#[derive(Clone, Debug)]
struct MyPool {
    connection: MySqlPool,
}

impl PartialEq for MyPool {
    fn eq(&self, other: &Self) -> bool {
        let a = format! {"{:?}", self.connection} == format! {"{:?}", other.connection};
        a
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Deserialize)]
struct SignIn {
    user: String,
    password: String,
}

async fn iniciar_sesion(connection: &sqlx::Pool<MySql>, tarjeta: &str, nip: &str) -> Result<Card> {
    let query = format!(
        r#"select * from cards where number = "{}" and nip = "{}" "#,
        tarjeta, nip
    );
    let cartas = make_query::<Card>(query, connection).await?;

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

async fn consultar_saldo(connection: &sqlx::Pool<MySql>, tarjeta: &str) -> Result<f64> {
    let deudas = make_query::<Deuda>(
        format!("Select * from deudas where number = {}", tarjeta),
        connection,
    )
    .await?;
    Ok(deudas[0].deuda)
}
async fn retira_dinero(connection: &sqlx::Pool<MySql>, atm: UseState<Atm>, card: UseState<Option<Card>>, opcion: f64, ) -> Result<String> {
    if card.get().is_none() {
        return Err(anyhow!("Theres no card in the state"));
    }
    let mut card_clone = card.get().as_ref().unwrap().clone();
    let mut atm_clone = atm.get().clone();
    let dinero = card_clone.balance - opcion;
    let atm_dinero = atm_clone.money - opcion;
    if !(dinero >= 0. && atm_dinero >= 0.) {
        println!("No tienes suficiente dinero");
    }
    if card_clone.r#type == "Debit" {
        let _ = sqlx::query(&format!(
            r#"UPDATE cards SET balance = {} WHERE number = {}"#,
            dinero, card_clone.number
        )).execute(connection).await?;
        let _ = sqlx::query(&format!(
            r#"UPDATE atms SET money = {} WHERE name = "{}""#,
            atm_dinero, atm_clone.name
        )).execute(connection).await?;
        let _ = sqlx::query(&format!(
            r#"INSERT INTO withdrawals (amount, atm_name, card_number) VALUES ({},"{}","{}")"#,
            opcion as i32, atm_clone.name, card_clone.number
        )).execute(connection).await?;
        let final_balance = card_clone.balance;
        card_clone.balance = dinero;
        card.set(Some(card_clone));
        atm_clone.money = atm_dinero;
        atm.set(atm_clone);
        Ok(format!("Retiro exitoso {}", final_balance))
    } else {
        let deudas = make_query::<Deuda>(
            format!("Select * from deudas where number = {}", card_clone.number),
            connection,
        ).await?;
        if deudas.len() != 1 {
            return Ok("No encontró la tarjeta".to_string());
        }
        let dinero_credito = deudas[0].deuda + opcion + (opcion * 0.03);
        if dinero_credito >= 0. && dinero_credito <= card_clone.balance {
            let _ = sqlx::query(&format!(
                r#"UPDATE deudas SET deuda = {} WHERE number = "{}""#,
                dinero_credito, card_clone.number
            )).execute(connection).await?;

            let _ = sqlx::query(&format!(
                r#"UPDATE atms SET money = {} WHERE name = "{}""#,
                atm_dinero, atm_clone.name
            )).execute(connection).await?;

            let _ = sqlx::query(&format!(
                r#"INSERT INTO withdrawals (amount, atm_name, card_number) VALUES ({},"{}","{}")"#,
                opcion as i32, atm_clone.name, card_clone.number
            )).execute(connection).await?;
            return Ok(format!("Retiro exitoso {}", dinero_credito));
        } else {
            return Ok(format!(
                "No tienes deuda o saldo mayor al que hay que pagar"
            ));
        }
    }
}
async fn trans(connection: &sqlx::Pool<MySql>, atm: UseState<Atm>, card: UseState<Option<Card>>, opcion: f64, cliend_card: String, ) -> Result<String> {
    if card.get().is_none() {
        return Err(anyhow!("Theres no card in the state"));
    }
    let mut card_clone = card.get().as_ref().unwrap().clone();

    let dinero = card_clone.balance - opcion;

    if !(dinero >= 0.) {
        println!("No tienes suficiente dinero");
    }
    if card_clone.r#type == "Credit" {
        println!("No puedes realizar esta operación con una tarjeta de crédito");
    }
    if cliend_card == card_clone.number {
        println!("No puedes transferirte a ti mismo");
    }
    let q = format!("select * from cards where number = {} ", cliend_card);
    let mut target = make_query::<Card>(q, connection).await?;
    let target = if target.len() == 1 {
        return Err(anyhow!("Se encontró mas de un dato"));
    } else {
        target.into_iter().next().unwrap()
    };
    let dinero = card_clone.balance - opcion;
    if target.r#type == "Debit" {
        let _ = sqlx::query(&format!(
            r#"UPDATE cards SET balance = {} WHERE number = {}"#,
            dinero,card_clone.number
        )).execute(connection).await?;
        let _ = sqlx::query(&format!(
            r#"UPDATE cards SET balance = {} WHERE number = {}"#,
            target.balance + opcion,
            target.number
        )).execute(connection).await?;
        let _ = sqlx::query(&format!(
            r#"INSERT INTO transfers (amount, sent_money, received_money) VALUES ({}, "{}","{}")"#,
            opcion, card_clone.number, target.number
        )).execute(connection).await?;
        card_clone.balance = dinero;
        println!("Transferencia exitosa {} ", card_clone.balance);
    } else {
        let deudas = make_query::<Deuda>(
            format!("Select * from deudas where number = {}", target.number),
            connection,
        ).await?;
        let mut deuda = if deudas.len() != 1 {
            return Err(anyhow!("No encontró la tarjeta"));
        } else {
            deudas.into_iter().next().unwrap()
        };
        let dinero_deuda = deuda.deuda - opcion;
        if dinero_deuda >= 0. && dinero_deuda <= target.balance {
            let _ = sqlx::query(&format!(
                r#"UPDATE cards SET balance = {} WHERE number = {}"#,
                dinero,card_clone.number
            ))
            .execute(connection)
            .await?;
            let _ = sqlx::query(&format!(
                r#"UPDATE deudas SET deuda = {} WHERE number = "{}""#,
                dinero_deuda,target.number
            ))
            .execute(connection)
            .await?;
            let _ = sqlx::query(
                &format!(r#"INSERT INTO transfers (amount, sent_money, received_money) VALUES ({}, "{}","{}")"#, opcion, card_clone.number, target.number),
            ).execute(connection).await?;
            deuda.deuda = dinero_deuda;
        } else {
            return Err(anyhow!("No tienes deuda o saldo mayor al que hay que pagar"));
        }
    }
    return Ok("Transferencia exitosa".to_string());
}
/*
async fn map(atm: &mut Atm, connection: &sqlx::Pool<MySql>) -> Result<()>  {


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
*/

fn Main(cx: Scope) -> Element {
    let connection: &UseState<MyPool>;
    let atm: &UseState<Atm>;
    let card: &UseState<Option<Card>> = use_state(&cx, || None);

    let future: &UseFuture<Result<_>> = use_future(&cx, (), |()| async move {
        let connection = MyPool {
            connection: MySqlPool::connect("mysql://daniel:1234@localhost/banco")
                .await
                .unwrap(),
        };
        let atm = get_atm(&connection.connection).await?;
        Ok((connection, atm))
    });
    match future.value() {
        Some(res) => {
            if let Ok((_con, _atm)) = res {
                atm = use_state(&cx, || _atm.clone());
                connection = use_state(&cx, || _con.clone());

                cx.render(rsx!{
                Router {
                    Route { to: "/", Login { card: card.clone(), connection: connection.clone() } }
                    Route { to: "/user", Vacio { card: card.clone() } }
                    Route{to: "/consult", Consulta { card: card.clone(), connection: connection.clone()} }
                    Route{to: "/withdrawal",
                        Retiro {
                            card: card.clone(),
                            atm: atm.clone(),
                            connection: connection.clone()
                        }
                    }

                }
            })
            } else {
                cx.render(rsx! {
                    [format_args!("Error al obtener el atm: {:?}", res.as_ref().unwrap_err())]
                })
            }
        }
        None => None,
    }
}

#[inline_props]
fn Transaction(cx: Scope, connection: UseState<MyPool>, atm: UseState<Atm>, card: UseState<Option<Card>>, opcion: f64,cliend_card: String ) -> Element {
    let future = use_future(&cx, (connection.get()), |(connection)| {
        dioxus::core::to_owned![atm, card, opcion, cliend_card];
        async move { trans(&connection.connection, atm, card, opcion, cliend_card).await }
    });
    let router = use_router(&cx);
    match future.value() {
        Some(data) => {
            router.push_route("/", None, None);
            cx.render(rsx! {
                p {}
            })
        }
        None => None,
    }
}

#[inline_props]
fn Retirador(cx: Scope, connection: UseState<MyPool>, atm: UseState<Atm>, card: UseState<Option<Card>>, opcion: f64, ) -> Element {
    let future = use_future(&cx, (connection.get()), |(connection)| {
        dioxus::core::to_owned![atm, card, opcion];
        async move { retira_dinero(&connection.connection, atm, card, opcion).await }
    });
    let router = use_router(&cx);
    match future.value() {
        Some(data) => {
            router.push_route("/", None, None);
            cx.render(rsx! {
                p {}
            })
        }
        None => None,
    }
}
#[inline_props]
fn Retiro(cx: Scope, card: UseState<Option<Card>>, atm: UseState<Atm>, connection: UseState<MyPool>, ) -> Element {
    let valor = use_state(&cx, || 100);
    let activador = use_state(&cx, || false);
    cx.render(rsx! {
        link {
                rel:"stylesheet",
                href:"/static/tailwindcss.css"
            },
        form {
            class: "flex flex-col",
            div {
                class: "mb-10",

                div {
                    "{valor}",
                    div {
                        activador.then(|| {
                            rsx!{
                                Retirador {
                                    connection: connection.clone(),
                                    atm: atm.clone(),
                                    card: card.clone(),
                                    opcion: *valor.get() as f64
                                }
                            }
                        })
                    }
                }
            }
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(100); activador.set(true)},
                "100 $"
            }},
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(200); activador.set(true)},
                "200 $"
            }},
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(500); activador.set(true)},
                "500 $"
            }},
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(1000); activador.set(true)},
                "1000 $"
            }},
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(2000); activador.set(true)},
                "2000 $"
            }},
            div {button {
                class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                prevent_default: "onclick",
                onclick: |_evt| {valor.set(4000); activador.set(true)},
                "4000 $"
            }}

        }
    })
}
#[inline_props]
fn Consulta(cx: Scope, card: UseState<Option<Card>>, connection: UseState<MyPool>) -> Element {
    let tarjeta = if let Some(c) = card.get() {
        c.number.to_owned()
    } else {
        return cx.render(rsx! {
            "Error, algo salio mal."
        });
    };

    let future = use_future(
        &cx,
        (&tarjeta, connection.get()),
        |(tarjeta, connection)| async move { consultar_saldo(&connection.connection, &tarjeta).await },
    );
    match future.value() {
        Some(value) => {
            cx.render(rsx! {
            link {
                rel:"stylesheet",
                href:"/static/tailwindcss.css"
            },

                p { "Saldo" }
                card.get().is_some().then(|| {
                    rsx!{
                        p {
                        //esto solo es para las de credito
                            if card.get().iter().next().unwrap().r#type == "Credit" {
                                if value.is_ok() {
                                    rsx!{
                                        [format_args!("Su saldo es  {:?} $ ", card.get().iter().next().unwrap().balance - value.as_ref().unwrap() )]
                                    }
                                } else {
                                    rsx!{
                                        "Algo salio mal al obtener la deuda"
                                    }
                                }

                            } else {
                                 //esto solo es para las de debito
                                rsx!{
                                    [format_args!("Su saldo es  {:?} $ ", card.get().iter().next().unwrap().balance)]
                                }

                            }


                        }
                        div {
                            class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                                Link {
                                    to: "/user",
                                    "Regresar"
                                     },
                             },
                    }
                })
            })
        }
        None => None,
    }
}

#[inline_props]
fn Vacio(cx: Scope, card: UseState<Option<Card>>) -> Element {
    cx.render(rsx! {
        link {
            rel:"stylesheet",
            href:"/static/tailwindcss.css"
        },
        div {
            p { "Menu!" }
            card.get().is_some().then(|| {
                rsx!{
                    p {
                        [format_args!("Card: {:?}", card.get().iter().next().unwrap().number)]
                    }
                }
            })
            div {
               class: "flex flex-col items-center",
                div {
                    class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                        Link {
                            to: "/consult",
                            "1. Consultar saldo"
                        },
                    },
                     div {
                    class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                    Link {

                        to: "/withdrawal",
                        "2. Retirar efectivo"
                    },
                        },
                     div {
                    class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                    Link {
                         to: "/deposit",
                        "3. Depositar efectivo"
                    },
                        },
                     div {
                    class: "rounded-md p-5 m-2 bg-black text-white font-bold",
                     Link {
                         to: "/transfer",
                        "4. Transferir efectivo"
                    },
                        }
            }
        }
    })
}

#[inline_props]
fn Logger(cx: Scope, card: UseState<Option<Card>>, user: String, password: String, connection: UseState<MyPool>, ) -> Element {
    let future = use_future(
        &cx,
        (user, password, connection.get()),
        |(user, password, connection)| async move {
            iniciar_sesion(&connection.connection, &user, &password).await
        },
    );
    let router = use_router(&cx);
    match future.value() {
        Some(Ok(cardf)) => {
            card.set(Some(cardf.clone()));
            router.push_route("/user", None, None);

            cx.render(rsx! { Link {
                to: "/user",
                active_class: "is-active",  // Only for this Link. Overwrites "custom-active" from Router.
                "User"
            } })
        }
        Some(Err(err)) => cx.render(rsx! { "Error" }),
        None => cx.render(rsx! { "Nada" }),
    }
}

#[inline_props]
fn Login(cx: Scope, card: UseState<Option<Card>>, connection: UseState<MyPool>) -> Element {
    let user: &UseState<String> = use_state(&cx, || "".to_owned());
    let password: &UseState<String> = use_state(&cx, || "".to_owned());
    let try_log: &UseState<bool> = use_state(&cx, || false);
    cx.render(rsx! {
        link {
            rel:"stylesheet",
            href:"/static/tailwindcss.css"
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
                        /* Email input */
                        div { class: "mb-6",
                            input { class: "form-control block w-full px-4 py-2 text-xl font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                                value: "{user}",
                                name: "user",
                                r#type: "text",
                                placeholder: "tarjeta",
                                onchange: |evt| {
                                    user.set(evt.value.clone());
                                }
                            }
                        }
                        /* Password input */
                        div { class: "mb-6",
                            input { class: "form-control block w-full px-4 py-2 text-xl font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                                value: "{password}",
                                    r#type: "password",
                                name: "password",
                                placeholder: "Password",
                                onchange: |evt| {
                                    password.set(evt.value.clone());
                                }
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
                                prevent_default: "onclick",
                                    onclick: move |evt| {
                                        try_log.set(true);
                                    },

                                    "Login"
                            },
                                {
                                    try_log.then(|| {
                                        rsx! {
                                            Logger {
                                                card: card.clone(),
                                                user: user.get().to_owned(),
                                                password: password.get().to_owned(),
                                                connection: connection.clone()
                                            }
                                        }
                                    })
                                },
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

struct Estado {
    connection: sqlx::Pool<MySql>,
    atm: Atm,
}

#[tokio::main]
async fn main() {
    let vdom = VirtualDom::new(Main);
    let content = dioxus::ssr::render_vdom_cfg(&vdom, |f| f.pre_render(true));
    dioxus::desktop::launch_cfg(Main, |c| c.with_prerendered(content));
}
