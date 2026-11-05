# Matriz de trazabilidad

Fuente normativa: especificación local en `context/`. Los resultados observados se registran en `docs/evidencias/`. La política de RF-29 es detenerse en el primer error de las etapas de análisis. Las pruebas agrupadas contienen varias entradas; no confundir el número de funciones de prueba con el número de programas analizados.

| Requisito | Implementación | Evidencia |
|---|---|---|
| RF-01 | `Parser::programa`: Inicio y Fin únicos | Aceptación: estructura ausente/repetida; anexo |
| RF-02 | `programa`: comienzo y ausencia de tokens posteriores a Fin | Aceptación: texto anterior/posterior |
| RF-03 | `terminar`; cierres con `aceptar(";")` opcional | P08, asignación sin `;`, cierres con/sin `;` |
| RF-04 | `declaracion`: lista y tipo antes de `;` | P01, P02, P06, P07 |
| RF-05 | `declaracion`: alternancia nombre/coma | P05; coma inicial/final |
| RF-06 | `lexico::es_identificador`, regex explícita | P03; nombres con _, dígitos, ñ, reservadas y espacios |
| RF-07 | `TipoDato::leer` | Cinco tipos y alias Logico/Lógico con mayúsculas |
| RF-08 | Tabla de símbolos y lista de nombres de la declaración | Duplicado entre declaraciones y sin distinción de mayúsculas |
| RF-09 | `declarada` en expresiones, I/O, condiciones y Para | P13; Lea/Escriba y Para con nombres desconocidos |
| RF-10 | `asignacion` | P09, falta de =, lado izquierdo no declarado |
| RF-11 | `expresion`: alternancia de operandos y operadores | P10, P14, operandos consecutivos, operador colgante; 80 combinaciones válidas |
| RF-12 | Pila de paréntesis y comprobación de orden | P11, P12, cierre antes de apertura y 2000 niveles |
| RF-13 | `bloque_condicional`, `componente` y pila de bloques | B01, B02; Si sin cierre |
| RF-14 | `sino`: solo en Si superior, una sola vez | B06; Sino fuera de Si |
| RF-15 | `cerrar` exige coincidencia con la cima de la pila | Anexo y anidamiento profundo |
| RF-16 | `bloque_condicional` para MQ | B03; MQ sin Haga |
| RF-17 | `cerrar` y verificación final de bloques | MQ sin FMQ, anexo |
| RF-18 | `bloque_para`: var, =, índice, Hasta, índice y Haga | B04; componentes ausentes |
| RF-19 | `indice` y `exigir_entero` | Real como contador, índice decimal y variable índice Real |
| RF-20 | Pila Para/FPara | B05; contador de cierres y orden |
| RF-21 | `condicion` y `operando` | Comparadores; condiciones con operadores extras o paréntesis inválidas |
| RF-22 | Pila única con tipos distintos; cuerpo vacío admitido | Anexo, Si/MQ/Para mezclados y vacíos |
| RF-23 | `cerrar` rechaza pila vacía o tipo incorrecto | FSi/FMQ/FPara huérfanos; cruce de cierres |
| RF-24 | `entrada_salida` para Lea | Anexo, variable desconocida, falta de paréntesis, argumento numérico |
| RF-25 | `entrada_salida` para Escriba | Anexo, espacios antes de (, expresiones no admitidas |
| RF-26 | `ErrorCompilacion` con código, tipo, línea y causa | Pruebas de terminal y capturas del editor |
| RF-27 | Línea de apertura guardada en Bloque; línea de token para cierre | B05; bloque externo sin cierre en línea absoluta 3 |
| RF-28 | Resultado explícito en GUI y CLI | anexo_a, captura de programa correcto |
| RF-29 | `Result` propaga primer fallo y permite corregir/reanalizar | P14; CLI termina con estado 1; mensaje del panel |

| Requisito | Implementación o condición | Evidencia / alcance |
|---|---|---|
| RNF-01 | Rust, edición 2024 | Justificación en el informe |
| RNF-02 | Cargo.lock, instalación y comandos documentados | Compilación y ventana verificadas en Fedora. Probar previamente en el equipo de sustentación |
| RNF-03 | Constante `PATRON_IDENTIFICADOR` y crate regex | Código fuente e imagen del fragmento del informe |
| RNF-04 | TextEdit multilínea, campo de ruta y botón Abrir archivo | Pruebas de archivos y capturas nativas |
| RNF-05 | Mensajes E01-E16 y problemas de carga en español | Casos observados y pruebas CLI |
| RNF-06 | Motor puro respecto a la entrada, recorrido estable y BTreeMap | Prueba de determinismo con diez repeticiones |
| RNF-07 | Separación de lexico, sintactico, errores e interfaz | Estructura de `src/` |
| RNF-08 | Debe respetarse la autoría académica | Implementación asistida por IA, indicada expresamente. El estudiante debe revisar, comprender y declarar la asistencia según las reglas de su curso; la aceptación académica no se verifica por software |

## Entregables de la sección 9

- Aplicación: fuente compilable y ejecutable local en `target/debug/compilador`.
- Informe: `output/pdf/informe_tecnico.pdf`, nueve páginas de contenido; texto editable en `docs/informe_tecnico.md`.
- Siete funcionalidades: cada una tiene requisitos, estructuras, rutina, imagen real del código y entradas válida/inválida con salida observada.
- Marco teórico, justificación de Rust y dificultades técnicas: incluidos en el informe.
- El código fuente completo se entrega como proyecto separado y no se cuenta en las nueve páginas.

La implementación se verifica contra la especificación entregada, no contra reglas adicionales ajenas a ella. No se afirma que las pruebas cubran toda entrada posible ni que garanticen una calificación o la compatibilidad con otros equipos.

## Ampliación posterior: consola de ejecución

A petición del usuario se añadió un intérprete opcional en `src/ejecucion.rs`, activado con Ejecutar/F5. Mantiene intacto el modo Analizar exigido por la especificación: cargar o analizar no ejecuta instrucciones. La ampliación agrega evaluación de expresiones, Escriba con valores, Lea interactivo, Si/Sino, MQ, Para y Detener.

Sus decisiones de ejecución y límites se documentan en `docs/ejecucion.md`. Las pruebas en `tests/ejecucion.rs` y las pruebas del editor verifican valores, orden, precedencia, ciclos, pausa/reanudación y eventos de teclado. Los errores de ejecución están separados de E01-E16. No se atribuye esta ampliación a un RF que originalmente excluía la ejecución.
