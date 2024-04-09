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
        match &inp[x..x+1] {
            "A" => {
                match &inp[x+1..x+2] {
                    "X" => {println!("perdeu"); pts = pts + 0 + 3},
                    "Y" => {println!("empatou"); pts = pts + 3 + 1},
                    "Z" => {println!("ganhou"); pts = pts + 6 + 2},
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
                    "X" => {println!("perdeu"); pts = pts + 0 + 2},
                    "Y" => {println!("empatou"); pts = pts + 3 + 3},
                    "Z" => {println!("ganhou"); pts = pts + 6 + 1},
                    _ => ()
                }
            },
            _ => ()
        }
    }
    println!("{}", pts);
}

// x lose 
// y tie 
// z win