# Simulador de compilador en Rust

Aplicación de escritorio para validar el pseudocódigo definido en `context/Especificación de Requisitos – II Proyecto Simulación de un Compilador.md`. Realiza análisis léxico, sintáctico y comprobaciones semánticas de declaración y del tipo de las variables del `Para`. El botón Analizar conserva ese comportamiento. Como ampliación solicitada, el botón Ejecutar interpreta el programa y muestra en la consola los valores de Escriba; Lea pide datos en la misma ventana. No genera código objeto.

## Iniciar

Requiere Rust estable con Cargo, un enlazador de C y una sesión gráfica. El proyecto utiliza la edición 2024; la versión mínima efectiva depende también de eframe. La versión de Rust usada en la verificación queda registrada en `docs/evidencias/verificacion.txt`. Conservar `Cargo.lock` para reproducir dependencias.

```bash
cargo run --locked
```

Para comenzar con el ejemplo completo del documento:

```bash
cargo run --locked -- --abrir ejemplos/anexo_a.txt
```

La primera compilación descarga y compila las dependencias. En Fedora se necesita el compilador/enlazador de desarrollo de C (por ejemplo `gcc`) y las bibliotecas gráficas de la sesión Wayland/X11. Si aparece un error de dependencia del sistema, instalar el paquete correspondiente antes de la sustentación. El ejecutable debe compilarse para el sistema donde se presentará; no se ha verificado Windows ni macOS.

## Usar la ventana

1. Escribir o pegar el pseudocódigo en el editor con números de línea.
2. Pulsar **Analizar** o **Ctrl+Enter** para validar; pulsar **Ejecutar** o **F5** para obtener la salida del programa.
3. Si es válido, aparece **El programa es sintácticamente correcto.**
4. Si es inválido, aparece su código E01-E16, tipo, línea absoluta del archivo y causa. Corregirlo y volver a analizar.
5. Para cargar un archivo, escribir su ruta en el campo superior y pulsar **Abrir archivo**. Se carga y analiza automáticamente. Las rutas relativas parten del directorio desde donde se inició el programa; también se admiten rutas absolutas.

Los archivos deben ser texto UTF-8. Se aceptan BOM inicial, finales de línea LF y CRLF. Si un archivo no existe o no es UTF-8, el editor conserva el texto anterior y muestra el problema de carga. **Abrir un archivo existente reemplaza el contenido del editor.** Usar **Copiar código** para conservarlo en otro editor: esta aplicación no guarda automáticamente. Tab permite recorrer los controles.

La opción **Reglas del lenguaje** muestra una referencia breve. Al modificar el código, el resultado anterior desaparece para evitar confundir una validación antigua con el texto nuevo. El panel de resultados y consola se puede redimensionar y desplazar. Al modificar el código también se descarta la ejecución anterior.


## Consola: mostrar el valor de una variable

Ejecuta este programa con **Ejecutar** o **F5**:

```text
Inicio
Declare a Entero;
a = 10;
Escriba(a);
Fin
```

La consola muestra `10`. La asignación guarda el valor y `Escriba(a)` lo imprime; el nombre de la variable no se imprime como texto literal. El ejemplo está en `ejemplos/consola_10.txt`:

```bash
cargo run --locked -- --abrir ejemplos/consola_10.txt
```

Con `Lea(a);`, la ejecución se pausa: escribe el dato en **Entrada** y pulsa **Enviar dato** o Enter. Si el dato no sirve para el tipo declarado, puedes corregirlo sin reiniciar. `ejemplos/consola_entrada.txt` suma dos entradas y `ejemplos/consola_bloques.txt` muestra salidas de ciclos y condiciones.

**Detener** interrumpe una ejecución o una espera de entrada. Cada nueva ejecución comienza con variables sin inicializar y una consola vacía. Un ciclo que no termina se detiene al llegar a 100000 instrucciones; hay un límite de 5000 líneas de salida. Los detalles y las decisiones numéricas están en `docs/ejecucion.md`. Analizar y cargar archivos nunca ejecutan el programa automáticamente.

## Lenguaje de entrada

```text
Inicio
Declare a, b Entero;
Declare x Real;
Lea(a);
b = (a + 2) * 3;
Si b >= 5 Entonces
    Escriba(b);
Sino
    b = 0;
FSi;
MQ a < 10 Haga
    a = a + 1;
FMQ
Para a = 1 Hasta 5 Haga
    Escriba(a);
FPara
Fin
```

