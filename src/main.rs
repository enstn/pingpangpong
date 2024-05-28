mod game;
mod testagent;
use std::env;

use testagent::TestAgent;

fn main() {
    println!("Hello world! Ready to be pingpanged?!");
    
     // Check command line arguments or a flag to enable/disable the agent
     let args: Vec<String> = env::args().collect();
     let use_agent = args.get(1).map_or(false, |arg| arg == "use-agent");
 
     // Create the agent conditionally
     let agent = if use_agent {
         println!("Using agent.");
         Some(TestAgent::new())
     } else {
         println!("Using manual control.");
         None
     };

    game::main(agent);
}



