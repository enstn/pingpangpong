mod game;
mod testagent;
// mod agent1;
use std::env;

use testagent::TestAgent;
// use agent1::Agent1;

fn main() {
    println!("Hello world! Ready to be pingpanged?!");
    
     // Check command line arguments for enabling/disabling the agent
     let args: Vec<String> = env::args().collect();
     let use_testagent = args.get(1).map_or(false, |arg| arg == "use-testagent");
     let use_agent1 = args.get(1).map_or(false, |arg| arg == "use-agent1");
 
     // Create the agent conditionally
     let agent = if use_testagent {
        println!("Game control is now transferred to: test-agent.");
        Some(TestAgent::new())
     } 
        // else if use_agent1 {
        // println!("Game control is now tranferred to: agent1.");
        // Some(Agent1::new()) }
        else {
        println!("You are now using: manual control.");
        None
     };

    game::main(agent);
}



