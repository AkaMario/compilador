****# Guía paso a paso de la lógica del compilador

Esta guía explica la implementación que está en `src`: cómo entra el pseudocódigo, cómo se convierte en tokens, cómo se valida y cómo se presenta el resultado. Puedes leerla en orden sin conocer previamente cómo funciona un compilador.

## 1. Qué hace este proyecto

El proyecto es un simulador de compilador escrito en Rust. Tiene un editor gráfico y un motor que reconoce un lenguaje pequeño de pseudocódigo.

La operación principal es **Analizar**: comprueba caracteres, nombres, estructura, declaraciones y algunas reglas semánticas. Devuelve un resumen o el primer error.

El alcance principal excluye ejecutar el pseudocódigo: no calcula expresiones, no solicita datos con `Lea`, no imprime valores con `Escriba` ni genera código máquina. La ventana contiene un editor y un reporte de análisis.

## 2. Mapa de los archivos

| Archivo | Responsabilidad |
| --- | --- |
| [`src/main.rs`](../src/main.rs) | Punto de entrada. Lee argumentos y decide entre abrir la ventana, analizar un archivo o mostrar ayuda. |
| [`src/lib.rs`](../src/lib.rs) | Expone los módulos del motor para que puedan usarlos la aplicación y las pruebas. |
| [`src/archivos.rs`](../src/archivos.rs) | Carga archivos de texto UTF-8. |
| [`src/lexico.rs`](../src/lexico.rs) | Divide el código en tokens y reconoce las reglas básicas de nombres y números. |
| [`src/sintactico.rs`](../src/sintactico.rs) | Recorre los tokens y comprueba estructura y reglas semánticas. |
| [`src/errores.rs`](../src/errores.rs) | Define los errores E01–E16 y cómo se muestran. |
| [`src/interfaz.rs`](../src/interfaz.rs) | Gestiona el editor, el botón Analizar y el reporte de resultados. |
| `tests/` | Contiene pruebas de análisis, archivos, terminal y robustez. |
| `ejemplos/` | Contiene programas para probar el lenguaje. |

Las dependencias directas de `Cargo.toml` son `eframe`, para la ventana y los componentes de `egui`, y `regex`, para reconocer patrones de texto.

## 3. Flujo general: desde el texto hasta el resultado

```text
Usuario escribe código o se carga un archivo UTF-8
    -> Analizar / Ctrl+Enter / --analizar RUTA
    -> tokenizar: texto -> lista de tokens
    -> Parser::programa: validar estructura y declaraciones
    -> Resumen de tokens/variables o primer ErrorCompilacion
    -> Mostrar el resultado en la ventana o en la terminal
```

El análisis léxico separa las piezas. El análisis sintáctico comprueba cómo se ordenan. La validación semántica comprueba reglas de significado, como declarar una variable antes de usarla.

En este proyecto, la validación sintáctica y semántica ocurre dentro del mismo `Parser`; no son dos recorridos independientes. El resultado de la terminal es el diagnóstico del analizador; no es la salida de un programa de pseudocódigo.

## 4. Cómo comienza el programa: `main.rs`

La función `main` lee los argumentos de la terminal, sin incluir el nombre del ejecutable.

| Forma de inicio | Qué sucede |
| --- | --- |
| `cargo run --locked` | Abre el editor vacío. |
| `cargo run --locked -- --abrir ejemplos/anexo_a.txt` | Abre la ventana, carga ese archivo y lo analiza. Solo lo valida. |
| `cargo run --locked -- --analizar ejemplos/anexo_a.txt` | Carga y analiza el archivo desde la terminal, sin abrir la ventana. |
| `cargo run --locked -- --ayuda` | Muestra las opciones de uso. También acepta `--help`. |

El `--` de estos comandos separa las opciones de Cargo de los argumentos que recibe nuestra aplicación.

Los códigos de salida son `0` si la operación termina correctamente, `1` si el pseudocódigo tiene un error de análisis y `2` si hay un problema de archivo, argumentos o inicio de la ventana.

Cuando se abre la interfaz, `eframe::run_native` crea la ventana con un `Editor`. La ventana solicita inicialmente 1050 × 800 y tiene un tamaño mínimo de 720 × 600.

### Carga de archivos