- `Inicio` y `Fin` aparecen exactamente una vez y no llevan `;`. No hay contenido fuera de ellos.
- Las declaraciones van primero y fuera de los bloques: `Declare a, b Entero;`.
- Tipos: `Entero`, `Real`, `Cadena`, `Fecha`, `Lógico` y su alias `Logico`.
- Regex visible en `src/lexico.rs`: `^[A-Za-z][A-Za-z0-9_]*$`. No se admiten nombres que empiecen por `_` o un dígito, letras con tilde, ñ ni palabras reservadas.
- No se distinguen mayúsculas de minúsculas en palabras reservadas ni nombres. `a` y `A` son la misma variable.
- Todas las variables usadas deben estar declaradas; no se permiten declaraciones repetidas.
- Expresiones: identificadores, enteros, reales de la forma `3.14`, `+ - * /` y paréntesis. No hay signos unarios: para expresar un negativo puede usarse `0 - 5`.
- Condiciones: exactamente un identificador o número a cada lado de `> < <> >= <= =`. No hay operaciones aritméticas ni paréntesis en las condiciones.
- `Para variable = inicio Hasta fin Haga`: variable declarada `Entero`, índices variables `Entero` o literales enteros.
- `Lea(variable);` y `Escriba(variable);` reciben una sola variable declarada; no reciben expresiones ni cadenas.
- Las sentencias terminan en `;`. Los encabezados de bloques, `Inicio`, `Fin` y `Sino` no llevan `;`. Los cierres `FSi`, `FMQ` y `FPara` aceptan un `;` opcional.
- Se admite cualquier combinación de bloques anidados, vacíos o no. Solo un `Sino` por `Si`.
- Una sentencia puede ocupar varias líneas y una línea contener varias sentencias. Los saltos de línea solo sirven para ubicar errores.
- No hay comentarios, literales de cadena, booleanos, fechas, potencias, operadores lógicos ni `FinSi`: la gramática no los incluye. Los cinco tipos de declaración no amplían la gramática de expresiones.

## Política de errores

Se reporta **el primer error encontrado por las etapas de análisis**, conforme al mínimo de RF-29. El lexer se ejecuta antes que el parser: un carácter prohibido en cualquier parte produce E01 antes de analizar la estructura. Los nombres mal formados se conservan como tokens para que la estructura determine E02 o E13; así P14 detecta el operando ausente antes de `8M`.

Los números de línea empiezan en 1 y se refieren al programa completo. En los ejemplos de bloques de la especificación, sumar las líneas del envoltorio `Inicio` y declaraciones cuando se agregan al archivo. Un bloque sin cierre informa su apertura; un cierre huérfano informa la línea del cierre. Un `;` ausente se informa en la línea del último elemento consumido de la sentencia. Un paréntesis sin cierre informa su apertura.

| Código | Tipo | Causa |
|---|---|---|
| E01 | Léxico | Carácter no permitido |
| E02 | Léxico | Identificador inválido o reservado |
| E03 | Sintáctico | Inicio ausente, repetido o mal ubicado |
| E04 | Sintáctico | Fin ausente o contenido posterior |
| E05 | Sintáctico | Falta punto y coma |
| E06 | Sintáctico | Declaración mal formada o fuera del inicio |
| E07 | Sintáctico | Asignación, Lea, Escriba o sentencia inválida |
| E08 | Sintáctico | Condición inválida |
| E09 | Sintáctico | Componente obligatorio ausente en encabezado de bloque |
| E10 | Sintáctico | Bloque sin cierre |
| E11 | Sintáctico | Cierre fuera de orden, huérfano o Sino inválido |
| E12 | Sintáctico | Paréntesis desbalanceados |
| E13 | Sintáctico | Expresión aritmética mal formada |
| E14 | Semántico | Variable no declarada |
| E15 | Semántico | Declaración duplicada |
| E16 | Semántico | Variable o índice de Para no entero |

Estas reglas corresponden al análisis estático. Ejecutar puede detectar además división por cero, variables sin inicializar y operaciones con valores no numéricos; son errores de ejecución, sin alterar el catálogo E01-E16.

La prioridad entre problemas simultáneos es la del recorrido. No se garantiza detectar todos los errores en una pasada. El tipo declarado no se compara con los operandos de una asignación; tampoco se comprueba inicialización ni división por cero, porque Analizar no evalúa valores. Ejecutar sí evalúa y puede informar esos problemas.

## Organización y lectura del código

| Archivo | Responsabilidad |
|---|---|
| `src/main.rs` | Arranque de ventana, argumentos y códigos de salida |
| `src/lib.rs` | Expone el motor sin depender del editor |
| `src/interfaz.rs` | Estado del editor, controles, atajo y resultados |
| `src/archivos.rs` | Carga UTF-8 y errores de E/S |
| `src/lexico.rs` | Tokens, regex, palabras reservadas, operadores y líneas |
| `src/sintactico.rs` | Gramática, tabla de símbolos y pilas de bloques/paréntesis |
| `src/ejecucion.rs` | Intérprete por turnos, valores, entrada y salida, expresiones y saltos de control |
| `src/errores.rs` | Catálogo E01-E16 y formato en español |
| `tests/` | Aceptación, léxico, archivos, terminal y robustez |
| `ejemplos/` | Programa del anexo y errores de demostración |
| `docs/` | Informe, matriz y evidencias reproducibles |

