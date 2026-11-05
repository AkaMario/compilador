use compilador::lexico::{TipoToken, tokenizar};

#[test]
fn separa_elementos_sin_espacios_y_operadores_compuestos() {
    let tokens = tokenizar("a=3.14;\nSi a>=2 Entonces a=a+1; FSi").unwrap();
    let textos: Vec<_> = tokens.iter().map(|t| t.texto.as_str()).collect();
    assert_eq!(
        textos,
        [
            "a", "=", "3.14", ";", "Si", "a", ">=", "2", "Entonces", "a", "=", "a", "+", "1", ";",
            "FSi"
        ]
    );
    assert_eq!(tokens[2].tipo, TipoToken::Numero);
    assert_eq!(tokens[4].linea, 2);
}

#[test]
fn no_confunde_un_identificador_invalido_con_dos_tokens() {
    let tokens = tokenizar("1x _a año 8M 3. 1.2.3").unwrap();
    assert_eq!(tokens.len(), 6);
    assert!(tokens.iter().all(|t| t.tipo == TipoToken::Invalido));
    assert_eq!(tokenizar("a&b").unwrap_err().codigo.to_string(), "E01");
}

#[test]
fn admite_bom_y_saltos_windows() {
    let tokens = tokenizar("\u{feff}Inicio\r\n\tFin").unwrap();
    assert_eq!(tokens[0].texto, "Inicio");
    assert_eq!(tokens[1].linea, 2);
}