`archivos::cargar` usa `std::fs::read_to_string`. Su resultado es el texto del archivo o un mensaje de error. Si el archivo no es UTF-8, lo indica específicamente. Un archivo inexistente también produce un mensaje, pero estos problemas de lectura no son errores E01–E16 del lenguaje.

## 5. Qué es un token

Un **token** es una pieza del texto que el analizador trata como una unidad: una palabra, un número o un símbolo, por ejemplo.

Considera esta sentencia:

```text
total = 10 + 2;
```

Produce seis tokens:

| Orden | Texto | Tipo |
| --- | --- | --- |
| 1 | `total` | `Palabra` |
| 2 | `=` | `Simbolo` |
| 3 | `10` | `Numero` |
| 4 | `+` | `Simbolo` |
| 5 | `2` | `Numero` |
| 6 | `;` | `Simbolo` |

Los espacios no son tokens. `total=10+2;` produce las mismas piezas. El punto y coma sí cuenta como token.

El tipo definido en Rust es:

```rust
pub struct Token {
    pub tipo: TipoToken,
    pub texto: String,
    pub linea: usize,
}
```

- `tipo`: categoría de la pieza.
- `texto`: contenido original, como `Declare`, `total` o `>=`.
- `linea`: línea del código fuente donde aparece, contando desde 1. Sirve para ubicar errores.

No se guarda la columna del token y no se agrega un token especial de fin de archivo. El parser detecta el final cuando ya no hay un elemento en la lista.

### Los cuatro tipos de token

| Tipo | Significado | Ejemplos |
| --- | --- | --- |
| `Palabra` | Identificador con forma válida o palabra reservada. | `total`, `Inicio`, `Entero`, `Lógico` |
| `Numero` | Número sin signo, con parte decimal opcional. | `0`, `25`, `3.14` |
| `Simbolo` | Puntuación u operador reconocido. | `,`, `;`, `(`, `)`, `+`, `=`, `<=` |
| `Invalido` | Fragmento agrupado que no cumple los patrones anteriores. | `_total`, `3abc`, `1.2.3`, `niño` |

`Palabra` no significa automáticamente “variable”. Por ejemplo, `Inicio` y `total` tienen esa misma categoría; el parser distingue su función según el texto y el lugar donde aparecen.

`Invalido` tampoco implica que `tokenizar` devuelva inmediatamente un error. Conserva esa pieza para que el parser pueda dar un diagnóstico según el contexto. En cambio, un carácter no permitido, como `@`, sí hace que `tokenizar` devuelva E01 directamente.

## 6. Cómo se generan los tokens: `lexico.rs`

La función `tokenizar(codigo)` devuelve `Result<Vec<Token>, ErrorCompilacion>`: una lista de tokens si pudo recorrer el texto o un error léxico si encontró un carácter no permitido.

Su recorrido es el siguiente:

1. Elimina un BOM Unicode inicial si lo hay. Es una marca que algunos archivos de texto incluyen al comienzo.
2. Crea un recorrido de caracteres con `peekable`, que permite consultar el siguiente carácter sin consumirlo.
3. Comienza con `linea = 1` y una lista de tokens vacía.
4. Si encuentra `\n`, aumenta la línea. Los demás espacios en blanco se ignoran.
5. Si encuentra un carácter alfanumérico, `_` o `.`, reúne todos los caracteres consecutivos de esas clases en un solo fragmento.
6. Clasifica ese fragmento usando los patrones de número, identificador y palabras reservadas.
7. Si encuentra un símbolo permitido, genera un token de símbolo. Puede unir dos caracteres para formar `<=`, `<>` o `>=`.
8. Si encuentra otro carácter, devuelve E01 y termina.
9. Si llega al final, devuelve la lista completa.

El agrupamiento conserva `3abc` como un solo token inválido: no lo separa artificialmente en el número `3` y la palabra `abc`.

### Patrón de identificadores

```text
^[A-Za-z][A-Za-z0-9_]*$
```

| Parte | Significado |
| --- | --- |
| `^` | Comienzo del texto. |
| `[A-Za-z]` | El primer carácter debe ser una letra ASCII. |
| `[A-Za-z0-9_]*` | Después puede haber cero o más letras ASCII, dígitos o guiones bajos. |
| `$` | Final del texto. Todo el nombre debe cumplir la regla. |

Son válidos `a`, `total2` y `mi_variable`. No son válidos `2total`, `_total`, `mi-variable` ni `año`. En `mi-variable`, el lexer separa `mi`, `-` y `variable`; no existe un identificador con guion.