Para estudiar: comenzar por `Token` y `tokenizar`, continuar por `Parser::programa`, `declaracion`, `asignacion` y `expresion`, después los bloques y finalmente `Editor`. Los bloques y los paréntesis usan pilas explícitas: el análisis no consume una llamada recursiva por nivel de anidamiento. La tabla `BTreeMap` guarda nombres ASCII en minúscula con su tipo. Las regex se compilan una sola vez con `LazyLock`.

## Terminal y verificaciones

```bash
cargo run --locked -- --analizar ejemplos/anexo_a.txt
cargo run --locked -- --analizar ejemplos/error_sintactico.txt
cargo run --locked -- --ayuda
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
cargo build --locked
python3 scripts/verificar_casos.py
```

El análisis por terminal devuelve `0` para programa correcto, `1` para pseudocódigo inválido y `2` para problemas de archivo, argumentos o inicio de la ventana. La segunda orden de ejemplo devuelve `1` deliberadamente. `--analizar` funciona sin sesión gráfica.

Después de descargar las dependencias se pueden agregar `--offline` a las órdenes de Cargo. El binario de desarrollo queda en `target/debug/compilador`; ejecutar `cargo build --release --locked` si se desea un binario optimizado para otra presentación. La verificación entregada corresponde al binario de desarrollo.

## Documentación

- `docs/ejecucion.md`: manual de la consola, semántica de ejecución y ejemplos.
- `docs/informe_tecnico.md`: contenido editable del informe.
- `output/pdf/informe_tecnico.pdf`: informe de 9 páginas de contenido, sin incluir un listado completo del código fuente.
- `docs/matriz_requisitos.md`: trazabilidad de RF-01 a RF-29 y RNF-01 a RNF-08.
- `docs/evidencias/casos.md`: resultados reales de los casos de aceptación y de las siete funcionalidades del informe.
- `docs/evidencias/casos.json`: las mismas ejecuciones en formato estructurado.
- `docs/evidencias/verificacion.txt`: herramientas y salida de las comprobaciones.
- `docs/evidencias/`: capturas de la aplicación real y fragmentos del código fuente.

El informe documenta la implementación realizada con asistencia de IA; no atribuye autoría exclusiva al estudiante ni inventa experiencias personales, nombres de integrantes o resultados. Antes de presentarlo, revisar sus datos académicos y estudiar el código para la sustentación.

## Regenerar el informe

Las herramientas de documentación no son necesarias para usar la aplicación. Para regenerar el PDF y las imágenes de código:

```bash
python3 -m venv /tmp/compilador-docs
/tmp/compilador-docs/bin/pip install reportlab pillow pypdf
python3 scripts/verificar_casos.py
/tmp/compilador-docs/bin/python scripts/generar_informe.py
```

El script exige las capturas PNG entregadas en `docs/evidencias/`. Para repetir una captura nativa con una sesión gráfica activa:

```bash
cargo run --locked --example capturar -- ejemplos/anexo_a.txt /tmp/correcto.ppm
```

Ese ejemplo abre solo el simulador, captura su propio contenido y se cierra. Convertir el PPM a PNG con Pillow si se desea reemplazar la evidencia. No captura el escritorio.

## Entrega preparada

El paquete `entrega/proyecto_compilador.zip` incluye el código, las pruebas, los ejemplos, la documentación y un ejecutable Linux x86_64 derivado de la compilación de desarrollo, sin símbolos de depuración. Tras extraerlo y situarse en su carpeta:

```bash
./compilador-linux-x86_64
./compilador-linux-x86_64 --abrir ejemplos/anexo_a.txt
```

En este repositorio, ese ejecutable está en `entrega/compilador-linux-x86_64`. Fue probado en el Fedora actual y necesita las bibliotecas gráficas del sistema; no es un paquete universal ni una compilación optimizada de release. El paquete no incluye la carpeta target ni el historial Git. Sus sumas de verificación quedan en `entrega/SHA256SUMS.txt`.

Para repetir toda la verificación y generar el registro: `python3 scripts/verificar_proyecto.py`. Para editar el contenido base del informe, modificar `docs/informe_base.md`; el generador resuelve los casos observados y produce `docs/informe_tecnico.md` y el PDF. Las modificaciones directas al Markdown generado se sobrescriben al regenerar.
