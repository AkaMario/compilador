//! Intérprete opcional. No cambia las reglas ni los errores del analizador.
//! Ejecuta instrucciones por turnos y pausa Lea sin bloquear la ventana.
use crate::{
    errores::ErrorCompilacion,
    lexico::{self, TipoToken, Token},
    sintactico,
};
use std::collections::BTreeMap;

const MAX_PASOS: usize = 100_000;
const MAX_SALIDAS: usize = 5_000;

type Resultado<T> = Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorEjecucion {
    pub linea: usize,
    pub mensaje: String,
}

impl std::fmt::Display for ErrorEjecucion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error de ejecución - Línea {}: {}",
            self.linea, self.mensaje
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Estado {
    EnCurso,
    Esperando { variable: String, linea: usize },
    Finalizada,
    Detenida,
    Error(ErrorEjecucion),
}

#[derive(Debug, Clone)]
enum Valor {
    Numero(f64),
    Texto(String),
    Logico(bool),
}

impl std::fmt::Display for Valor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numero(n) => write!(f, "{}", if *n == 0.0 { 0.0 } else { *n }),
            Self::Texto(texto) => write!(f, "{texto}"),
            Self::Logico(v) => write!(f, "{}", if *v { "Verdadero" } else { "Falso" }),
        }
    }
}

#[derive(Clone)]
struct Variable {
    tipo: String,
    valor: Option<Valor>,
}

#[derive(Clone)]
enum Elemento {
    Numero(String),
    Variable(String),
    Operador(char),
}

type Expresion = Vec<Elemento>;

#[derive(Clone)]
struct Condicion {
    izquierda: Expresion,
    operador: String,
    derecha: Expresion,
}

#[derive(Clone)]
enum Accion {
    Asignar {
        nombre: String,
        expresion: Expresion,
    },
    Leer(String),
    Escribir(String),
    SiFalso {
        condicion: Condicion,
        destino: usize,
    },
    Saltar(usize),
    ProbarPara {
        nombre: String,
        limite: Expresion,
        destino: usize,
    },
    Incrementar {
        nombre: String,
        destino: usize,
    },
}

#[derive(Clone)]
struct Instruccion {
    linea: usize,
    accion: Accion,
}

enum Marco {
    Si {
        condicion: usize,
        salto: Option<usize>,
    },
    Mq {
        condicion: usize,
    },
    Para {
        condicion: usize,
        nombre: String,
    },
}

pub struct Ejecucion {
    pub estado: Estado,
    pub salida: Vec<String>,
    instrucciones: Vec<Instruccion>,
    variables: BTreeMap<String, Variable>,
    posicion: usize,
    pasos: usize,
}

impl Ejecucion {
    pub fn nueva(codigo: &str) -> Result<Self, ErrorCompilacion> {
        sintactico::analizar(codigo)?;
        let tokens = lexico::tokenizar(codigo)?;
        let (instrucciones, variables) = compilar(&tokens);
        Ok(Self {
            estado: Estado::EnCurso,
            salida: Vec::new(),
            instrucciones,
            variables,
            posicion: 0,
            pasos: 0,
        })
    }

    /// Un presupuesto por cuadro mantiene disponibles el editor y Detener.
    pub fn avanzar(&mut self, presupuesto: usize) {
        for _ in 0..presupuesto {
            if self.estado != Estado::EnCurso {
                break;
            }
            let Some(instruccion) = self.instrucciones.get(self.posicion).cloned() else {
                self.estado = Estado::Finalizada;
                break;
            };
            if self.pasos >= MAX_PASOS {
                self.fallar(instruccion.linea, format!("Se alcanzó el límite de {MAX_PASOS} instrucciones. Revisa si hay un ciclo infinito."));
                break;
            }
            self.pasos += 1;
            if let Err(mensaje) = self.paso(&instruccion) {
                self.fallar(instruccion.linea, mensaje);
                break;
            }
        }
        if self.estado == Estado::EnCurso && self.posicion == self.instrucciones.len() {
            self.estado = Estado::Finalizada;
        }
    }

    fn fallar(&mut self, linea: usize, mensaje: String) {
        self.estado = Estado::Error(ErrorEjecucion { linea, mensaje });
    }

    pub fn detener(&mut self) {
        if matches!(self.estado, Estado::EnCurso | Estado::Esperando { .. }) {
            self.estado = Estado::Detenida;
        }
    }