`es_identificador` también descarta las palabras reservadas. Aunque `Inicio` cumple el patrón de letras, no puede usarse como nombre de variable.

### Patrón de números

```text
^[0-9]+(\.[0-9]+)?$
```

Admite uno o más dígitos y, opcionalmente, un punto seguido de uno o más dígitos. Acepta `12` y `12.5`; no acepta `.5`, `12.`, notación científica ni coma decimal como un único número.

En el código fuente, `-5` se separa en `-` y `5`. Como la gramática no admite operadores unarios, una asignación como `a = -5;` se rechaza. Se puede escribir `a = 0 - 5;`.

### Palabras reservadas

```text
Inicio Fin Declare Entero Real Cadena Fecha Lógico Logico
Si Entonces Sino FSi MQ Haga FMQ Para Hasta FPara Lea Escriba
```

El reconocimiento no distingue mayúsculas y minúsculas. Se conserva el texto original del token, pero las comparaciones de palabras se normalizan. Las variables también se buscan sin distinguir mayúsculas ASCII: `Total` y `total` identifican la misma variable.

Las expresiones regulares se guardan en `LazyLock`: se construyen cuando se usan por primera vez y se reutilizan en las llamadas siguientes.

## 7. El analizador: `sintactico::analizar`

La función `analizar(codigo)` hace tres cosas:

1. Llama a `tokenizar(codigo)`.
2. Crea un `Parser` y llama a `programa()`.
3. Si no hubo errores, devuelve `Resumen { tokens, variables }`.

El resumen cuenta todos los tokens producidos y las variables declaradas. Esos números no representan instrucciones ejecutadas ni valores calculados.

### Qué guarda el parser

| Campo | Para qué sirve |
| --- | --- |
| `tokens` | Referencia a la lista que debe revisar. |
| `posicion` | Índice del siguiente token por procesar. Empieza en 0. |
| `ultima_linea` | Línea de referencia para errores cuando se acaba la entrada. Se calcula con `codigo.lines().count().max(1)`. |
| `simbolos` | Tabla `BTreeMap` que relaciona cada nombre de variable normalizado con su tipo declarado. |
| `bloques` | Pila de estructuras abiertas: `Si`, `MQ` y `Para`. |

El parser no modifica el texto fuente. Avanza cambiando `posicion`.

### Funciones auxiliares de navegación

| Función | Qué hace |
| --- | --- |
| `actual()` | Consulta el token en `posicion`, si existe. |
| `linea()` | Obtiene la línea actual o la línea de referencia al final de la entrada. |
| `es(texto)` | Pregunta si el token actual coincide con el texto, sin consumirlo. |
| `aceptar(texto)` | Si coincide, avanza una posición y devuelve `true`; si no, no avanza. |
| `error(...)` | Construye un error usando la línea actual. |
| `identificador()` | Exige un nombre válido, consume el token y lo devuelve. |
| `declarada(token)` | Busca la variable en `simbolos`; devuelve su tipo o E14. |
| `terminar()` | Exige y consume `;`. Si falta, devuelve E05, normalmente en la línea del token anterior. |

## 8. La estructura completa: `programa()`

La forma general esperada es:

```text
Inicio
Declare ...;
Declare ...;
sentencias
Fin
```

`programa()` realiza este recorrido:

1. Exige `Inicio` como primer token; si falta, devuelve E03.
2. Procesa las declaraciones consecutivas del comienzo.
3. Procesa las demás sentencias hasta encontrar `Fin` o agotar los tokens.
4. Comprueba que no queden bloques abiertos; si queda alguno, devuelve E10.
5. Exige `Fin`; si falta, devuelve E04.
6. Comprueba que no haya contenido después de `Fin`; si lo hay, devuelve E04.

Los saltos de línea ayudan a informar errores, pero no separan sentencias. Por eso se puede escribir un programa en una sola línea si contiene todos sus delimitadores.

`Inicio` y `Fin` no llevan punto y coma en esta gramática. Las declaraciones, asignaciones y operaciones de entrada/salida sí lo requieren. Los cierres `FSi`, `FMQ` y `FPara` aceptan opcionalmente un `;`.

## 9. Declaraciones y tabla de símbolos

Ejemplo:

```text
Declare a, b Entero;
```

`declaracion()`:

1. Consume `Declare`.
2. Lee un identificador.
3. Rechaza nombres ya declarados o repetidos dentro de la misma lista.
4. Si hay coma, repite la lectura del siguiente identificador.
5. Lee el tipo: `Entero`, `Real`, `Cadena`, `Fecha` o `Lógico`/`Logico`.
6. Exige `;`.
7. Solo después de validar la declaración completa inserta sus variables en `simbolos`.

La tabla queda conceptualmente así:

| Nombre | Tipo |
| --- | --- |
| `a` | `Entero` |
| `b` | `Entero` |

Esta tabla del analizador guarda tipos, no valores. Declarar `a` no le asigna automáticamente cero.

No hay ámbitos locales para los bloques: las variables se declaran al comienzo del programa. Una declaración después de una asignación o dentro de un bloque produce E06.

## 10. Cómo se decide qué sentencia procesar

`sentencia()` mira el texto del token actual y selecciona una función:

| Token inicial | Acción del parser |
| --- | --- |
| `Inicio` | E03: no se puede abrir otra vez el programa. |
| `Declare` | E06: ya pasó la zona de declaraciones. |
| `Si` o `MQ` | `bloque_condicional()`. |
| `Para` | `bloque_para()`. |
| `Sino` | `sino()`. |
| `FSi`, `FMQ` o `FPara` | `cerrar()`. |
| `Lea` o `Escriba` | `entrada_salida()`. |
| Un identificador o un token `Invalido` | `asignacion()`, que también puede diagnosticar un nombre inválido. |
| Otro token | E07: sentencia no reconocida. |

## 11. Asignaciones y expresiones aritméticas

Para `a = (b + 2) * 3;`, `asignacion()`:

1. Lee el identificador `a`.
2. Exige `=`.
3. Comprueba que `a` esté declarada.
4. Valida la expresión con `expresion()`.
5. Exige `;`.

Todavía no calcula el resultado. La variable `b` también debe estar declarada, pero el análisis no exige que ya tenga un valor.

### Cómo valida `expresion()`

Utiliza una pila de paréntesis y una bandera `espera_operando`.

Un **operando** es un número o una variable. Un **operador** es `+`, `-`, `*` o `/`.

El estado inicial espera un operando. Después de un operando espera un operador. Después de un operador vuelve a esperar un operando.

Para `(b + 2) * 3`:

| Token | Qué comprueba o cambia |
| --- | --- |
| `(` | Guarda la línea en la pila de paréntesis; sigue esperando un operando. |
| `b` | Verifica que esté declarada; pasa a esperar un operador. |
| `+` | Pasa a esperar un operando. |
| `2` | Pasa a esperar un operador. |
| `)` | Retira una apertura de la pila; ya hay un operando completo. |
| `*` | Pasa a esperar un operando. |
| `3` | Pasa a esperar un operador. |
| `;` | Termina la lectura de la expresión; el punto y coma lo consume `terminar()`. |

Así detecta paréntesis desbalanceados, operandos seguidos, operadores sin operandos y expresiones vacías. No calcula valores ni aplica precedencia para obtener resultados; solo reconoce la estructura de la expresión.

Tiene una regla de diagnóstico para `a = 1 b = 2;`: al reconocer que comienza otra asignación, deja que `terminar()` informe E05 por el punto y coma que falta.

## 12. Entrada y salida: `Lea` y `Escriba`

Las formas admitidas son:

```text
Lea(a);
Escriba(a);
```

`entrada_salida()` consume el nombre de la operación, exige `(`, lee exactamente un identificador declarado, exige `)` y finalmente `;`.

`Escriba(10);`, `Escriba(a + b);` y `Lea(a, b);` no cumplen esta gramática. La forma admitida para representar una escritura es `Escriba(variable);`; aquí solo se comprueba su sintaxis.

El análisis no pide datos ni imprime valores. `Lea("10");` produce E01 por las comillas; `Lea(10);` produce E07 porque el argumento debe ser un identificador. `Lea(a);` se acepta si `a` está declarada.

## 13. Condiciones y estructuras de control

### Condiciones

`condicion()` exige exactamente:

```text
operando operador_relacional operando
```

Cada operando puede ser un número o una variable declarada. Los operadores permitidos son `>`, `<`, `<>`, `>=`, `<=` y `=`.

Por ejemplo, `a >= 10` es válido. `a + 1 > 10`, `(a > 10)` y `a > 10 y b < 5` no son condiciones admitidas.

