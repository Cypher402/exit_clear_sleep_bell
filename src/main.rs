use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ExitCode {
    let time = Duration::from_secs(6);
    let time_2 = Duration::from_secs(3);

    print!("\x07"); // Terminal bell.  Hex: 0x07  Name: BEL
    println!("\n#-----------------------------------------------------------------------------------------------------------------------------------#");
    println!("#                                                       exit_clear_sleep_bell                                                       #");
    println!("#-----------------------------------------------------------------------------------------------------------------------------------#");
    
    println!("\nWe're going to test 'ExitCode' in RUST.  Neat!");
    sleep(time_2);
    println!("\nGoing to sleep for 6 seconds!  Then clearing the screen.");
    sleep(time_2);
    println!("\nDid you see the error for 'Hell yeah motherfucker!'?");
    sleep(time);

    clearscreen::clear().expect("failed to clear screen"); //library 'cargo add clearscreen'
    println!("\n#----------------------------------------------------------------------------------------------------------------------------------------#");
    println!("#                                                                Goodbye!                                                                #");
    println!("#----------------------------------------------------------------------------------------------------------------------------------------#\n");
    return ExitCode::SUCCESS;

    println!("Hell yeah motherfucker!");  // This shouldn't print and give a compiler error.  Showing ExitCode is working.
}