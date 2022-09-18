use std::io;
use std::fmt::Debug;

/// # S-Nim
/// It shows you the N and P positions of the following game.
/// You are given a pile of rocks and you can take from the given pile
/// only sizes recorded in S.
/// The first one unable to take stones away loses.
struct SNim{
    s_ : Vec<usize>, //The amount you can take from the only pile
    states_: Vec<usize>, // 1 if P position, 0 if N position
    number_states_: usize, //How many states one should calculate up to
}

impl SNim {
    ///Given the max number of stones, which you still want to check
    ///and the possible number of stones you can take away from the pile
    ///it generates an instance.
    fn new(n: usize, s: Vec<usize>) -> SNim {
        SNim{
            s_: s,
            states_: vec![1; n],
            number_states_: n,
        }
    }
    
    fn print_states(&self) -> (){
        for (pos, v) in self.states_.iter().enumerate() {
            if *v == 0 {
                println!["{}: N ", pos];
            }
            else {
                println!["{}: P ", pos];
            }
        }
    }

    fn calculate_states(&mut self) {
       /*let true_st = self.states_.iter_mut()
            .enumerate()
            .map(|(pos, v)| -> usize {
                for m in self.s_.iter() {
                    if (pos as isize)-(*m as isize) >= 0 {
                        if self.states_[pos-*m] == 1 {
                            0
                        }
                    }
                }
                1
            })
            .collect();
        
        self.states_ = true_st;
       */
        
        for pos in 0..self.states_.len() {
            for m in self.s_.iter() {
                if (pos as isize)-(*m as isize) >= 0 {
                    if self.states_[pos-*m] == 1 {
                        self.states_[pos] = 0;
                    }
                }
            }
        }
    }
}

///A simple line by line input reader for code shortening
fn read_number() -> usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)
      .expect("Failed to read line");
    input_line.trim().parse()
      .expect("Type in a number or a valid number")
}

fn main() {

    println!("Add meg, hogy max mennyi kavics esetén szeretnéd látni a pozíciókat:");
    let n = read_number();
    //let s = vec![2,4,7];
    
    println!("Add meg az S nagyságát:");
    let s_size = read_number();
    println!("Add meg soronként S elemeit:");
    
    
    let mut s = Vec::new();
    for i in 0..s_size {
        s.push(read_number());
    }
    
    let mut Test = SNim::new(n+1, s);
    Test.calculate_states();
    println!("Eredmény:");
    Test.print_states();
}