El signo `=` significa asignación en `a = 10;` y comparación de igualdad en `Si a = 10 Entonces`. El contexto determina su función.

### `Si` y `MQ`

```text
Si a > 10 Entonces
    Escriba(a);
Sino
    a = 10;
FSi

MQ a < 20 Haga
    a = a + 1;
FMQ
```

`bloque_condicional()` lee la condición, exige `Entonces` para `Si` o `Haga` para `MQ` y agrega un `Bloque` a la pila. `componente()` es la función auxiliar que exige estas partes obligatorias; si faltan, devuelve E09.

El parser valida todas las sentencias de ambas ramas y de los cuerpos de los ciclos. No evalúa las condiciones ni selecciona una rama para ejecutarla.

### `Para`

```text
Para i = 1 Hasta n Haga
    Escriba(i);
FPara
```

`bloque_para()` exige, en orden, un contador declarado de tipo `Entero`, `=`, un índice inicial, `Hasta`, un índice final y `Haga`. Después abre el bloque en la pila.

`indice()` acepta un número sin punto decimal o una variable declarada de tipo `Entero`. `exigir_entero()` verifica el tipo declarado de las variables del encabezado. No se admiten expresiones como `Hasta n + 1` ni un literal como `1.0`, aunque matemáticamente sea entero.

### Por qué se utiliza una pila de bloques

Una pila permite agregar y retirar elementos por el mismo extremo. El último bloque abierto debe ser el primero en cerrarse.

```text
Si ... Entonces       pila: [Si]
    MQ ... Haga       pila: [Si, MQ]
    FMQ               pila: [Si]
FSi                   pila: []
```

Cada `Bloque` guarda su nombre, cierre esperado, línea de apertura y si ya tiene `Sino`.

`sino()` solo acepta un `Sino` si el bloque superior es un `Si` que aún no lo tiene. `cerrar()` comprueba que el cierre corresponda al bloque superior, lo retira y consume un `;` opcional.

Si se intenta cerrar el `Si` antes del `MQ`, se obtiene E11. Si llega `Fin` con un bloque abierto, se obtiene E10. No se necesita una llamada recursiva por cada nivel de anidamiento: la pila guarda ese estado.

## 14. Errores y propagación

`ErrorCompilacion` contiene `codigo`, `linea` y `mensaje`. Su representación es:

```text
Error [E14] Semántico - Línea 3: Variable 'a' no declarada.
```

| Código | Categoría | Causa general |
| --- | --- | --- |
| E01 | Léxico | Carácter no permitido. |
| E02 | Léxico | Identificador inválido. |
| E03 | Sintáctico | Falta `Inicio` o aparece otra vez. |
| E04 | Sintáctico | Falta `Fin` o hay contenido después. |
| E05 | Sintáctico | Falta `;` en una sentencia que lo requiere. |
| E06 | Sintáctico | Declaración incorrecta o fuera de lugar. |
| E07 | Sintáctico | Sentencia, asignación o entrada/salida incorrecta. |
| E08 | Sintáctico | Condición incorrecta. |
| E09 | Sintáctico | Falta un componente obligatorio del encabezado de un bloque. |
| E10 | Sintáctico | Bloque abierto sin cierre. |
| E11 | Sintáctico | Cierre incorrecto o `Sino` fuera de lugar/repetido. |
| E12 | Sintáctico | Paréntesis desbalanceados en una expresión. |
| E13 | Sintáctico | Expresión aritmética incorrecta. |
| E14 | Semántico | Variable no declarada. |
| E15 | Semántico | Variable declarada más de una vez. |
| E16 | Semántico | Contador o índices de `Para` incumplen la regla de enteros. |

La categoría del error depende del código asignado. Por eso E02 se presenta como léxico aunque lo detecte una función del parser.

Rust representa éxito o fallo con `Result`: `Ok(valor)` o `Err(error)`. El operador `?` continúa si hay éxito y devuelve inmediatamente el error si hay fallo. Por ejemplo, si falla `self.expresion()?`, la asignación no sigue hasta `terminar()`.

El analizador devuelve el primer error encontrado por su recorrido, no una lista de todos los errores. Primero tokeniza todo el texto; por eso un carácter no permitido al final puede reportarse antes que un problema sintáctico situado al comienzo.

## 15. Ejemplo completo de análisis, token por token

