//! Ventana de edición. La lógica del lenguaje vive en la biblioteca.
use compilador::{
    archivos,
    errores::ErrorCompilacion,
    sintactico::{self, Resumen},
};
use eframe::egui;

#[derive(Default)]
pub struct Editor {
    codigo: String,
    ruta: String,
    resultado: Option<Result<Resumen, ErrorCompilacion>>,
    aviso_archivo: Option<String>,
    mostrar_ayuda: bool,
}

impl Editor {
    pub fn con_archivo(ruta: &str) -> Self {
        let mut editor = Self {
            ruta: ruta.into(),
            ..Self::default()
        };
        editor.abrir();
        editor
    }

    fn abrir(&mut self) {
        match archivos::cargar(self.ruta.trim()) {
            Ok(codigo) => {
                self.codigo = codigo;
                self.analizar();
                self.aviso_archivo = None;
            }
            Err(error) => self.aviso_archivo = Some(error),
        }
    }

    fn analizar(&mut self) {
        self.resultado = Some(sintactico::analizar(&self.codigo));
    }

    fn dibujar(&mut self, ui: &mut egui::Ui) {
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Enter)) {
            self.analizar();
        }
        egui::Panel::top("barra").show(ui, |ui| {
            ui.add_space(8.0);
            // ui.heading("Compilador");
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Analizar · Ctrl+Enter").clicked() {
                    self.analizar();
                }
                // if ui.button("Copiar código").clicked() { ui.ctx().copy_text(self.codigo.clone()); }
                // ui.checkbox(&mut self.mostrar_ayuda, "Reglas del lenguaje");
            });
            // ui.add_space(6.0);
            // ui.label("Ruta del archivo de texto (UTF-8):");
            // ui.horizontal(|ui| {
            //     ui.add(egui::TextEdit::singleline(&mut self.ruta)
            //         .desired_width((ui.available_width() - 125.0).max(100.0))
            //         .hint_text("ejemplos/anexo_a.txt")
            //         .id(egui::Id::unique("ruta_archivo")));
            //     if ui.add_enabled(!self.ruta.trim().is_empty(), egui::Button::new("Abrir archivo")).clicked() { self.abrir(); }
            // });
            // ui.small("Abrir reemplaza el texto del editor. Copia tu código antes de cargar otro archivo.");
            // if let Some(aviso) = &self.aviso_archivo { ui.label(format!("Archivo: {aviso}")); }
            // ui.add_space(8.0);
        });
        egui::Panel::bottom("resultados").resizable(true)
            .default_size(140.0).min_size(100.0).show(ui, |ui| {
                ui.strong("Resultado del análisis");
                egui::ScrollArea::vertical().id_salt("reporte").show(ui, |ui| {
                    match &self.resultado {
                        None => { ui.label("Sin analizar. Pulsa Analizar o Ctrl+Enter."); }
                        Some(Ok(resumen)) => {
                            ui.strong("El programa es sintácticamente correcto.");
                            ui.label(format!("{} tokens · {} variables declaradas", resumen.tokens, resumen.variables));
                        }
                        Some(Err(error)) => {
                            ui.strong(format!("{} · {} · Línea {}", error.codigo, error.codigo.tipo(), error.linea));
                            ui.label(&error.mensaje);
                            ui.small("Se muestra el primer error. Corrígelo y vuelve a analizar para continuar.");
                        }
                    }
                });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            if self.mostrar_ayuda {
                egui::CollapsingHeader::new("Sintaxis admitida").default_open(true).show(ui, |ui| {
                    ui.label("Inicio ... Fin · Declare a, b Entero; · a = (b + 2) * 3; · Lea(a); · Escriba(a);");
                    ui.label("Si a > b Entonces ... Sino ... FSi · MQ a < b Haga ... FMQ");
                    ui.label("Para i = 1 Hasta n Haga ... FPara · Las variables del Para deben ser Entero.");
                    ui.label("Tipos: Entero, Real, Cadena, Fecha y Lógico/Logico. No distingue mayúsculas.");
                    ui.small("Solo se valida el pseudocódigo. Lea y Escriba no solicitan datos ni imprimen valores.");
                    ui.small("No admite comentarios, cadenas literales ni signos unarios en el pseudocódigo.");
                });
                ui.separator();
            }
            ui.horizontal(|ui| {
                ui.strong("Pseudocódigo");
                ui.label(format!("{} líneas", self.codigo.split('\n').count()));
            });
            let altura = ui.available_height();
            egui::ScrollArea::both().id_salt("editor_scroll").show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let numeros = (1..=self.codigo.split('\n').count()).map(|n| n.to_string()).collect::<Vec<_>>().join("\n");
                    ui.add(egui::Label::new(egui::RichText::new(numeros).monospace().weak()).selectable(false));
                    let respuesta = ui.add_sized([ui.available_width().max(250.0), altura.max(80.0)],
                        egui::TextEdit::multiline(&mut self.codigo)
                            .id(egui::Id::unique("codigo"))
                            .code_editor().lock_focus(false)
                            .hint_text("Inicio\nDeclare a Entero;\na = 5;\nEscriba(a);\nFin")
                            .desired_width(f32::INFINITY));
                    if respuesta.changed() { self.resultado = None; }
                });
            });
        });
    }
}

