use kifi::{commands, output::{DebugOutput, Output}};
use std::path::PathBuf;


// macro_rules! command {
//     ($($function:ident($($arg:ident : $arg_ty:ty),*)),*) => {
//         use kifi::output::{DebugOutput, Output};
//         use kifi::commands;
//         use std::path::PathBuf;

//         $(
//             #[tauri::command]
//             pub fn $function($($arg : $arg_ty),*) -> String {
//                 let mut output = DebugOutput::new();
//                 $(
//                     let $arg = match stringify!($arg) {
//                         "path" => Some(PathBuf::from($arg)),
//                         _ => $arg,
//                     };
//                 )*
//                 match commands::$function(&mut output, $($arg),*) {
//                     Ok(()) => {
//                         match output.print() {
//                             Some(output_vector) => output_vector.join("\n"),
//                             None => String::new(),
//                         }
//                     }
//                     Err(e) => {
//                         format!("Error: {:#?}", e)
//                     }
//                 }
//             }
//         )*
//     }
// }

// command!(
//     initialise(path: String),
//     track(file_name: String, forced: bool, path: String),
//     preview(path: String),
//     snapshot(path: String),
//     log(path: String),
//     revert(name: String, path: String),
//     register(name: String, email: String)
// );

#[tauri::command]
pub fn initialise(path: String) -> Result<String, String> {
    let mut output = DebugOutput::new();
    match commands::initialise(&mut output, Some(PathBuf::from(path))) {
        Ok(()) => {
            Ok(match output.print() {
                Some(output_vector) => output_vector.join("\n"),
                None => String::new(),
            })
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            Err(format!("Error: {:#?}", output_string))
        }
    }
}

#[tauri::command]
pub fn meta(path: String) -> Result<String, String> {
    let mut output = DebugOutput::new();
    match commands::meta(&mut output, Some(PathBuf::from(path))) {
        Ok(()) => {
            Ok(match output.print() {
                Some(output_vector) => output_vector.join("\n"),
                None => String::new(),
            })
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            Err(format!("Error: {:#?}", output_string))
        }
    }
}

#[tauri::command]
pub fn track(file_name: String, forced: bool, path: String) -> Result<String, String> {
    let mut output = DebugOutput::new();
    match commands::track(&file_name, &forced, &mut output, Some(PathBuf::from(path))) {
        Ok(()) => {
            match output.print() {
                Some(output_vector) => Ok(output_vector.join("\n")),
                None => Ok(String::new()),
            }
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            Err(format!("Error: {:#?}", output_string))
        }
    }
}

#[tauri::command]
pub fn preview(path: String) -> Result<String, String> {
    let mut output = DebugOutput::new();
    match commands::preview(&mut output, Some(PathBuf::from(path))) {
        Ok(()) => match output.print() {
            Some(output_vector) => Ok(output_vector.join("\n")),
            None => Ok(String::new()),
        },
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            Err(format!("Error: {:#?}", output_string))
        }
    }
}

#[tauri::command]
pub fn snapshot(path: String) -> String {
    match commands::snapshot(Some(PathBuf::from(path))) {
        Ok(()) => {
            String::from("Snapshot created")
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            format!("Error: {:#?}", output_string)
        }
    }
}

#[tauri::command]
pub fn log(path: String) -> String {
    let mut output = DebugOutput::new();
    match commands::log(&mut output, Some(PathBuf::from(&path))) {
        Ok(()) => {
            match output.print() {
                Some(output_vector) => output_vector.join("\n"),
                None => String::new(),
            }
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            format!("Error: {:#?}", output_string)
        }
    }
}

#[tauri::command]
pub fn revert(name: String, path: String) -> String {
    let mut output = DebugOutput::new();
    match commands::revert(&mut output, name, Some(PathBuf::from(path))) {
        Ok(()) => {
            String::from("Reverted to last snapshot")
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            format!("Error: {:#?}", output_string)
        }
    }
}

#[tauri::command]
pub fn register(name: String, email: String) -> String {
    match commands::register(&name, &email) {
        Ok(()) => {
            String::from("Registered")
        }
        Err(e) => {
            let mut output = DebugOutput::new();
            e.handle(&mut output);
            let output_string = match output.print() {
                Some(lines) => lines.join("\n"),
                None => String::new(),
            };
            format!("Error: {:#?}", output_string)
        }
    }
}