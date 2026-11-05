mod interfaz;

use compilador::{archivos, sintactico};
use interfaz::Editor;
use std::process::ExitCode;

fn main() -> ExitCode {
    let argumentos: Vec<_> = std::env::args().skip(1).collect();
    if argumentos
        .first()
        .is_some_and(|a| a == "--ayuda" || a == "--help")
    {
        println!(
            "Simulador de compilador\n\nSin argumentos: abre el editor.\n--abrir RUTA: abre el editor con un archivo UTF-8.\n--analizar RUTA: analiza sin abrir la ventana.\n\nSalida: 0 correcto; 1 error del pseudocódigo; 2 error de archivo, argumentos o ventana."
        );
        return ExitCode::SUCCESS;
    }
    if argumentos.first().is_some_and(|a| a == "--analizar") && argumentos.len() == 2 {
        return match archivos::cargar(&argumentos[1]) {
            Ok(codigo) => match sintactico::analizar(&codigo) {
                Ok(_) => {
                    println!("El programa es sintácticamente correcto.");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(1)
                }
            },
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(2)
            }
        };
    }
    let ruta = match argumentos.as_slice() {
        [] => None,
        [opcion, ruta] if opcion == "--abrir" => Some(ruta.clone()),
        _ => {
            eprintln!("Argumentos no válidos. Usa --ayuda para ver las opciones.");
            return ExitCode::from(2);
        }
    };
    let opciones = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1050.0, 800.0])
            .with_min_inner_size([720.0, 600.0]),
        ..Default::default()
    };
    match eframe::run_native(
        "Compilador",
        opciones,
        Box::new(move |_cc| {
            Ok(Box::new(
                ruta.as_deref()
                    .map_or_else(Editor::default, Editor::con_archivo),
            ))
        }),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("No se pudo iniciar la ventana: {error}");
            ExitCode::from(2)
        }
    }
}