    pub fn aportar_entrada(&mut self, texto: &str) -> Resultado<()> {
        let Estado::Esperando { variable, .. } = &self.estado else {
            return Err("El programa no está esperando una entrada.".into());
        };
        let nombre = variable.clone();
        let tipo = &self.variables[&nombre].tipo;
        let valor = match tipo.as_str() {
            "entero" => {
                let numero = numero_finito(texto.trim())?;
                entero(numero)?;
                Valor::Numero(numero)
            }
            "real" => Valor::Numero(numero_finito(texto.trim())?),
            "logico" | "lógico" => match texto.trim().to_lowercase().as_str() {
                "verdadero" | "true" => Valor::Logico(true),
                "falso" | "false" => Valor::Logico(false),
                _ => return Err("Escribe Verdadero o Falso para un dato Lógico.".into()),
            },
            // La especificación no define un formato de fecha: se conserva el texto.
            _ => Valor::Texto(texto.to_owned()),
        };
        self.variables
            .get_mut(&nombre)
            .expect("variable validada")
            .valor = Some(valor);
        self.posicion += 1;
        self.estado = Estado::EnCurso;
        Ok(())
    }

    fn paso(&mut self, instruccion: &Instruccion) -> Resultado<()> {
        match &instruccion.accion {
            Accion::Asignar { nombre, expresion } => {
                let valor = self.evaluar(expresion)?;
                self.variables
                    .get_mut(nombre)
                    .expect("destino declarado")
                    .valor = Some(valor);
                self.posicion += 1;
            }
            Accion::Leer(nombre) => {
                self.estado = Estado::Esperando {
                    variable: nombre.clone(),
                    linea: instruccion.linea,
                }
            }
            Accion::Escribir(nombre) => {
                let valor = self.valor(nombre)?.to_string();
                if self.salida.len() >= MAX_SALIDAS {
                    return Err(format!(
                        "Se alcanzó el límite de {MAX_SALIDAS} líneas de salida."
                    ));
                }
                self.salida.push(valor);
                self.posicion += 1;
            }
            Accion::SiFalso { condicion, destino } => {
                self.posicion = if self.comparar(condicion)? {
                    self.posicion + 1
                } else {
                    *destino
                };
            }
            Accion::Saltar(destino) => self.posicion = *destino,
            Accion::ProbarPara {
                nombre,
                limite,
                destino,
            } => {
                let actual = como_numero(self.valor(nombre)?)?;
                let final_ = como_numero(self.evaluar(limite)?)?;
                entero(actual)?;
                entero(final_)?;
                self.posicion = if actual <= final_ {
                    self.posicion + 1
                } else {
                    *destino
                };
            }
            Accion::Incrementar { nombre, destino } => {
                let actual = como_numero(self.valor(nombre)?)?;
                entero(actual)?;
                let siguiente = actual + 1.0;
                entero(siguiente)?;
                self.variables
                    .get_mut(nombre)
                    .expect("contador declarado")
                    .valor = Some(Valor::Numero(siguiente));
                self.posicion = *destino;
            }
        }
        Ok(())
    }

    fn valor(&self, nombre: &str) -> Resultado<Valor> {
        self.variables.get(nombre).and_then(|v| v.valor.clone()).ok_or_else(|| format!("Variable '{nombre}' sin inicializar. Asigna un valor o usa Lea antes de consultarla."))
    }

    fn evaluar(&self, expresion: &Expresion) -> Resultado<Valor> {
        let mut pila = Vec::new();
        for elemento in expresion {
            match elemento {
                Elemento::Numero(n) => pila.push(Valor::Numero(numero_finito(n)?)),
                Elemento::Variable(nombre) => pila.push(self.valor(nombre)?),
                Elemento::Operador(operador) => {
                    let derecha =
                        como_numero(pila.pop().ok_or("Falta un operando en ejecución.")?)?;
                    let izquierda =
                        como_numero(pila.pop().ok_or("Falta un operando en ejecución.")?)?;
                    let resultado = match operador {
                        '+' => izquierda + derecha,
                        '-' => izquierda - derecha,
                        '*' => izquierda * derecha,
                        '/' if derecha == 0.0 => return Err("División por cero.".into()),
                        '/' => izquierda / derecha,
                        _ => return Err("Operador de ejecución desconocido.".into()),
                    };
                    if !resultado.is_finite() {
                        return Err("El resultado numérico excede el rango de ejecución.".into());
                    }
                    pila.push(Valor::Numero(resultado));
                }
            }
        }
        pila.pop()
            .ok_or_else(|| "La expresión no produjo un valor.".into())
    }