impl eframe::App for Editor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.dibujar(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analizar_lectura_y_escritura_muestra_solo_el_reporte() {
        let ctx = egui::Context::default();
        let mut editor = Editor {
            codigo: "Inicio Declare a Entero; Lea(a); Escriba(a); Fin".into(),
            ..Default::default()
        };
        editor.analizar();
        assert!(editor.resultado.as_ref().unwrap().is_ok());
        for _ in 0..2 {
            let mut salida = cuadro(&ctx, &mut editor, vec![]);
            salida.textures_delta.clear();
        }
        let salida = cuadro(&ctx, &mut editor, vec![]);
        let textos: Vec<_> = salida
            .shapes
            .iter()
            .filter_map(|forma| match &forma.shape {
                egui::epaint::Shape::Text(texto) => Some(texto.galley.text().to_owned()),
                _ => None,
            })
            .collect();
        assert!(
            textos
                .iter()
                .any(|t| t.contains("sintácticamente correcto"))
        );
        for control in [
            "Consola de ejecución",
            "Ejecutar",
            "Detener",
            "Enviar dato",
            "Copiar salida",
        ] {
            assert!(
                !textos.iter().any(|t| t.contains(control)),
                "Control ajeno al analizador: {control}"
            );
        }
    }

    fn cuadro(
        ctx: &egui::Context,
        editor: &mut Editor,
        eventos: Vec<egui::Event>,
    ) -> egui::FullOutput {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1050.0, 800.0),
                )),
                events: eventos,
                ..Default::default()
            },
            |ui| editor.dibujar(ui),
        )
    }

    #[test]
    fn boton_teclado_y_edicion_usan_el_motor_y_descartan_resultados_obsoletos() {
        let ctx = egui::Context::default();
        let mut editor = Editor::default();
        // Los paneles solicitan un primer pase de medición antes de pintar.
        for _ in 0..2 {
            let mut salida = cuadro(&ctx, &mut editor, vec![]);
            salida.textures_delta.clear();
        }
        let mut salida = cuadro(&ctx, &mut editor, vec![]);
        salida.textures_delta.clear();
        let boton = salida
            .shapes
            .iter()
            .find_map(|forma| match &forma.shape {
                egui::epaint::Shape::Text(texto) if texto.galley.text().contains("Analizar ·") => {
                    Some(texto.pos + egui::vec2(10.0, 6.0))
                }
                _ => None,
            })
            .expect("botón visible");
        for pressed in [true, false] {
            let mut salida = cuadro(
                &ctx,
                &mut editor,
                vec![
                    egui::Event::PointerMoved(boton),
                    egui::Event::PointerButton {
                        pos: boton,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            salida.textures_delta.clear();
        }
        assert_eq!(
            editor
                .resultado
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap_err()
                .codigo
                .to_string(),
            "E03"
        );
        ctx.memory_mut(|m| m.request_focus(egui::Id::unique("codigo")));
        let mut salida = cuadro(
            &ctx,
            &mut editor,
            vec![egui::Event::Text("Inicio Fin".into())],
        );
        salida.textures_delta.clear();
        assert_eq!(editor.codigo, "Inicio Fin");
        assert!(editor.resultado.is_none());
        let mut salida = cuadro(
            &ctx,
            &mut editor,
            vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::CTRL,
            }],
        );
        salida.textures_delta.clear();
        assert!(editor.resultado.unwrap().is_ok());
    }

    #[test]
    fn el_editor_analiza_su_texto_y_conserva_el_codigo_si_falla_la_carga() {
        let mut editor = Editor::con_archivo("ejemplos/anexo_a.txt");
        editor.analizar();
        assert!(editor.resultado.as_ref().unwrap().is_ok());
        let anterior = editor.codigo.clone();
        editor.ruta = "archivo_que_no_existe.txt".into();
        editor.abrir();
        assert_eq!(editor.codigo, anterior);
        assert!(editor.aviso_archivo.is_some());
        editor.codigo = "Inicio\nz=4;\nFin".into();
        editor.analizar();
        assert_eq!(editor.resultado.unwrap().unwrap_err().linea, 2);
    }

    #[test]
    fn renderiza_con_y_sin_ayuda_en_ventanas_pequenas() {
        let ctx = egui::Context::default();
        let mut editor = Editor::con_archivo("ejemplos/anexo_a.txt");
        for ayuda in [false, true] {
            editor.mostrar_ayuda = ayuda;
            let entrada = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(720.0, 600.0),
                )),
                ..Default::default()
            };
            let mut salida = ctx.run_ui(entrada, |ui| editor.dibujar(ui));
            assert!(!salida.shapes.is_empty());
            salida.textures_delta.clear();
        }
    }
}
