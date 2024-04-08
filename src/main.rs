fn main() {
    let mut inp:String = String::new();

    match std::fs::read_to_string("inp2.txt") {
        Ok(contents) => {
            inp = contents;
        }
        Err(err) => {
            println!("Erro: {}", err);
        }
    }
    inp.retain(|c| !c.is_whitespace());
    let mut pts = 0;
    for x in (0..inp.len()-1).step_by(2) {
        println!("{}{}", &inp[x..x+1], &inp[x+1..x+2]);
        match &inp[x..x+1] {
            "A" => {
                match &inp[x+1..x+2] {
                    "X" => {println!("empatou"); pts = pts + 3 + 1},
                    "Y" => {println!("ganhou"); pts = pts + 6 + 2},
                    "Z" => {println!("perdeu"); pts = pts + 0 + 3},
                    _ => ()
                }
            },
            "B" => {
                match &inp[x+1..x+2] {
                    "X" => {println!("perdeu"); pts = pts + 0 + 1},
                    "Y" => {println!("empatou"); pts = pts + 3 + 2},
                    "Z" => {println!("ganhou"); pts = pts + 6 + 3},
                    _ => ()
                }
            },
            "C" => {
                match &inp[x+1..x+2] {
                    "X" => {println!("ganhou"); pts = pts + 6 + 1},
                    "Y" => {println!("perdeu"); pts = pts + 0 + 2},
                    "Z" => {println!("empatou"); pts = pts + 3 + 3},
                    _ => ()
                }
            },
            _ => ()
        }
    }
    println!("{}", pts);
}