```text
Inicio
Declare a Entero;
a = 2 + 3 * 4;
Escriba(a);
Fin
```

La lista contiene **19 tokens**:

| Índice en `Vec` | Texto | Tipo | Línea |
| --- | --- | --- | --- |
| 0 | `Inicio` | Palabra | 1 |
| 1 | `Declare` | Palabra | 2 |
| 2 | `a` | Palabra | 2 |
| 3 | `Entero` | Palabra | 2 |
| 4 | `;` | Simbolo | 2 |
| 5 | `a` | Palabra | 3 |
| 6 | `=` | Simbolo | 3 |
| 7 | `2` | Numero | 3 |
| 8 | `+` | Simbolo | 3 |
| 9 | `3` | Numero | 3 |
| 10 | `*` | Simbolo | 3 |
| 11 | `4` | Numero | 3 |
| 12 | `;` | Simbolo | 3 |
| 13 | `Escriba` | Palabra | 4 |
| 14 | `(` | Simbolo | 4 |
| 15 | `a` | Palabra | 4 |
| 16 | `)` | Simbolo | 4 |
| 17 | `;` | Simbolo | 4 |
| 18 | `Fin` | Palabra | 5 |

El avance del parser es:

1. Consume `Inicio`: `posicion` pasa de 0 a 1.
2. Consume la declaración completa: llega a 5 y registra `a -> Entero`.
3. Consume la asignación: verifica la variable, expresión y `;`; llega a 13.
4. Consume `Escriba(a);`: verifica que `a` esté declarada; llega a 18.
5. Confirma que no hay bloques abiertos y consume `Fin`: llega a 19.
6. Como ya no hay tokens, devuelve `Resumen { tokens: 19, variables: 1 }`.

Al pulsar **Analizar**, se muestra que el programa es correcto y tiene 19 tokens y una variable. No se calcula ni se imprime el valor de `a`.

## 16. Cómo conecta todo la interfaz

`Editor` conserva el código, la ruta de archivo, el resultado del análisis, los avisos de carga y el estado de la ayuda.

| Función o evento | Efecto |
| --- | --- |
| `con_archivo()` | Prepara un editor con una ruta y llama a `abrir()`. |
| `abrir()` | Carga el archivo y analiza el texto si la carga es correcta. |
| `analizar()` | Guarda el resultado de `sintactico::analizar`. |
| `dibujar()` | Procesa Ctrl+Enter y dibuja el editor y los resultados. |
| `eframe::App::ui()` | Entrada que usa la biblioteca gráfica para llamar a `dibujar()`. |
| Editar el código | Borra el resultado del análisis anterior. |

El botón **Analizar** y **Ctrl+Enter** realizan la misma validación. No hay botón Ejecutar, atajo F5, botón Detener, consola ni campo para aportar datos.

En `interfaz.rs`, los controles de ruta y **Abrir archivo**, el control de ayuda y el botón de copiar código están comentados. La carga inicial por `--abrir` sí está implementada.

## 17. Qué se detecta y qué queda fuera del análisis

### Variable no declarada

```text
Inicio
a = 10;
Fin
```

El lexer genera los tokens, pero `declarada()` no encuentra `a`. El análisis devuelve E14.

### Variable declarada sin valor

```text
Inicio
Declare a Entero;
Escriba(a);
Fin
```

El análisis es correcto porque `a` está declarada. No se comprueba inicialización ni se consulta su valor. No se imprime nada.

### División por cero

```text
Inicio
Declare a Real;
a = 10 / 0;
Fin
```

El análisis es correcto: la expresión tiene la forma admitida. La aplicación no calcula la división, por lo que no diagnostica división por cero.

## 18. Qué significan MQ, FMQ y las demás palabras

Estos nombres son palabras reservadas del pseudocódigo. Los significados describen la intención de las instrucciones; nuestra aplicación solo valida su escritura y estructura.

