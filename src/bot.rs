use crate::expre::eval;
use crate::jugada::auto;
use crate::pantalla::capture;
use dotenv::dotenv;
use dptree::HandlerResult;
use enigo::agent::Token;
use std::{
    clone, collections::HashMap, env, fs, hash::Hash, path::Path, sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    }, vec
};
use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::*,
    types::{InputFile, InputMediaPhoto, Recipient},
};

type HHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub struct sala {
    pub nombre: String,

    pub numeros: Vec<String>,

    pub cantidad: String,

    pub tipo: String,

    pub id:String,

    pub grupo:String
}
type Ar = Arc<Mutex<Sender<sala>>>;
pub async fn telegram() {
    //dotenv().ok();

    let (tx, mut rx) = mpsc::channel();

    let tx: Ar = Arc::new(Mutex::new(tx));

   // let token = env::var("TOKEN").unwrap();

    let mut bot = Bot::new("7564267258:AAHG2Vf22EU3sxfm75lFVowG_13GVuSzW_Y");
    let mut bot1 = bot.clone();
    tokio::spawn(async move {
        while let Ok(task) = rx.recv() {
            match auto(task.numeros, task.cantidad, task.tipo).await {
                "Agotado" => {

                    bot1.send_message(task.id,"Animalitos Agotados o Reducidos\npruebe con una cantidad menor de dinero").await;
                      
                },
                "numero"=>{
                    bot1.send_message(task.id,"Los numeros solicitados estan agotados").await;
                },
                _ => {

                    let grup=grupo(task.grupo);
                    let file = Path::new("2776250164-2.png");
                    bot1.send_photo(grup.clone(), InputFile::file(file))
                        .await;
                    bot1.send_message(grup,task.nombre).await;

                    bot1.send_photo(task.id.clone(), InputFile::file(file))
                        .await;
                    bot1.send_message(task.id,"Jugada exitosa").await;


                    
                }
            }

            fs::remove_file("2776250164-2.png");
        }
    });

    Dispatcher::builder(bot, Update::filter_message().endpoint(recep))
        .dependencies(dptree::deps![tx])
        .build()
        .dispatch()
        .await;
}

async fn recep(bot: Bot, msg: Message, tx: Ar) -> HHandlerResult {
    let start = format!("Bienvenido\nformato nombre (animalito1,animalito2,animalito3) monto l(lotto) o g(granjita)");

    if let Some(mensage) = msg.text() {
        match mensage {
            "/start" => {
                bot.send_message(msg.chat.id, start).await;
            }
            _ => match eval(mensage) {
                Ok((nombre,grupo, plis, cantida, tipo)) => {
                    async move {
                        let cola = Arc::clone(&tx);
                        let tx = cola.lock().unwrap();
                        tx.send(sala {
                            nombre: nombre,
                            numeros: plis,
                            cantidad: cantida,
                            tipo: tipo,
                            id:msg.chat.id.to_string(),
                            grupo:grupo
                        })
                    }
                    .await;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, e).await;
                }
            },
        }
    }

    Ok(())
}

fn grupo(inicial:String)->String{

    let mut grup=HashMap::new();

    grup.insert("ji".to_string(), "-1002316592138");
    grup.insert("va".to_string(), "-1002263773673");
    grup.insert("je".to_string(), "-1002336345111");
    grup.insert("sa".to_string(), "-1002482233327");
    grup.insert("da".to_string(), "-1002262432104");

    grup.get(&inicial).unwrap().to_string()

}