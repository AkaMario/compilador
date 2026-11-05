use std::process::Command;

#[test]
fn la_terminal_distingue_exito_error_de_programa_y_error_de_archivo() {
    for (ruta, estado, texto) in [
        ("ejemplos/anexo_a.txt", 0, "sintácticamente correcto"),
        ("ejemplos/error_sintactico.txt", 1, "E09"),
        ("ejemplos/error_semantico.txt", 1, "E14"),
        ("ejemplos/no_existe.txt", 2, "No se pudo abrir"),
    ] {
        let salida = Command::new(env!("CARGO_BIN_EXE_compilador"))
            .args(["--analizar", ruta])
            .output()
            .unwrap();
        assert_eq!(salida.status.code(), Some(estado), "{ruta}");
        let mensaje = format!(
            "{}{}",
            String::from_utf8_lossy(&salida.stdout),
            String::from_utf8_lossy(&salida.stderr)
        );
        assert!(mensaje.contains(texto), "{mensaje}");
    }
}

#[test]
fn argumentos_invalidos_no_intentan_abrir_la_ventana() {
    for argumentos in [vec!["--analizar"], vec!["--abrir"], vec!["--desconocido"]] {
        let salida = Command::new(env!("CARGO_BIN_EXE_compilador"))
            .args(argumentos)
            .output()
            .unwrap();
        assert_eq!(salida.status.code(), Some(2));
    }
}
