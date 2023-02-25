use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let args_length = args.len();
    if args_length ==3 {

       let output = format!(
           "{} {}{}\n{} {}{} {}\n{}",
           "interface", &args[1],"{","function ",&args[2],"()","external;","}"
        );
        
        println!("{}",output); // Print the header to console.
        
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        
        ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
    }
    if args_length ==4 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
     if args_length ==5 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "function",&args[4],"()","external;",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
     if args_length ==6 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "function",&args[4],"()","external;",
            "function",&args[5],"()","external;",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
     if args_length ==7 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "function",&args[4],"()","external;",
            "function",&args[5],"()","external;",
            "function",&args[6],"()","external",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
     if args_length ==8 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "function",&args[4],"()","external;",
            "function",&args[5],"()","external;",
            "function",&args[6],"()","external",
            "function",&args[7],"()","external;",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
     if args_length ==9 {

        let output = format!(
            "{} {}{}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{} {}{} {}\n{}",
            "interface", &args[1],"{","function ",&args[2],"()","external;",
            "function",&args[3],"()","external;",
            "function",&args[4],"()","external;",
            "function",&args[5],"()","external;",
            "function",&args[6],"()","external",
            "function",&args[7],"()","external;",
            "function",&args[8],"()","external;",
            "}"
         );
         
         println!("{}",output); // Print the header to console.
         
         let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
         
         ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
     }
}