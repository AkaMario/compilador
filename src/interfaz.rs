//! Ventana de edición. La lógica del lenguaje vive en la biblioteca.
use compilador::{
    archivos,
    ejecucion::{Ejecucion, Estado},
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
    ejecucion: Option<Ejecucion>,
    entrada: String,
    aviso_entrada: Option<String>,
    enfocar_entrada: bool,
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
                self.descartar_ejecucion();
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

    /// Ejecutar es una acción explícita: cargar o analizar nunca ejecuta código.
    pub fn ejecutar(&mut self) {
        self.descartar_ejecucion();
        self.analizar();
        if self.resultado.as_ref().is_some_and(Result::is_ok) {
            match Ejecucion::nueva(&self.codigo) {
                Ok(ejecucion) => self.ejecucion = Some(ejecucion),
                Err(error) => self.resultado = Some(Err(error)),
            }
        }
    }

    fn enviar_entrada(&mut self) {
        if let Some(ejecucion) = &mut self.ejecucion {
            match ejecucion.aportar_entrada(&self.entrada) {
                Ok(()) => {
                    self.entrada.clear();
                    self.aviso_entrada = None;
                }
                Err(error) => {
                    self.aviso_entrada = Some(error);
                    self.enfocar_entrada = true;
                }
            }
        }
    }

    fn descartar_ejecucion(&mut self) {
        self.ejecucion = None;
        self.entrada.clear();
        self.aviso_entrada = None;
        self.enfocar_entrada = false;
    }

    fn consola(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.strong("Consola de ejecución");
            if ui.button("Copiar salida").clicked() {
                let texto = self
                    .ejecucion
                    .as_ref()
                    .map_or_else(String::new, |e| e.salida.join("\n"));
                ui.ctx().copy_text(texto);
            }
        });
        let estado = self.ejecucion.as_ref().map(|e| e.estado.clone());
        match &estado {
            None => {
                ui.small("Pulsa Ejecutar para obtener la salida de Escriba.");
            }
            Some(Estado::EnCurso) => {
                ui.small("Ejecutando… Puedes detener el programa.");
            }
            Some(Estado::Finalizada) => {
                ui.small("Ejecución finalizada.");
            }
            Some(Estado::Detenida) => {
                ui.small("Ejecución detenida por el usuario.");
            }
            Some(Estado::Esperando { variable, linea }) => {
                ui.small(format!(
                    "Lea({variable}) · Línea {linea}: esperando un dato."
                ));
            }
            Some(Estado::Error(error)) => {
                ui.label(error.to_string());
            }
        }
        let reservada = if matches!(estado, Some(Estado::Esperando { .. })) {
            65.0
        } else {
            0.0
        };
        egui::ScrollArea::vertical()
            .id_salt("salida_ejecucion")
            .max_height((ui.available_height() - reservada).max(40.0))
            .stick_to_bottom(true)
            .show(ui, |ui| {
                if let Some(ejecucion) = &self.ejecucion {
                    for linea in &ejecucion.salida {
                        ui.monospace(linea);
                    }
                }
            });
        if matches!(estado, Some(Estado::Esperando { .. })) {
            ui.horizontal(|ui| {
                ui.label("Entrada:");
                let respuesta = ui.add(
                    egui::TextEdit::singleline(&mut self.entrada)
                        .id(egui::Id::unique("entrada_consola"))
                        .desired_width((ui.available_width() - 90.0).max(80.0)),
                );
                if self.enfocar_entrada {
                    respuesta.request_focus();
                    self.enfocar_entrada = false;
                }
                let enter = respuesta.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Enviar dato").clicked() || enter {
                    self.enviar_entrada();
                }
            });
            if let Some(aviso) = &self.aviso_entrada {
                ui.label(format!("Entrada inválida: {aviso}"));
            }
        }
    }

    fn dibujar(&mut self, ui: &mut egui::Ui) {
        if let Some(ejecucion) = &mut self.ejecucion {
            let esperaba = matches!(ejecucion.estado, Estado::Esperando { .. });
            ejecucion.avanzar(500);
            if !esperaba && matches!(ejecucion.estado, Estado::Esperando { .. }) {
                self.enfocar_entrada = true;
            }
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Enter)) {
            self.analizar();
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F5)) {
            self.ejecutar();
        }
        egui::Panel::top("barra").show(ui, |ui| {
            ui.add_space(8.0);
            // ui.heading("Compilador");
            // ui.label("Analiza tu pseudocódigo o ejecútalo y consulta su salida.");
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Analizar · Ctrl+Enter").clicked() { self.analizar(); }
                if ui.button("Ejecutar · F5").clicked() { self.ejecutar(); }
                let activa = self.ejecucion.as_ref().is_some_and(|e| matches!(e.estado, Estado::EnCurso | Estado::Esperando { .. }));
                if ui.add_enabled(activa, egui::Button::new("Detener")).clicked()
                    && let Some(ejecucion) = &mut self.ejecucion {
                    ejecucion.detener();
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
            .default_size(260.0).min_size(180.0).show(ui, |ui| {
                ui.strong("Resultado del análisis");
                egui::ScrollArea::vertical().id_salt("reporte").max_height(65.0).show(ui, |ui| {
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
                ui.separator();
                self.consola(ui);
            });
        egui::CentralPanel::default().show(ui, |ui| {
            if self.mostrar_ayuda {
                egui::CollapsingHeader::new("Sintaxis admitida").default_open(true).show(ui, |ui| {
                    ui.label("Inicio ... Fin · Declare a, b Entero; · a = (b + 2) * 3; · Lea(a); · Escriba(a);");
                    ui.label("Si a > b Entonces ... Sino ... FSi · MQ a < b Haga ... FMQ");
                    ui.label("Para i = 1 Hasta n Haga ... FPara · Las variables del Para deben ser Entero.");
                    ui.label("Tipos: Entero, Real, Cadena, Fecha y Lógico/Logico. No distingue mayúsculas.");
                    ui.small("Analizar valida; Ejecutar interpreta. Escriba(a) imprime su valor y Lea(a) solicita una entrada.");
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
                    if respuesta.changed() { self.resultado = None; self.descartar_ejecucion(); }
                });
            });
        });
        if self
            .ejecucion
            .as_ref()
            .is_some_and(|e| e.estado == Estado::EnCurso)
        {
            ui.ctx().request_repaint();
        }
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
    fn f5_y_entrada_por_enter_imprimen_el_dato_y_editar_detiene_la_ejecucion() {
        let ctx = egui::Context::default();
        let mut editor = Editor {
            codigo: "Inicio Declare a Entero; Lea(a); Escriba(a); Fin".into(),
            ..Default::default()
        };
        for _ in 0..2 {
            let mut s = cuadro(&ctx, &mut editor, vec![]);
            s.textures_delta.clear();
        }
        let mut s = cuadro(
            &ctx,
            &mut editor,
            vec![egui::Event::Key {
                key: egui::Key::F5,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        s.textures_delta.clear();
        let mut s = cuadro(&ctx, &mut editor, vec![]);
        s.textures_delta.clear();
        assert!(matches!(
            editor.ejecucion.as_ref().unwrap().estado,
            Estado::Esperando { .. }
        ));
        let mut s = cuadro(&ctx, &mut editor, vec![egui::Event::Text("10".into())]);
        s.textures_delta.clear();
        let mut s = cuadro(
            &ctx,
            &mut editor,
            vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        s.textures_delta.clear();
        let mut s = cuadro(&ctx, &mut editor, vec![]);
        s.textures_delta.clear();
        assert_eq!(editor.ejecucion.as_ref().unwrap().salida, ["10"]);
        ctx.memory_mut(|m| m.request_focus(egui::Id::unique("codigo")));
        let mut s = cuadro(&ctx, &mut editor, vec![egui::Event::Text(" ".into())]);
        s.textures_delta.clear();
        assert!(editor.ejecucion.is_none());
        assert!(editor.resultado.is_none());
    }

    #[test]
    fn codigo_invalido_no_ejecuta_ni_produce_salida() {
        let mut editor = Editor {
            codigo: "Inicio Escriba(z); Fin".into(),
            ..Default::default()
        };
        editor.ejecutar();
        assert!(editor.ejecucion.is_none());
        assert_eq!(
            editor.resultado.unwrap().unwrap_err().codigo.to_string(),
            "E14"
        );
    }

    #[test]
    fn ejecutar_muestra_diez_y_lea_recibe_datos_desde_la_consola() {
        let mut editor = Editor {
            codigo: "Inicio Declare a Entero; a=10; Escriba(a); Fin".into(),
            ..Default::default()
        };
        editor.ejecutar();
        assert!(
            editor.ejecucion.is_some(),
            "Ejecutar debe iniciar el intérprete"
        );
        editor.ejecucion.as_mut().unwrap().avanzar(100);
        assert_eq!(editor.ejecucion.as_ref().unwrap().salida, ["10"]);
        editor.codigo = "Inicio Declare a Entero; Lea(a); Escriba(a); Fin".into();
        editor.ejecutar();
        editor.ejecucion.as_mut().unwrap().avanzar(100);
        editor.entrada = "12".into();
        editor.enviar_entrada();
        editor.ejecucion.as_mut().unwrap().avanzar(100);
        assert_eq!(editor.ejecucion.as_ref().unwrap().salida, ["12"]);
        assert!(editor.entrada.is_empty());
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
