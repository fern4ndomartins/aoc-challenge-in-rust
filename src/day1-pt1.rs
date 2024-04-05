fn main() {

    let mut inp:String = String::new();

    match std::fs::read_to_string("inp1.txt") {
        Ok(contents) => {
            inp = contents;
        }
        Err(err) => {
            println!("Erro: {}", err);
        }
    }
    inp.retain(|c| c != ' ' && c != '\r');

    let mut v: Vec<i32> = Vec::new();
    inp.retain(|c| c != ' ');
    for chunk in inp.split("\n\n") {
        let mut soma = 0;
        for line in chunk.split("\n") {
            soma = soma + line.parse::<i32>().unwrap_or(0);
        }
        v.push(soma);
    }
    println!("{:?}", v);
    println!("{:?}", v.iter().max()); // <- resposta !!!
}


