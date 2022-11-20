
use sqlx::mysql::MySqlPool;
use std::ptr::null;
// Importamos try_read que devuelve un error si falla y el tipo de error que devuelve
use text_io::{try_read, Error};
#[derive(Debug, sqlx::FromRow)]
struct Instructor {

    ID:String,
    #[sqlx(default)]
    name:String,
    #[sqlx(default)]
    dept_name:String,
    #[sqlx(default)]
    salary:f64
}

#[derive(Debug, sqlx::FromRow)]
struct User{
    nip:i8,
    tarjeta:String,
    #[sqlx(default)]
    name:String,
    #[sqlx(default)]
    id_cajero:i8,
    #[sqlx(default)]
    dinero:i64,
}

impl User {
    fn new (nip:i8, tarjeta:String, id_cajero:i8) -> User {
        return User {nip, name: "".to_string(), tarjeta, id_cajero, dinero:0}
    }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = MySqlPool::connect("mysql://root:1234@localhost/university").await?;

    let mut campos = String::new();
    let _ = std::io::stdin().read_line(&mut campos)?;
    /*
        let result = sqlx::query_as::<_, Instructor>(&format!("select ID, {campos} from instructor"))
            .fetch_all(&connection)
            .await?;
    */
    campos = campos.trim().to_string();

    println!("Ingrese el número de tarjeta: ");
    let mut tarjeta:String = String::new();
    let _ = std::io::stdin().read_line(&mut tarjeta)?;
 




    let query = format!(r#"select ID from instructor where dept_name = "{campos}" "# );
    println!("{:?}",query);
    let result2 = sqlx::query_as::<_,Instructor>(&query)
        .fetch_all(&connection)
        .await?;


    //println!("Result: {:#?}", result);
    println!("Result2: {:#?}", result2);
    println!("tamaño {}", result2.len());
    println!("{}", tarjeta);








    Ok(())
}