    fn comparar(&self, condicion: &Condicion) -> Resultado<bool> {
        let izquierda = self.evaluar(&condicion.izquierda)?;
        let derecha = self.evaluar(&condicion.derecha)?;
        let orden =
            match (izquierda, derecha) {
                (Valor::Numero(a), Valor::Numero(b)) => a.partial_cmp(&b),
                (Valor::Texto(a), Valor::Texto(b)) => Some(a.cmp(&b)),
                (Valor::Logico(a), Valor::Logico(b)) => Some(a.cmp(&b)),
                _ => return Err(
                    "No se pueden comparar valores de categorías distintas durante la ejecución."
                        .into(),
                ),
            }
            .ok_or("Comparación numérica no válida.")?;
        use std::cmp::Ordering::{Equal, Greater, Less};
        Ok(match condicion.operador.as_str() {
            "<" => orden == Less,
            ">" => orden == Greater,
            "=" => orden == Equal,
            "<>" => orden != Equal,
            "<=" => orden != Greater,
            ">=" => orden != Less,
            _ => return Err("Operador de comparación no válido.".into()),
        })
    }
}

fn numero_finito(texto: &str) -> Resultado<f64> {
    let numero: f64 = texto.parse().map_err(|_| "Se esperaba un número válido.")?;
    if !numero.is_finite() {
        return Err("Se esperaba un número finito dentro del rango de ejecución.".into());
    }
    Ok(numero)
}

fn como_numero(valor: Valor) -> Resultado<f64> {
    if let Valor::Numero(n) = valor {
        Ok(n)
    } else {
        Err("Esta operación necesita valores numéricos.".into())
    }
}

fn entero(numero: f64) -> Resultado<()> {
    if numero.fract() != 0.0 || numero.abs() > 9_007_199_254_740_991.0 {
        return Err("Se necesita un entero entre -9007199254740991 y 9007199254740991 para esta entrada o contador Para.".into());
    }
    Ok(())
}

fn operando(token: &Token) -> Elemento {
    if token.tipo == TipoToken::Numero {
        Elemento::Numero(token.texto.clone())
    } else {
        Elemento::Variable(token.texto.to_ascii_lowercase())
    }
}

/// Shunting-yard: produce notación posfija con precedencia y asociación izquierda.
fn expresion(tokens: &[Token]) -> Expresion {
    let mut salida = Vec::new();
    let mut operadores: Vec<char> = Vec::new();
    fn prioridad(op: char) -> u8 {
        match op {
            '*' | '/' => 2,
            '+' | '-' => 1,
            _ => 0,
        }
    }
    for token in tokens {
        match token.texto.as_str() {
            "(" => operadores.push('('),
            ")" => {
                while let Some(op) = operadores.pop() {
                    if op == '(' {
                        break;
                    }
                    salida.push(Elemento::Operador(op));
                }
            }
            "+" | "-" | "*" | "/" => {
                let op = token.texto.chars().next().expect("operador no vacío");
                while operadores.last().is_some_and(|&anterior| {
                    anterior != '(' && prioridad(anterior) >= prioridad(op)
                }) {
                    salida.push(Elemento::Operador(
                        operadores.pop().expect("operador consultado"),
                    ));
                }
                operadores.push(op);
            }
            _ => salida.push(operando(token)),
        }
    }
    while let Some(op) = operadores.pop() {
        salida.push(Elemento::Operador(op));
    }
    salida
}

