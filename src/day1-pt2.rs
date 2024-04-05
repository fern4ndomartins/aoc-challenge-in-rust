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
    let mut v: Vec<i32> = Vec::new();
    inp.retain(|c| c != ' ' && c != '\r');

    for chunk in inp.split("\n\n") {
        let mut soma = 0;
        for line in chunk.split("\n") {
            soma = soma + line.parse::<i32>().unwrap_or(0);
        }
        v.push(soma);
    }

    v.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;

    for x in v[0..3].iter() {
        ans = ans + x;
    }
    println!("{}", ans); // resposta
}


