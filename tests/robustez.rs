use compilador::sintactico::analizar;

#[test]
fn distingue_expresion_invalida_de_punto_y_coma_ausente() {
    for (cuerpo, esperado) in [
        ("a=1=2;", "E13"),
        ("a=1 a=2;", "E05"),
        ("Lea(_a);", "E02"),
        ("a=ñ;", "E02"),
        ("Si 1<2 3 Entonces FSi", "E08"),
        ("Si 1<2 a Entonces FSi", "E08"),
        ("Si 1<2 hga FSi", "E09"),
    ] {
        let entrada = format!("Inicio Declare a Entero; {cuerpo} Fin");
        let error = analizar(&entrada).unwrap_err();
        assert_eq!(error.codigo.to_string(), esperado, "{entrada}: {error}");
    }
}

#[test]
fn acepta_expresiones_de_la_gramatica_y_rechaza_operadores_sin_operando() {
    // Combinación de producciones de la gramática, sin reutilizar el parser.
    for izquierdo in ["a", "123", "3.14", "(a+1)", "((a))"] {
        for derecho in ["a", "0", "2.5", "(3*a)"] {
            for operador in ["+", "-", "*", "/"] {
                let entrada =
                    format!("Inicio Declare a Real; a=({izquierdo}{operador}{derecho}); Fin");
                assert!(analizar(&entrada).is_ok(), "{entrada}");
                let invalida = format!("Inicio Declare a Real; a=({izquierdo}{operador}); Fin");
                assert!(analizar(&invalida).is_err(), "{invalida}");
            }
        }
    }
}

#[test]
fn anidamiento_profundo_no_depende_de_la_pila_de_llamadas() {
    let entrada = format!(
        "Inicio Declare a Entero; a={}1{}; {}{} Fin",
        "(".repeat(2000),
        ")".repeat(2000),
        "Si 1<2 Entonces MQ 1<2 Haga ".repeat(1000),
        "FMQ FSi ".repeat(1000)
    );
    assert!(analizar(&entrada).is_ok());
}

#[test]
fn entradas_arbitrarias_no_provocan_panico() {
    let piezas = [
        "Inicio", "Fin", "Si", "Entonces", "Sino", "FSi", "Para", "Hasta", "Haga", "MQ", "FMQ",
        "FPara", "Declare", "Entero", "a", "=", "(", ")", ";", ",", "1x", "1", "+", "&", "ñ",
    ];
    let mut semilla: usize = 137;
    for _ in 0..2000 {
        let mut entrada = String::from("Inicio ");
        for _ in 0..24 {
            semilla = semilla.wrapping_mul(1664525).wrapping_add(1013904223);
            entrada.push_str(piezas[semilla % piezas.len()]);
            entrada.push(' ');
        }
        let _ = analizar(&entrada);
    }
}
