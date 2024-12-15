use crate::pantalla::capture;
use crate::texto::lector;
use enigo::{Button, Coordinate::Abs, Direction::Click, Enigo, Key, Keyboard, Mouse};
use std::{collections::HashMap, thread::sleep, time::Duration};
use teloxide::Bot;

pub async fn auto(numer: Vec<String>, cantidad: String, tipo: String) -> &'static str {
    let mut animalito = HashMap::new();
    animalito.insert("0".to_string(), (380, 180));
    animalito.insert("00".to_string(), (482, 160));
    animalito.insert("1".to_string(), (334, 216));
    animalito.insert("2".to_string(), (416, 222));
    animalito.insert("3".to_string(), (489, 224));
    animalito.insert("4".to_string(), (338, 258));
    animalito.insert("5".to_string(), (405, 264));
    animalito.insert("6".to_string(), (498, 269));
    animalito.insert("7".to_string(), (336, 303));
    animalito.insert("8".to_string(), (402, 307));
    animalito.insert("9".to_string(), (492, 305));
    animalito.insert("10".to_string(), (338, 338));
    animalito.insert("11".to_string(), (424, 343));
    animalito.insert("12".to_string(), (506, 342));
    animalito.insert("13".to_string(), (323, 379));
    animalito.insert("14".to_string(), (408, 386));
    animalito.insert("15".to_string(), (500, 388));
    animalito.insert("16".to_string(), (332, 426));
    animalito.insert("17".to_string(), (414, 429));
    animalito.insert("18".to_string(), (497, 429));
    animalito.insert("19".to_string(), (316, 471));
    animalito.insert("20".to_string(), (421, 466));
    animalito.insert("21".to_string(), (478, 467));
    animalito.insert("22".to_string(), (322, 508));
    animalito.insert("23".to_string(), (406, 510));
    animalito.insert("24".to_string(), (494, 512));
    animalito.insert("25".to_string(), (338, 550));
    animalito.insert("26".to_string(), (398, 549));
    animalito.insert("27".to_string(), (494, 548));
    animalito.insert("28".to_string(), (346, 588));
    animalito.insert("29".to_string(), (407, 588));
    animalito.insert("30".to_string(), (473, 590));
    animalito.insert("31".to_string(), (326, 629));
    animalito.insert("32".to_string(), (410, 631));
    animalito.insert("33".to_string(), (478, 631));
    animalito.insert("34".to_string(), (325, 673));
    animalito.insert("35".to_string(), (410, 670));
    animalito.insert("36".to_string(), (491, 673));

    let mut enigo = Enigo::new(&Default::default()).unwrap();

    enigo.move_mouse(73, 156, Abs);
    enigo.button(Button::Left, Click);

    let mut ani = |nu: String, enigo: &mut Enigo| {
        let (x, y) = animalito.get(&nu).unwrap();

        enigo.move_mouse(*x, *y, Abs).unwrap();
        enigo.button(Button::Left, Click);

        sleep(Duration::from_secs(2));
    };

    numer.into_iter().for_each(|x| ani(x, &mut enigo));

    enigo.move_mouse(850, 354, Abs);
    enigo.button(Button::Left, Click);
    enigo.text(cantidad.as_str());

    sleep(Duration::from_secs(2));

    if tipo == "g" {
        //la granjita
        enigo.move_mouse(554, 438, Abs);
        enigo.button(Button::Left, Click);
    } else {
        //lotto
        enigo.move_mouse(554, 458, Abs);
        enigo.button(Button::Left, Click);
    }

    sleep(Duration::from_secs(2));
    //agregar
    enigo.move_mouse(1139, 354, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(2));
    //imprimir tikect
    enigo.move_mouse(1218, 356, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(2));

    capture().await;
    println!("llego aqui");
    match lector().await.unwrap() {
        "Agotado" => {
            //modificar
            modificar(&mut enigo).await;

            return "Agotado";
        },
        "numero" => {
            numero(&mut enigo).await;
            return "numero";
        },
        "VAR"=>{
            imprimir(&mut enigo).await;
          return "VAR"
        },
        _ => {
            //imprimir

            imprimir(&mut enigo);
            return "";
        }
    }
}

async fn imprimir(enigo: &mut Enigo) {
    enigo.move_mouse(661, 667, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(3));
    //cerrar pestana
    enigo.move_mouse(1233, 125, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(2));
}

async fn modificar(enigo: &mut Enigo) {
    enigo.move_mouse(772, 684, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(3));
    //cerrar pestana
    enigo.move_mouse(1233, 125, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(2));
}

async fn numero(enigo: &mut Enigo) {
    enigo.move_mouse(835, 474, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(3));
    //cerrar pestana
    enigo.move_mouse(1233, 125, Abs);
    enigo.button(Button::Left, Click);

    sleep(Duration::from_secs(2));
}
