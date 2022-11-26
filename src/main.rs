use std::io::Write;
use sqlx::mysql::MySqlPool;
use std::ptr::null;
use sqlx::MySql;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read};
use anyhow::{Result, Error, anyhow};

#[derive(Debug, sqlx::FromRow)]
struct User {
    nom_card: String,
    F_name: String,
    M_name: String,
    #[sqlx(default)]
    C_id: i32,

}

#[derive(Default, Clone)]
#[derive(Debug, sqlx::FromRow)]
struct card{
    Card_No:String,
    Card_nip:i32,
    #[sqlx(default)]
    Card_Balance:f64,
    #[sqlx(default)]
    Card_Type:String,
    #[sqlx(default)]
    Card_ExpiryDate:String,
    #[sqlx(default)]
    Card_CVV:String,
    #[sqlx(default)]
    Card_status:bool,
    #[sqlx(default)]
    Card_Bankname:String,
}

#[derive(Debug, sqlx::FromRow)]
struct atm_machine{
    ATM_id:i32,
    ATM_name:String,
    ATM_add:String,
    ATM_bankname:String,
    ATM_money:f64,
}

impl card {
    fn new (Card_No:String, Card_nip:i32) -> card {
        card {
            Card_No,
            Card_nip,
            ..Default::default()
        }
    }

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

async fn get_atm(connection: &sqlx::Pool<MySql>) -> Result<atm_machine> {
    if let Some(addr) = mac_address::get_mac_address()? {
        let q = format!("insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname, ATM_money)
values (1,'{addr}', 'Oso 81, Col del Valle Centro, Benito Juárez, 03100 Ciudad de México, CDMX', 'Santander', 200000.0)");
        let crear = sqlx::query(&q)
            .fetch_all(connection)
            .await?;
        let q = format!("select * from atm_machine where atm_name = '{addr}'");
        let result = make_query::<atm_machine>(q, connection).await?;

        if result.len() == 1 {
            Ok(result.into_iter().next().unwrap())
        } else {
            Err(anyhow!("Se encontraron varios ATMs"))
        }

    } else {
        Err(anyhow!("No se pudo obtener la MAC"))
    }
}

async fn iniciar_sesion(connection: &sqlx::Pool<MySql>) -> Result<card> {
    let mut i = 0;

    loop {
        let tarjeta = input("Ingrese el número de tarjeta: ")?;
        let nip = input("Ingrese el nip: ")?;

        let query = format!(r#"select Card_No, Card_nip from card where Card_No = "{tarjeta}" and Card_nip = "{nip}" "# );
        if let cartas =  make_query::<card>(query, connection).await? {
            if cartas.len() == 1 {
                break Ok(cartas.into_iter().next().unwrap());
            } else {
                break Err(anyhow!("Se encontró mas de un dato"));
            }
        } else {
            i+=1;
            if i == 3 {
                break Err(anyhow!("Se intentó ingresar demasiadas veces"));
            }
        }
    }
}

async fn app(connection: &sqlx::Pool<MySql>) -> Result<()>  {



    let result2 = iniciar_sesion(connection).await?;
    println!("Result2: {:#?}", result2);

    Ok(())
}

#[tokio::main]
async fn main() {
    let connection = MySqlPool::connect("mysql://root:1234@localhost/Banco").await.unwrap();
    let atm = get_atm(&connection).await.unwrap();

    println!("{atm:?}");

    loop {
        match app(&connection).await {
            Err(error) => {
                println!("Error: {}", error);
                continue
            },
            Ok(_) => {}
        }
    }
}


/*
    let result = sqlx::query_as::<_, Instructor>(&format!("select ID, {campos} from instructor"))
        .fetch_all(&connection)
        .await?;
*/