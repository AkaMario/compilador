//! Lectura de elementos sin depender de los saltos de línea como separadores.
use crate::errores::{CodigoError::E01, ErrorCompilacion};
use regex::Regex;
use std::sync::LazyLock;

// RF-06 y RNF-03: expresión regular explícita de los identificadores.
pub const PATRON_IDENTIFICADOR: &str = r"^[A-Za-z][A-Za-z0-9_]*$";
static IDENTIFICADOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(PATRON_IDENTIFICADOR).expect("patrón fijo de identificadores válido")
});
static NUMERO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]+(\.[0-9]+)?$").expect("patrón fijo de números válido"));

pub fn es_identificador(texto: &str) -> bool {
    IDENTIFICADOR.is_match(texto) && !es_reservada(texto)
}

pub fn es_reservada(texto: &str) -> bool {
    matches!(
        texto.to_lowercase().as_str(),
        "inicio"
            | "fin"
            | "declare"
            | "entero"
            | "real"
            | "cadena"
            | "fecha"
            | "lógico"
            | "logico"
            | "si"
            | "entonces"
            | "sino"
            | "fsi"
            | "mq"
            | "haga"
            | "fmq"
            | "para"
            | "hasta"
            | "fpara"
            | "lea"
            | "escriba"
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoToken {
    Palabra,
    Numero,
    Simbolo,
    Invalido,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tipo: TipoToken,
    pub texto: String,
    pub linea: usize,
}

/// Los nombres inválidos se conservan completos: el parser decide entre
/// E02 (identificador) y E13 (expresión), importante para el caso P14.
pub fn tokenizar(codigo: &str) -> Result<Vec<Token>, ErrorCompilacion> {
    let codigo = codigo.strip_prefix('\u{feff}').unwrap_or(codigo);
    let mut caracteres = codigo.chars().peekable();
    let mut linea = 1;
    let mut tokens = Vec::new();
    while let Some(c) = caracteres.next() {
        if c == '\n' {
            linea += 1;
        } else if c.is_whitespace() {
            continue;
        } else if c.is_alphanumeric() || c == '_' || c == '.' {
            let mut texto = String::from(c);
            while let Some(&siguiente) = caracteres.peek() {
                if siguiente.is_alphanumeric() || siguiente == '_' || siguiente == '.' {
                    texto.push(siguiente);
                    caracteres.next();
                } else {
                    break;
                }
            }
            let tipo = if NUMERO.is_match(&texto) {
                TipoToken::Numero
            } else if IDENTIFICADOR.is_match(&texto) || es_reservada(&texto) {
                TipoToken::Palabra
            } else {
                TipoToken::Invalido
            };
            tokens.push(Token { tipo, texto, linea });
        } else if ",;()+-*/=<>".contains(c) {
            let mut texto = String::from(c);
            if matches!(
                (c, caracteres.peek().copied()),
                ('<', Some('=' | '>')) | ('>', Some('='))
            ) {
                texto.push(caracteres.next().expect("carácter consultado"));
            }
            tokens.push(Token {
                tipo: TipoToken::Simbolo,
                texto,
                linea,
            });
        } else {
            return Err(ErrorCompilacion::nuevo(
                E01,
                linea,
                format!("Carácter no permitido '{c}'."),
            ));
        }
    }
    Ok(tokens)
}
