//! Validación estructural y semántica. No ejecuta ni evalúa el pseudocódigo.
use crate::errores::{CodigoError, CodigoError::*, ErrorCompilacion};
use crate::lexico::{TipoToken, Token, es_identificador, es_reservada, tokenizar};
use std::collections::BTreeMap;

type Resultado<T = ()> = Result<T, ErrorCompilacion>;

#[derive(Debug)]
pub struct Resumen {
    pub tokens: usize,
    pub variables: usize,
}

pub fn analizar(codigo: &str) -> Resultado<Resumen> {
    let tokens = tokenizar(codigo)?;
    let mut parser = Parser {
        tokens: &tokens,
        posicion: 0,
        ultima_linea: codigo.lines().count().max(1),
        simbolos: BTreeMap::new(),
        bloques: Vec::new(),
    };
    parser.programa()?;
    Ok(Resumen {
        tokens: tokens.len(),
        variables: parser.simbolos.len(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TipoDato {
    Entero,
    Real,
    Cadena,
    Fecha,
    Logico,
}

impl TipoDato {
    fn leer(texto: &str) -> Option<Self> {
        match texto.to_lowercase().as_str() {
            "entero" => Some(Self::Entero),
            "real" => Some(Self::Real),
            "cadena" => Some(Self::Cadena),
            "fecha" => Some(Self::Fecha),
            "logico" | "lógico" => Some(Self::Logico),
            _ => None,
        }
    }
}

struct Bloque {
    nombre: &'static str,
    cierre: &'static str,
    linea: usize,
    tiene_sino: bool,
}

struct Parser<'a> {
    tokens: &'a [Token],
    posicion: usize,
    ultima_linea: usize,
    simbolos: BTreeMap<String, TipoDato>,
    bloques: Vec<Bloque>,
}

impl Parser<'_> {
    fn actual(&self) -> Option<&Token> {
        self.tokens.get(self.posicion)
    }
    fn linea(&self) -> usize {
        self.actual().map_or(self.ultima_linea, |t| t.linea)
    }
    fn es(&self, texto: &str) -> bool {
        self.actual()
            .is_some_and(|t| t.texto.to_lowercase() == texto)
    }
    fn aceptar(&mut self, texto: &str) -> bool {
        if self.es(texto) {
            self.posicion += 1;
            true
        } else {
            false
        }
    }
    fn error(&self, codigo: CodigoError, mensaje: impl Into<String>) -> ErrorCompilacion {
        ErrorCompilacion::nuevo(codigo, self.linea(), mensaje)
    }

    fn programa(&mut self) -> Resultado {
        if !self.aceptar("inicio") {
            return Err(self.error(E03, "Falta 'Inicio' al comienzo del programa."));
        }
        while self.es("declare") {
            self.declaracion()?;
        }
        while self.actual().is_some() && !self.es("fin") {
            self.sentencia()?;
        }
        if let Some(bloque) = self.bloques.last() {
            return Err(ErrorCompilacion::nuevo(
                E10,
                bloque.linea,
                format!("Bloque {} sin su {}.", bloque.nombre, bloque.cierre),
            ));
        }
        if !self.aceptar("fin") {
            return Err(self.error(E04, "Falta 'Fin' al final del programa."));
        }
        if self.actual().is_some() {
            return Err(self.error(E04, "No se permite contenido después de 'Fin'."));
        }
        Ok(())
    }

    fn identificador(&mut self) -> Resultado<Token> {
        let token = self
            .actual()
            .cloned()
            .ok_or_else(|| self.error(E02, "Falta un identificador."))?;
        if !es_identificador(&token.texto) {
            return Err(self.error(E02, format!("Identificador inválido '{}'; debe empezar por una letra ASCII y no ser una palabra reservada.", token.texto)));
        }
        self.posicion += 1;
        Ok(token)
    }

    fn declarada(&self, token: &Token) -> Resultado<TipoDato> {
        self.simbolos
            .get(&token.texto.to_ascii_lowercase())
            .copied()
            .ok_or_else(|| {
                ErrorCompilacion::nuevo(
                    E14,
                    token.linea,
                    format!("Variable '{}' no declarada.", token.texto),
                )
            })
    }

    fn terminar(&mut self) -> Resultado {
        if self.aceptar(";") {
            return Ok(());
        }
        let linea = self
            .tokens
            .get(self.posicion.saturating_sub(1))
            .map_or(self.linea(), |t| t.linea);
        Err(ErrorCompilacion::nuevo(
            E05,
            linea,
            "Falta ';' al final de la sentencia.",
        ))
    }

    fn declaracion(&mut self) -> Resultado {
        self.posicion += 1; // Declare
        let mut nombres: Vec<Token> = Vec::new();
        loop {
            if self.es(",") || self.es(";") || self.actual().is_none() {
                return Err(self.error(
                    E06,
                    "Declaración inválida: falta un identificador en la lista.",
                ));
            }
            if !nombres.is_empty()
                && self
                    .actual()
                    .is_some_and(|t| TipoDato::leer(&t.texto).is_some())
            {
                return Err(self.error(E06, "Declaración inválida: coma al final de la lista."));
            }
            let nombre = self.identificador()?;
            let clave = nombre.texto.to_ascii_lowercase();
            if self.simbolos.contains_key(&clave)
                || nombres
                    .iter()
                    .any(|n| n.texto.eq_ignore_ascii_case(&nombre.texto))
            {
                return Err(ErrorCompilacion::nuevo(
                    E15,
                    nombre.linea,
                    format!("Variable '{}' ya declarada.", nombre.texto),
                ));
            }
            nombres.push(nombre);
            if !self.aceptar(",") {
                break;
            }
        }
        let Some(tipo) = self.actual().and_then(|t| TipoDato::leer(&t.texto)) else {
            // «a 1 Entero» es un nombre separado por espacios (sección 3.2).
            if self
                .tokens
                .get(self.posicion + 1)
                .is_some_and(|t| TipoDato::leer(&t.texto).is_some())
            {
                return Err(self.error(E02, "Identificador inválido: no se admiten espacios dentro del nombre; separe variables con coma."));
            }
            return Err(self.error(E06, "Declaración inválida: falta un tipo válido (Entero, Real, Cadena, Fecha o Lógico) o una coma entre variables."));
        };
        self.posicion += 1;
        self.terminar()?;
        for nombre in nombres {
            self.simbolos
                .insert(nombre.texto.to_ascii_lowercase(), tipo);
        }
        Ok(())
    }

    fn sentencia(&mut self) -> Resultado {
        let token = self.actual().expect("sentencia con token").clone();
        match token.texto.to_lowercase().as_str() {
            "inicio" => Err(self.error(E03, "'Inicio' solo puede aparecer una vez, al comienzo.")),
            "declare" => Err(self.error(E06, "Las declaraciones deben aparecer al inicio, antes de las demás sentencias y fuera de los bloques.")),
            "si" | "mq" => self.bloque_condicional(),
            "para" => self.bloque_para(),
            "sino" => self.sino(),
            "fsi" | "fmq" | "fpara" => self.cerrar(),
            "lea" | "escriba" => self.entrada_salida(),
            _ if es_identificador(&token.texto) || token.tipo == TipoToken::Invalido => self.asignacion(),
            _ => Err(self.error(E07, format!("Sentencia no reconocida: '{}'.", token.texto))),
        }
    }

    fn asignacion(&mut self) -> Resultado {
        let variable = self.identificador()?;
        if !self.aceptar("=") {
            return Err(self.error(E07, "Asignación inválida: falta el signo '='."));
        }
        self.declarada(&variable)?;
        self.expresion()?;
        self.terminar()
    }

    /// Reconoce la forma de una expresión con una pila y un estado
    /// operando/operador. No calcula valores ni exige tipos compatibles.
    fn expresion(&mut self) -> Resultado {
        let mut parentesis: Vec<usize> = Vec::new();
        let mut espera_operando = true;
        while let Some(token) = self.actual().cloned() {
            match token.texto.as_str() {
                "(" => {
                    if !espera_operando {
                        return Err(
                            self.error(E13, "Expresión inválida: falta un operador antes de '('.")
                        );
                    }
                    parentesis.push(token.linea);
                    self.posicion += 1;
                }
                ")" => {
                    if parentesis.is_empty() {
                        return Err(
                            self.error(E12, "Paréntesis de cierre sin apertura en la expresión.")
                        );
                    }
                    if espera_operando {
                        return Err(
                            self.error(E13, "Expresión inválida: falta un operando antes de ')'.")
                        );
                    }
                    parentesis.pop();
                    self.posicion += 1;
                }
                "+" | "-" | "*" | "/" => {
                    if espera_operando {
                        return Err(self.error(
                            E13,
                            "Expresión inválida: falta un operando antes del operador.",
                        ));
                    }
                    espera_operando = true;
                    self.posicion += 1;
                }
                _ if token.tipo == TipoToken::Numero || es_identificador(&token.texto) => {
                    if !espera_operando {
                        // Una nueva asignación sin ';' es E05, aunque ocupe la misma línea.
                        if self
                            .tokens
                            .get(self.posicion + 1)
                            .is_some_and(|t| t.texto == "=")
                        {
                            break;
                        }
                        return Err(self.error(
                            E13,
                            "Expresión inválida: hay operandos seguidos sin operador.",
                        ));
                    }
                    if token.tipo != TipoToken::Numero {
                        self.declarada(&token)?;
                    }
                    espera_operando = false;
                    self.posicion += 1;
                }
                _ if token.tipo == TipoToken::Invalido => {
                    let codigo = if token
                        .texto
                        .starts_with(|c: char| c.is_alphabetic() || c == '_')
                    {
                        E02
                    } else {
                        E13
                    };
                    return Err(self.error(
                        codigo,
                        format!("Expresión inválida: operando '{}' no válido.", token.texto),
                    ));
                }
                _ => break,
            }
        }
        if let Some(&linea) = parentesis.last() {
            return Err(ErrorCompilacion::nuevo(
                E12,
                linea,
                "Paréntesis de apertura sin cierre en la expresión.",
            ));
        }
        if espera_operando {
            return Err(self.error(E13, "Expresión inválida: falta un operando."));
        }
        if self.actual().is_some_and(|t| {
            !es_reservada(&t.texto) && t.texto != ";" && t.tipo == TipoToken::Simbolo
        }) {
            return Err(self.error(
                E13,
                "Expresión inválida: símbolo no permitido en una expresión aritmética.",
            ));
        }
        Ok(())
    }

    fn entrada_salida(&mut self) -> Resultado {
        let nombre = self.actual().expect("primitiva").texto.clone();
        self.posicion += 1;
        if !self.aceptar("(") {
            return Err(self.error(E07, format!("{nombre} inválido: falta '('.")));
        }
        if self
            .actual()
            .is_some_and(|t| t.tipo == TipoToken::Invalido || es_reservada(&t.texto))
        {
            return Err(self.error(
                E02,
                "Identificador inválido en la operación de entrada o salida.",
            ));
        }
        if !self.actual().is_some_and(|t| es_identificador(&t.texto)) {
            return Err(self.error(
                E07,
                format!("{nombre} requiere exactamente un identificador."),
            ));
        }
        let variable = self.identificador()?;
        self.declarada(&variable)?;
        if !self.aceptar(")") {
            return Err(self.error(
                E07,
                format!("{nombre} inválido: falta ')' o sobran argumentos."),
            ));
        }
        self.terminar()
    }

    fn operando(&mut self) -> Resultado {
        let Some(token) = self.actual().cloned() else {
            return Err(self.error(E08, "Condición inválida: falta un operando."));
        };
        if token.tipo == TipoToken::Numero {
            self.posicion += 1;
            return Ok(());
        }
        if es_identificador(&token.texto) {
            self.posicion += 1;
            self.declarada(&token)?;
            return Ok(());
        }
        Err(self.error(
            E08,
            "Condición inválida: se esperaba un identificador o un número.",
        ))
    }

    fn condicion(&mut self) -> Resultado {
        self.operando()?;
        if !self
            .actual()
            .is_some_and(|t| matches!(t.texto.as_str(), ">" | "<" | "<>" | ">=" | "<=" | "="))
        {
            return Err(self.error(
                E08,
                "Condición inválida: falta un operador relacional válido.",
            ));
        }
        self.posicion += 1;
        self.operando()?;
        if self
            .actual()
            .is_some_and(|t| t.tipo == TipoToken::Numero || t.tipo == TipoToken::Invalido)
            || (self.actual().is_some_and(|t| es_identificador(&t.texto))
                && self.tokens.get(self.posicion + 1).is_some_and(|t| {
                    matches!(t.texto.to_lowercase().as_str(), "entonces" | "haga")
                }))
        {
            return Err(self.error(
                E08,
                "Condición inválida: sobran operandos antes del encabezado.",
            ));
        }
        if self.actual().is_some_and(|t| {
            matches!(
                t.texto.as_str(),
                ">" | "<" | "<>" | ">=" | "<=" | "=" | "+" | "-" | "*" | "/" | "(" | ")"
            )
        }) {
            return Err(self.error(E08, "Condición inválida: debe contener exactamente dos operandos y un operador relacional."));
        }
        Ok(())
    }

    fn componente(&mut self, palabra: &str, nombre: &str, linea: usize) -> Resultado {
        if self.aceptar(palabra) {
            return Ok(());
        }
        Err(ErrorCompilacion::nuevo(
            E09,
            linea,
            format!("Falta '{palabra}' en el bloque {nombre}."),
        ))
    }

    fn bloque_condicional(&mut self) -> Resultado {
        let token = self.actual().expect("apertura").clone();
        let (nombre, cierre, componente) = if self.es("si") {
            ("Si", "FSi", "entonces")
        } else {
            ("MQ", "FMQ", "haga")
        };
        self.posicion += 1;
        self.condicion()?;
        self.componente(componente, nombre, token.linea)?;
        self.bloques.push(Bloque {
            nombre,
            cierre,
            linea: token.linea,
            tiene_sino: false,
        });
        Ok(())
    }

    fn indice(&mut self) -> Resultado {
        let Some(token) = self.actual().cloned() else {
            return Err(self.error(E16, "Falta un índice entero en el bloque Para."));
        };
        if token.tipo == TipoToken::Numero {
            if token.texto.contains('.') {
                return Err(self.error(
                    E16,
                    "Los índices del Para deben ser enteros, sin parte decimal.",
                ));
            }
            self.posicion += 1;
            return Ok(());
        }
        let variable = self.identificador()?;
        self.exigir_entero(&variable)
    }

    fn exigir_entero(&self, variable: &Token) -> Resultado {
        if self.declarada(variable)? != TipoDato::Entero {
            return Err(ErrorCompilacion::nuevo(
                E16,
                variable.linea,
                format!("La variable '{}' del Para debe ser Entero.", variable.texto),
            ));
        }
        Ok(())
    }

    fn bloque_para(&mut self) -> Resultado {
        let linea = self.linea();
        self.posicion += 1;
        let variable = self.identificador()?;
        self.exigir_entero(&variable)?;
        self.componente("=", "Para", linea)?;
        self.indice()?;
        self.componente("hasta", "Para", linea)?;
        self.indice()?;
        self.componente("haga", "Para", linea)?;
        self.bloques.push(Bloque {
            nombre: "Para",
            cierre: "FPara",
            linea,
            tiene_sino: false,
        });
        Ok(())
    }

    fn sino(&mut self) -> Resultado {
        match self.bloques.last_mut() {
            Some(bloque) if bloque.nombre == "Si" && !bloque.tiene_sino => {
                bloque.tiene_sino = true;
                self.posicion += 1;
                Ok(())
            }
            _ => Err(self.error(E11, "'Sino' fuera de un Si o repetido en el mismo bloque.")),
        }
    }

    fn cerrar(&mut self) -> Resultado {
        let cierre = self.actual().expect("cierre").texto.clone();
        if !self
            .bloques
            .last()
            .is_some_and(|b| b.cierre.eq_ignore_ascii_case(&cierre))
        {
            return Err(self.error(
                E11,
                format!(
                    "'{cierre}' sin apertura correspondiente o fuera del orden de anidamiento."
                ),
            ));
        }
        self.bloques.pop();
        self.posicion += 1;
        self.aceptar(";");
        Ok(())
    }
}
