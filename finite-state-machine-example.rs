use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum State{
    Locked,
    Unlocked,
}

#[derive(Debug)]
enum Event{
    Push, 
    Coin,
}

fn next_state(event: Event) -> State {

    // this machine has two events with the same outcome for each state
    
    match event {
        Event::Push => return State::Locked,
        Event::Coin => return State::Unlocked,
    };
}

// coin-operated turnstile
fn main(){
    let mut state = State::Locked;
    
    println!("State: {:?}", state);
    print!("> ");

    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines(){
        match line.unwrap().as_str() {
            "coin" => {state = next_state(Event::Coin);},
            "push" => {state = next_state(Event::Push);},
            "quit" => {return ();},
            e => {
                eprintln!("Error: Unknown event {}", e);
            }
        };
        

        println!("State: {:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
