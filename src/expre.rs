use regex::Regex;
pub fn eval(palabra: &str) -> Result<(String, String, Vec<String>, String, String), String> {
    let re = Regex::new(r"(\w+)\s\w{2}\s\((\d{1,2},\d{1,2},\d{1,2})\)\s(\d+)\s([lg])").unwrap();

    let iniciales = ["ji", "va", "je", "sa", "da"];

    if let Some(pala) = re.captures(palabra) {
        println!("{:?}", pala);
        let plis: Vec<String> = pala[3]
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split(",")
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let mut num = plis
            .clone()
            .into_iter()
            .map(|x| x.parse::<i32>().expect(""));

        if !num.all(|x| x >= 0 && x < 37) {
            return Err("Eror: Verifique los numeros de los animalitos".to_string());
        }

        if iniciales.contains(&&pala[2]) {
            return Err("Error: grupo no reconocido".to_string());
        }

        if plis[0] != plis[1] && plis[0] != plis[2] && plis[1] != plis[2] {
            return Ok((
                pala[1].to_string(),
                pala[2].to_string(),
                plis,
                pala[4].to_string(),
                pala[5].to_string(),
            ));
        } else {
            return Err("Error: Los animaitos tiene que ser diferentes".to_string());
        }
    }

    Err("Error: Formato invalido".to_string())
}