fn compilar(tokens: &[Token]) -> (Vec<Instruccion>, BTreeMap<String, Variable>) {
    let mut instrucciones: Vec<Instruccion> = Vec::new();
    let mut variables = BTreeMap::new();
    let mut bloques = Vec::new();
    let mut p = 1; // Inicio ya fue validado.
    while p < tokens.len() {
        let linea = tokens[p].linea;
        match tokens[p].texto.to_lowercase().as_str() {
            "fin" => break,
            ";" => p += 1,
            "declare" => {
                let inicio = p + 1;
                while tokens[p].texto != ";" {
                    p += 1;
                }
                let tipo = tokens[p - 1].texto.to_lowercase();
                for nombre in &tokens[inicio..p - 1] {
                    if nombre.texto != "," {
                        variables.insert(
                            nombre.texto.to_ascii_lowercase(),
                            Variable {
                                tipo: tipo.clone(),
                                valor: None,
                            },
                        );
                    }
                }
                p += 1;
            }
            "lea" | "escriba" => {
                let nombre = tokens[p + 2].texto.to_ascii_lowercase();
                let accion = if tokens[p].texto.eq_ignore_ascii_case("lea") {
                    Accion::Leer(nombre)
                } else {
                    Accion::Escribir(nombre)
                };
                instrucciones.push(Instruccion { linea, accion });
                p += 5;
            }
            "si" | "mq" => {
                let condicion = Condicion {
                    izquierda: vec![operando(&tokens[p + 1])],
                    operador: tokens[p + 2].texto.clone(),
                    derecha: vec![operando(&tokens[p + 3])],
                };
                let indice = instrucciones.len();
                if tokens[p].texto.eq_ignore_ascii_case("si") {
                    bloques.push(Marco::Si {
                        condicion: indice,
                        salto: None,
                    });
                } else {
                    bloques.push(Marco::Mq { condicion: indice });
                }
                instrucciones.push(Instruccion {
                    linea,
                    accion: Accion::SiFalso {
                        condicion,
                        destino: 0,
                    },
                });
                p += 5;
            }
            "sino" => {
                let indice = instrucciones.len();
                instrucciones.push(Instruccion {
                    linea,
                    accion: Accion::Saltar(0),
                });
                if let Some(Marco::Si { condicion, salto }) = bloques.last_mut() {
                    fijar_destino(&mut instrucciones[*condicion], indice + 1);
                    *salto = Some(indice);
                }
                p += 1;
            }
            "fsi" => {
                if let Some(Marco::Si { condicion, salto }) = bloques.pop() {
                    let destino = instrucciones.len();
                    fijar_destino(&mut instrucciones[salto.unwrap_or(condicion)], destino);
                }
                p += 1;
            }
            "fmq" => {
                if let Some(Marco::Mq { condicion }) = bloques.pop() {
                    instrucciones.push(Instruccion {
                        linea,
                        accion: Accion::Saltar(condicion),
                    });
                    let destino = instrucciones.len();
                    fijar_destino(&mut instrucciones[condicion], destino);
                }
                p += 1;
            }
            "para" => {
                let nombre = tokens[p + 1].texto.to_ascii_lowercase();
                instrucciones.push(Instruccion {
                    linea,
                    accion: Accion::Asignar {
                        nombre: nombre.clone(),
                        expresion: vec![operando(&tokens[p + 3])],
                    },
                });
                let condicion = instrucciones.len();
                instrucciones.push(Instruccion {
                    linea,
                    accion: Accion::ProbarPara {
                        nombre: nombre.clone(),
                        limite: vec![operando(&tokens[p + 5])],
                        destino: 0,
                    },
                });
                bloques.push(Marco::Para { condicion, nombre });
                p += 7;
            }
            "fpara" => {
                if let Some(Marco::Para { condicion, nombre }) = bloques.pop() {
                    instrucciones.push(Instruccion {
                        linea,
                        accion: Accion::Incrementar {
                            nombre,
                            destino: condicion,
                        },
                    });
                    let destino = instrucciones.len();
                    fijar_destino(&mut instrucciones[condicion], destino);
                }
                p += 1;
            }
            _ => {
                let nombre = tokens[p].texto.to_ascii_lowercase();
                let inicio = p + 2;
                while tokens[p].texto != ";" {
                    p += 1;
                }
                instrucciones.push(Instruccion {
                    linea,
                    accion: Accion::Asignar {
                        nombre,
                        expresion: expresion(&tokens[inicio..p]),
                    },
                });
                p += 1;
            }
        }
    }
    (instrucciones, variables)
}

fn fijar_destino(instruccion: &mut Instruccion, nuevo: usize) {
    match &mut instruccion.accion {
        Accion::SiFalso { destino, .. }
        | Accion::ProbarPara { destino, .. }
        | Accion::Saltar(destino) => *destino = nuevo,
        _ => unreachable!("solo los saltos tienen destino"),
    }
}