| Palabra | Significado | Función en el lenguaje |
| --- | --- | --- |
| `Inicio` | Inicio del programa. | Abre el programa; debe ser el primer token. |
| `Fin` | Fin del programa. | Cierra el programa; no admite contenido después. |
| `Declare` | Declarar variables. | Introduce nombres y su tipo antes de las demás sentencias. |
| `Entero` | Tipo de número sin parte decimal. | Es un tipo declarado; se exige para las variables del encabezado de `Para`. |
| `Real` | Tipo de número que puede tener decimales. | Es uno de los tipos de declaración admitidos. |
| `Cadena` | Tipo de texto. | Permite declarar una variable de texto; no agrega literales entre comillas a esta gramática. |
| `Fecha` | Tipo de fecha. | Permite declarar una variable; no implica validar fechas concretas. |
| `Lógico` / `Logico` | Tipo de verdad o falsedad. | Son dos formas admitidas del mismo tipo; no agregan literales booleanos a las expresiones. |
| `Si` | Si se cumple una condición. | Abre una estructura condicional. |
| `Entonces` | Entonces realizar el bloque. | Separa la condición de `Si` de su cuerpo. |
| `Sino` | En caso contrario. | Introduce la alternativa de un `Si`; es opcional y solo puede aparecer una vez por bloque. |
| `FSi` | Fin de Si. | Cierra el último bloque `Si` abierto. |
| `MQ` | Mientras Que. | Abre un ciclo con condición. |
| `Haga` | Hacer el cuerpo del bloque. | Termina el encabezado de `MQ` o de `Para`. |
| `FMQ` | Fin de Mientras Que. | Cierra el último bloque `MQ` abierto. |
| `Para` | Repetición con contador. | Abre el bloque con contador, inicio y límite. |
| `Hasta` | Hasta el límite indicado. | Separa el índice inicial del final en `Para`. |
| `FPara` | Fin de Para. | Cierra el último bloque `Para` abierto. |
| `Lea` | Leer un dato. | Su forma válida es `Lea(variable);`; el analizador no solicita el dato. |
| `Escriba` | Escribir un dato. | Su forma válida es `Escriba(variable);`; el analizador no imprime el dato. |

La `F` de `FSi`, `FMQ` y `FPara` significa **Fin**. Cada cierre debe coincidir con el bloque abierto más reciente.

```text
Inicio
Declare a Entero;
a = 0;
MQ a < 3 Haga
    a = a + 1;
FMQ
Fin
```

Se lee: «Mientras que `a` sea menor que 3, haga el cuerpo; fin de mientras que». El analizador comprueba la declaración, condición, `Haga`, asignación y `FMQ`; no repite el bloque ni cambia el valor de `a`.

## 19. Conceptos de Rust que aparecen en el recorrido

| Construcción | Cómo entenderla aquí |
| --- | --- |
| `struct` | Agrupa datos relacionados, como los campos de `Token` o `Parser`. |
| `enum` | Define alternativas posibles, como los tipos de token o códigos de error. |
| `impl` | Agrupa funciones asociadas a un tipo y sus métodos. |
| `Vec<T>` | Lista de elementos; también se usa como pila con `push`, `pop` y `last`. |
| `BTreeMap` | Tabla ordenada por clave. Permite buscar una variable por su nombre. |
| `Option<T>` | Un valor opcional: `Some(valor)` o `None`. |
| `Result<T, E>` | Éxito `Ok(T)` o error `Err(E)`. |
| `?` | Propaga un error al llamador y detiene ese recorrido. |
| `match` | Elige un comportamiento según el valor o variante encontrado. |
| `&` | Referencia a un dato existente, sin tomar su propiedad. |
| `&mut` | Referencia que permite modificar un dato. |
| `Parser<'a>` | El parser toma prestada la lista de tokens; la duración `'a` vincula esa referencia con su uso. |
| `clone()` / `cloned()` | Crea una copia de un dato cuando se necesita poseerlo por separado. |
| `expect(...)` | Extrae un valor que el código supone existente; si esa suposición falla, provoca un pánico. No es el mecanismo normal de diagnóstico del pseudocódigo. |

## 20. Dónde continuar la lectura y cómo comprobarlo

Para seguir el recorrido en el código, abre los archivos en este orden: `main.rs`, `lexico.rs`, `sintactico.rs`, `errores.rs` e `interfaz.rs`.

Para ejecutar las pruebas existentes desde la raíz:

```bash
cargo test --locked
```

`tests/lexico.rs` cubre reglas léxicas; `tests/aceptacion.rs` valida casos del lenguaje; `tests/robustez.rs` revisa expresiones, diagnósticos y anidamiento; `tests/archivos.rs` y `tests/cli.rs` cubren carga y terminal. También hay pruebas del editor dentro de `src/interfaz.rs`.

Para practicar con programas existentes y consultar los comandos de uso, revisa el [`README.md`](../README.md).
