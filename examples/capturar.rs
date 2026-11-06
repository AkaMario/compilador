//! Captura reproducible de la ventana real para el informe.
//! cargo run --example capturar -- ARCHIVO SALIDA.ppm [ANCHO ALTO]
#[path = "../src/interfaz.rs"]
mod interfaz;
use eframe::{App, egui};
use std::io::Write;

struct Captura {
    editor: interfaz::Editor,
    destino: String,
    frames: usize,
}

impl App for Captura {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.editor.ui(ui, frame);
        self.frames += 1;
        if self.frames == 4 {
            let destino = self.destino.clone();
            let ctx = ui.ctx().clone();
            ui.ctx().request_screenshot(move |imagen| {
                let mut archivo = std::fs::File::create(destino).expect("crear captura");
                write!(archivo, "P6\n{} {}\n255\n", imagen.size[0], imagen.size[1]).unwrap();
                for pixel in &imagen.pixels {
                    archivo.write_all(&pixel.to_array()[..3]).unwrap();
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            });
        }
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(100));
    }
}

fn main() -> eframe::Result {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Uso: capturar ARCHIVO SALIDA.ppm [ANCHO ALTO]");
        return Ok(());
    }
    let ancho = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1050.0);
    let alto = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(800.0);
    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([ancho, alto]),
        ..Default::default()
    };
    eframe::run_native(
        "Simulador - verificación visual",
        opciones,
        Box::new(move |_| {
            let editor = interfaz::Editor::con_archivo(&args[0]);
            Ok(Box::new(Captura {
                editor,
                destino: args[1].clone(),
                frames: 0,
            }))
        }),
    )
}
