# Fundación Universitaria Tecnológico Comfenalco

# Especificación de Requisitos – II Proyecto: Simulación de un Compilador

**Profesor: Ing. Carlos García Castro Electiva III**

## 1. Introducción

Este documento define qué debe hacer la aplicación «Simulador de compilador» que cada grupo desarrollará en el II Proyecto de Autómatas, Gramáticas y Lenguajes, y con qué criterios se verificará que lo hace bien.

### 1.1 Propósito

Establecer, sin ambigüedades, los requisitos del analizador sintáctico de pseudocódigo que los estudiantes deben construir. Cada requisito tiene un identificador (RF-xx, RNF-xx) para poder citarlo en el informe técnico, en las pruebas y en la sustentación.

### 1.2 Alcance

La aplicación recibe un programa escrito en pseudocódigo, lo analiza y responde si es sintácticamente correcto. Si no lo es, informa cada error con su tipo y la línea donde ocurre.

| Dentro del alcance | Fuera del alcance |
| --- | --- |
| Validar la estructura del programa (Inicio / Fin) | Ejecutar el pseudocódigo |
| Validar declaraciones de variables | Generar código objeto o intermedio |
| Validar asignaciones y paréntesis en expresiones | Verificar la compatibilidad de tipos en asignaciones |
| Validar los bloques Si, MQ y Para y su cierre | Optimizar el programa |
| Detectar variables no declaradas | Interpretar el valor de las expresiones |
| Reportar errores con tipo y línea |  |

### 1.3 Definiciones

| Término | Definición |
| --- | --- |
| Pseudocódigo | Lenguaje de entrada definido en la sección 3. |
| Sentencia | Instrucción completa del programa: declaración, asignación, lectura o escritura. |
| Bloque | Estructura de control con apertura y cierre: Si-Fsi, MQ-FMQ o Para-FPara. |
| Primitiva | Palabra reservada con significado propio en el lenguaje (Si, MQ, Para, Lea, Escriba, FSi, FMQ, FPara). |
| Identificador | Nombre de una variable, formado según la sección 3.2. |
| Condición | Comparación entre dos operandos con un operador relacional. |
| Expresión | Combinación de identificadores, números, operadores aritméticos y paréntesis. |
| Error sintáctico | Incumplimiento de la estructura que exige la gramática (sección 5). |

## 2. Descripción general

La aplicación hace las veces de un analizador sintáctico: lee pseudocódigo, lo valida contra las reglas de este documento y reporta los errores que encuentre. No ejecuta el programa.

### 2.1 Funcionamiento esperado

1. El usuario ingresa el pseudocódigo, escribiéndolo o cargándolo desde un archivo de texto.
2. La aplicación lo analiza línea por línea.
3. Si no hay errores, informa que el programa es sintácticamente correcto.
4. Si hay errores, muestra cada uno con su tipo (sección 6) y el número de la línea donde ocurre.

### 2.2 Usuarios

El usuario de la aplicación es cualquier persona que escriba pseudocódigo para validarlo. En la sustentación, el docente y los compañeros actúan como usuarios y pueden ingresar programas de prueba propios.

### 2.3 Entorno de desarrollo

Cada grupo elige libremente el lenguaje de programación. El grupo es responsable de que la aplicación funcione en el equipo donde se sustente y de instalar las librerías que necesite.

### 2.4 Restricciones y supuestos

* Las palabras reservadas y los identificadores no distinguen mayúsculas de minúsculas (Si, si y SI son equivalentes), como se ve en el ejemplo del enunciado.
* Los espacios en blanco y las tabulaciones entre elementos no cambian el significado; una sentencia puede ocupar una o varias líneas, y una línea puede tener varias sentencias.
* Los bloques de distinto tipo pueden anidarse entre sí (por ejemplo, un MQ dentro de un Si).
* Las declaraciones deben aparecer antes de cualquier otra sentencia, tal como en el ejemplo del enunciado.
* No se verifica la compatibilidad de tipos entre una variable y el valor que se le asigna; solo se exige el tipo Entero en las variables de un Para.
* Los puntos de la especificación que el enunciado deja abiertos se resolvieron aquí con el criterio más simple; las decisiones están marcadas como «Decisión» y se listan en el Anexo A.

## 3. Definición del lenguaje de pseudocódigo

El pseudocódigo que la aplicación debe aceptar usa un conjunto fijo de palabras reservadas, cinco tipos de datos y reglas simples para nombres, números y operadores.

### 3.1 Palabras reservadas

| Categoría | Palabras |
| --- | --- |
| Estructura del programa | Inicio, Fin, Declare |
| Tipos de datos | Entero, Real, Cadena, Fecha, Lógico |
| Bloque condicional | Si, Entonces, Sino, FSi |
| Bloque repetitivo con condición | MQ, Haga, FMQ |
| Bloque repetitivo con contador | Para, Hasta, Haga, FPara |
| Entrada y salida | Lea, Escriba |

Decisión: Lógico también se acepta escrito sin tilde (Logico). Una palabra reservada no puede usarse como identificador.

### 3.2 Identificadores

Un identificador es válido si cumple estas tres condiciones a la vez: empieza por una letra, después tiene cero o más letras o dígitos, y el único símbolo especial permitido es el guion bajo \_. La aplicación debe validarlo con una expresión regular, que el informe técnico debe mostrar.

^[A-Za-z][A-Za-z0-9\_]\*$

Decisión: «letra» significa A–Z o a–z; no se aceptan ñ ni letras con tilde.

| Válidos | Inválidos | Motivo |
| --- | --- | --- |
| a, b1 | 1x | Empieza por un número |
| x1y, x\_3 | x&1 | Contiene el carácter & |
| x1234567890, a123yx0 | a 1 | Contiene un espacio en blanco |

### 3.3 Números y operadores

| Elemento | Forma aceptada |
| --- | --- |
| Número entero | Uno o más dígitos: 5, 120 |
| Número real | Dígitos, punto y dígitos: 3.14 |
| Operadores aritméticos | +, -, \*, / |
| Operadores relacionales | >, <, <>, >=, <=, = |
| Asignación | = |
| Agrupación | ( y ) |

### 3.4 Terminador de sentencias

Toda sentencia termina en punto y coma (;). No llevan punto y coma las líneas de encabezado de los bloques (Si … Entonces, MQ … Haga, Para … Haga), ni Inicio, Fin y Sino.

Decisión: las líneas de cierre FSi, FMQ y FPara se aceptan con o sin punto y coma final, porque el enunciado las escribe de ambas formas.

## 4. Requisitos funcionales

La aplicación debe cumplir los 29 requisitos siguientes. La columna «Origen» indica el numeral del enunciado del que proviene el requisito; «Decisión» marca los puntos que el enunciado dejaba abiertos.

### 4.1 Estructura del programa

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-01 | El programa debe comenzar con Inicio y terminar con Fin, cada uno una sola vez. | Enunciado 1 |
| RF-02 | No puede haber sentencias antes de Inicio ni después de Fin. | Decisión |
| RF-03 | Toda sentencia termina en ;, salvo los encabezados de bloque, Inicio, Fin y Sino (sección 3.4). | Enunciado 1 |

### 4.2 Declaración de variables

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-04 | Una declaración tiene la forma Declare ListaVariables TipoDeDato;. | Enunciado 2 |
| RF-05 | ListaVariables es un identificador o varios separados por coma; no se admite coma al inicio, al final ni dos seguidas. | Enunciado 2b |
| RF-06 | Cada identificador se valida con una expresión regular que cumpla la sección 3.2. | Enunciado 2d–2h |
| RF-07 | El tipo debe ser Entero, Real, Cadena, Fecha o Lógico. | Observaciones |
| RF-08 | Un identificador no puede declararse más de una vez. | Decisión |
| RF-09 | Todo identificador usado en una asignación, condición, Lea, Escriba o Para debe haberse declarado antes; si no, se reporta variable no declarada. | Informe, punto 3 |

### 4.3 Asignaciones y expresiones

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-10 | Una asignación tiene la forma identificador = expresión; y el identificador debe estar declarado. | Enunciado 3d |
| RF-11 | Una expresión está bien formada cuando cada operador (+, -, \*, /) tiene un operando a cada lado. Son inválidas A = ( b - ) 8M y x = 5 +;. | Enunciado 6 |
| RF-12 | En toda expresión hay tantos ( como ), y cada ) cierra un ( abierto antes. Son válidas X = (5 + y); y A = 5 \* (8 + x \* (c + b));; son inválidas x = ( 5 + x; y Y = 8 + 3 \* ( b+c));. | Enunciado 6 |

### 4.4 Bloque Si-Entonces-Sino-FSi

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-13 | Forma: Si condición Entonces sentencias, Sino y sentencias (opcional), FSi. Son obligatorios Si, Entonces y FSi; si falta uno, el bloque es inválido y el error nombra el componente ausente. | Enunciado 3a–3b |
| RF-14 | Sino es opcional, pero cada Si admite como máximo uno. Un Sino fuera de un Si es error. | Enunciado 3b |
| RF-15 | Cada Si debe tener su FSi: la cantidad de aperturas y cierres debe coincidir y emparejarse por anidamiento (el último bloque abierto es el primero en cerrarse). | Enunciado 3c, análisis a |

### 4.5 Bloque MQ-Haga-FMQ

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-16 | Forma: MQ condición Haga sentencias, FMQ. Son obligatorios MQ, Haga y FMQ; con hga en lugar de haga el bloque es inválido. | Enunciado 4, análisis b |
| RF-17 | Cada MQ debe tener su FMQ, con la misma regla de emparejamiento del RF-15. | Enunciado 4c |

### 4.6 Bloque Para-FPara

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-18 | Forma: Para var = ind1 Hasta ind2 Haga sentencias, FPara. Son obligatorios Para, =, Hasta, Haga y FPara; sin Hasta el bloque es inválido. | Enunciado 5, análisis b |
| RF-19 | var debe ser una variable declarada como Entero; ind1 e ind2 deben ser variables declaradas como Entero o números enteros. | Enunciado 5b y Decisión |
| RF-20 | Cada Para debe tener su FPara, con la misma regla de emparejamiento del RF-15. | Enunciado 5c |

### 4.7 Reglas comunes a los bloques

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-21 | Una condición compara dos operandos (identificador o número) con exactamente un operador entre >, <, <>, >=, <=, =. | Enunciado 3b, 4b |
| RF-22 | Dentro de un bloque puede haber cero o más asignaciones, lecturas, escrituras u otros bloques, del mismo tipo o de otro. | Enunciado 3d, 4d, 5d y Decisión |
| RF-23 | Un cierre sin apertura (FSi, FMQ o FPara sobrante) es error. | Enunciado, análisis a |

### 4.8 Entrada y salida

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-24 | Lea(identificador); con el identificador declarado. | Ejemplo del enunciado |
| RF-25 | Escriba(identificador); con el identificador declarado. Se admiten espacios antes del paréntesis, como en Escriba (b);. | Ejemplo del enunciado |

### 4.9 Reporte de resultados

| ID | Requisito | Origen |
| --- | --- | --- |
| RF-26 | Cada error se reporta con su tipo explícito (sección 6) y el número de la línea donde ocurre. | Enunciado, análisis |
| RF-27 | Para un bloque sin cierre, la línea reportada es la de su apertura; para un cierre sin apertura, la del cierre. | Decisión |
| RF-28 | Si el programa no tiene errores, la aplicación lo indica con un mensaje explícito. | Decisión |
| RF-29 | La aplicación debe reportar al menos el primer error. Es recomendable que siga analizando y reporte todos los que pueda detectar. | Decisión |

## 5. Gramática formal del pseudocódigo

La gramática siguiente, en notación BNF, resume todo lo que la aplicación debe aceptar. Lo que no se pueda derivar de ella es un error. Las llaves { } indican cero o más repeticiones y los corchetes [ ] indican un elemento opcional. Las palabras entre comillas son reservadas y no distinguen mayúsculas de minúsculas.

<programa> ::= "Inicio" { <declaracion> } { <sentencia> } "Fin"

<declaracion> ::= "Declare" <lista\_ident> <tipo> ";"
<lista\_ident> ::= <ident> { "," <ident> }
<tipo> ::= "Entero" | "Real" | "Cadena" | "Fecha" | "Lógico"

<sentencia> ::= <asignacion> | <lectura> | <escritura>
 | <bloque\_si> | <bloque\_mq> | <bloque\_para>

<asignacion> ::= <ident> "=" <expr> ";"
<lectura> ::= "Lea" "(" <ident> ")" ";"
<escritura> ::= "Escriba" "(" <ident> ")" ";"

<bloque\_si> ::= "Si" <condicion> "Entonces" { <sentencia> }
 [ "Sino" { <sentencia> } ]
 "FSi" [ ";" ]
<bloque\_mq> ::= "MQ" <condicion> "Haga" { <sentencia> } "FMQ" [ ";" ]
<bloque\_para> ::= "Para" <ident> "=" <indice> "Hasta" <indice> "Haga"
 { <sentencia> } "FPara" [ ";" ]

<condicion> ::= <operando> <op\_rel> <operando>
<operando> ::= <ident> | <numero>
<indice> ::= <ident> | <entero>
<op\_rel> ::= ">" | "<" | "<>" | ">=" | "<=" | "="

<expr> ::= <termino> { ( "+" | "-" ) <termino> }
<termino> ::= <factor> { ( "\*" | "/" ) <factor> }
<factor> ::= <ident> | <numero> | "(" <expr> ")"

<ident> ::= <letra> { <letra> | <digito> | "\_" }
<numero> ::= <entero> [ "." <entero> ]
<entero> ::= <digito> { <digito> }
<letra> ::= "A" .. "Z" | "a" .. "z"
<digito> ::= "0" .. "9"

Tres reglas no se expresan en la gramática y la aplicación las comprueba aparte: que todo identificador esté declarado (RF-09), que no se declare dos veces (RF-08) y que las variables de un Para sean enteras (RF-19). Como <expr> solo admite paréntesis que se abren y se cierran en orden, la gramática ya garantiza el RF-12.

## 6. Catálogo de errores

El enunciado exige que la aplicación «tipifique explícitamente el error y la línea». Para eso, cada error pertenece a uno de tres tipos y lleva un código. Los mensajes de la tabla son sugeridos: el grupo puede redactarlos distinto, siempre que indiquen el tipo, la línea y la causa.

Formato recomendado de salida:

Error [E09] Sintáctico – Línea 7: falta la palabra 'Entonces' en el bloque Si.

| Código | Tipo | Se produce cuando | Mensaje sugerido |
| --- | --- | --- | --- |
| E01 | Léxico | Aparece un carácter que el lenguaje no admite, como &. | Carácter no permitido '&'. |
| E02 | Léxico | Un identificador no cumple la expresión regular (empieza por número, tiene espacio, usa una palabra reservada). | Identificador inválido '1x'. |
| E03 | Sintáctico | Falta Inicio o aparece algo antes de él. | Falta la palabra 'Inicio'. |
| E04 | Sintáctico | Falta Fin o aparece algo después de él. | Falta la palabra 'Fin'. |
| E05 | Sintáctico | Una sentencia no termina en ;. | Falta ';' al final de la sentencia. |
| E06 | Sintáctico | La línea Declare está mal formada: falta el tipo, el tipo no existe o la lista tiene comas mal puestas. | Declaración inválida: falta el tipo de dato. |
| E07 | Sintáctico | Una asignación, Lea o Escriba está mal formada. | Asignación inválida: falta el signo '='. |
| E08 | Sintáctico | Una condición no tiene dos operandos y un operador relacional válido. | Condición inválida: operador no reconocido. |
| E09 | Sintáctico | A un bloque le falta un componente obligatorio: Entonces, Haga o Hasta. Si en su lugar hay otra palabra (por ejemplo hga), se reporta igual. | Falta la palabra 'Haga' en el bloque MQ. |
| E10 | Sintáctico | Hay un bloque sin cierre: más Si, MQ o Para que FSi, FMQ o FPara. | Bloque Para sin su FPara. |
| E11 | Sintáctico | Hay un cierre sin apertura, o un Sino fuera de un Si o repetido en el mismo Si. | FSi sin su Si correspondiente. |
| E12 | Sintáctico | Los paréntesis de una expresión no están balanceados. | Paréntesis desbalanceados en la expresión. |
| E13 | Sintáctico | Una expresión está mal formada: operador sin operando, operandos seguidos, paréntesis vacíos. | Expresión inválida: falta un operando. |
| E14 | Semántico | Se usa un identificador que no fue declarado. | Variable 'n' no declarada. |
| E15 | Semántico | Un identificador se declara más de una vez. | Variable 'a' ya declarada. |
| E16 | Semántico | La variable o un índice de un Para no es de tipo Entero. | La variable 'x' del Para debe ser Entero. |

La línea que se informa sigue el RF-27: la de la apertura para un bloque sin cierre, la del cierre para un cierre sin apertura, y en los demás casos la línea donde aparece el problema.

Si un bloque tiene más de un problema, basta con reportar el primero que se encuentre (RF-29). Por ejemplo, en Si a > b sin Entonces y sin FSi, se reporta E09 en la línea del Si, y se recomienda reportar también E10.

## 7. Requisitos no funcionales

| ID | Requisito |
| --- | --- |
| RNF-01 | Lenguaje libre: cada grupo elige el lenguaje de programación y lo justifica en el informe técnico. |
| RNF-02 | La aplicación debe funcionar correctamente el día de la sustentación. El grupo instala con anticipación las librerías necesarias. |
| RNF-03 | La validación del nombre de las variables debe implementarse con una expresión regular visible en el código fuente. |
| RNF-04 | El usuario debe poder ingresar el pseudocódigo escribiéndolo o cargándolo desde un archivo de texto, y ver el resultado sin consultar el código. |
| RNF-05 | Los mensajes de error deben estar en español y ser comprensibles para quien escribió el pseudocódigo. |
| RNF-06 | El mismo programa de entrada debe producir siempre el mismo resultado. |
| RNF-07 | Se recomienda separar el código en tres módulos: lectura de elementos (analizador léxico), verificación de la estructura (analizador sintáctico) y reporte de errores. |
| RNF-08 | El código debe ser propio. Un programa copiado de otro grupo se sanciona según la sección 10. |

## 8. Casos de prueba de aceptación

El docente puede usar estos casos, y otros propios, para comprobar la aplicación en la sustentación. Cada caso se prueba dentro de un programa mínimo con Inicio, las declaraciones que necesite y Fin. Los números de línea se cuentan desde la primera línea del fragmento probado.

### 8.1 Sentencias de una línea

Las variables que aparecen en las asignaciones están declaradas, salvo en el caso P13.

| ID | Línea de prueba | Resultado esperado |
| --- | --- | --- |
| P01 | Declare a1 entero; | Válida |
| P02 | Declare x, y, z real; | Válida |
| P03 | Declare 1x entero; | E02: empieza por número |
| P04 | Declare x&1 entero; | E01: carácter & |
| P05 | Declare x, , y real; | E06: lista con comas mal puestas |
| P06 | Declare p; | E06: falta el tipo |
| P07 | Declare x numerico; | E06: tipo inexistente |
| P08 | Declare a entero | E05: falta ; |
| P09 | X = (5 + y); | Válida |
| P10 | A = 5 \* (8 + x \* (c + b)); | Válida |
| P11 | x = ( 5 + x; | E12: paréntesis desbalanceados |
| P12 | Y = 8 + 3 \* ( b+c)); | E12: paréntesis desbalanceados |
| P13 | z = 4; con z sin declarar | E14: variable no declarada |
| P14 | A = ( b - ) 8M | E13: expresión mal formada |

### 8.2 Bloques

Bloque anidado válido. Se espera «programa correcto» si a, b y x están declaradas:

Si a < 5 entonces
 a = 3;
 Si b > 1 entonces
 b = a \* (b + (4 - x));
 Sino
 b = b - (a + 2);
 FSi
FSi;

Bloque Si sin Entonces. Se espera E09 en la línea 1:

Si a > b
 a = 1;
FSi

Bloque MQ con hga en lugar de haga. Se espera E09 en la línea 1:

MQ a >= 5 hga
 b = b \* a;
 a = a \* 1;
FMQ

Bloque Para sin Hasta. Se espera E09 en la línea 1:

Para i = 1 n haga
 b = 2 \* i;
FPara

Dos Para y un solo FPara. Se espera E10 en la línea 1, la del Para externo, que queda sin cerrar:

Para i = 1 hasta n haga
 Para j = 1 hasta n haga
 x = x + 1;
 FPara

Dos Sino en el mismo Si. Se espera E11 en la línea 5:

Si a > b entonces
 a = 1;
Sino
 a = 2;
Sino
 a = 3;
FSi;

## 9. Entregables

Cada grupo entrega dos cosas: la aplicación funcionando y un informe técnico.

| Entregable | Formato | Dónde se entrega |
| --- | --- | --- |
| Aplicación | Programa que cumple las secciones 3 a 7 | Se sustenta en el salón de clases |
| Informe técnico + código fuente | Documento de 8 a 10 hojas de contenido sin contar el código fuente. | Cargado a la plataforma Ferrum |

### 9.1 Contenido del informe técnico

1. **Marco teórico del análisis sintáctico.** Qué es un analizador sintáctico, qué relación tiene con las gramáticas y con la compilación.
2. **Justificación del lenguaje de programación.** Por qué el grupo eligió el lenguaje en el que desarrolló la aplicación.
3. **Descripción de cómo se validó cada funcionalidad.** Para cada una, se explica la estructura de datos y las rutinas usadas, y se muestra un pantallazo del código fuente. Las funcionalidades son:
   * Variables no declaradas
   * Líneas Declare
   * Líneas de asignación
   * Paréntesis
   * Primitiva Si
   * Bloques Para
   * Bloques MQ
4. **Dificultades encontradas y cómo se resolvieron.**

### 9.2 Contenido mínimo de cada funcionalidad en el informe

* Los requisitos RF que cubre (por ejemplo, la validación de Declare cubre RF-04 a RF-08).
* La estructura de datos usada (por ejemplo, una pila para emparejar bloques y paréntesis, o una tabla de símbolos para las variables declaradas).
* La rutina o función que la implementa, descrita en pocas líneas.
* El pantallazo del fragmento de código fuente correspondiente.
* Un caso de prueba válido y uno inválido, con el resultado que mostró la aplicación.

## 10. Plazo, sustentación y evaluación

### 10.1 Plazo de entrega

El enunciado original fija la entrega el jueves **26 de Noviembre 2026**.

### 10.2 Sustentación

El proyecto se sustenta en el salón de clases. Cada grupo muestra la aplicación funcionando, procesa los programas de prueba que indique el docente y responde preguntas sobre su código y su informe.

### 10.3 Criterios de evaluación

| Criterio | Qué se revisa | Peso |
| --- | --- | --- |
| Declaraciones y variables | Línea Declare, expresión regular de identificadores, variables no declaradas o repetidas (RF-04 a RF-09) | 15 % |
| Asignaciones y paréntesis | Asignaciones, expresiones bien formadas, paréntesis, Lea y Escriba (RF-10 a RF-12, RF-24, RF-25) | 10 % |
| Bloques de control | Si, MQ y Para: componentes obligatorios, anidamiento y emparejamiento con su cierre (RF-13 a RF-23) | 20 % |
| Estructura y reporte de errores | Inicio, Fin, terminador ;, y tipo y línea correctos en cada error (RF-01 a RF-03, RF-26 a RF-29) | 10 % |
| Informe técnico | Las cuatro partes de la sección 9.1, en 8 a 10 hojas de contenido | 25 % |
| Sustentación | Dominio del código, funcionamiento ante programas de prueba y respuesta a las preguntas | 20 % |
| **Total** |  | **100 %** |

### 10.4 Sanciones

* El grupo que no presente la aplicación recibe una calificación de 0.0.
* Si un grupo copia el programa de otro, ambos grupos reciben 0.0.

## Anexo A. Programa de ejemplo y decisiones de interpretación

### A.1 Programa que la aplicación debe aceptar

Es el ejemplo del enunciado, ajustado a las reglas de este documento. La tabla A.2 explica cada ajuste.

Inicio
Declare a, b, c1 Entero;
Declare x Real;
Declare p Cadena;
Declare j, i Entero;
Lea(a); Lea(b);
Si a < 5 Entonces
 a = 3;
 Si b > 1 Entonces
 b = a \* (b + (4 - x));
 Sino
 b = b - (a + 2);
 FSi
FSi;
Para i = 1 Hasta j Haga
 c1 = c1 + 1;
 Para b = 1 Hasta i Haga
 x = x + 2 \* c1;
 FPara;
FPara;
MQ a < j Haga
 a = a + 1;
 MQ i < a Haga
 i = i + 1;
 FMQ;
FMQ;
Escriba(a);
Escriba(b);
Fin

### A.2 Diferencias con el ejemplo del enunciado

| En el enunciado | En este documento | Motivo |
| --- | --- | --- |
| Para i = 1 a j haga | Para i = 1 Hasta j Haga | El enunciado exige la palabra hasta y da como inválido un Para que no la tiene. |
| FinSi tras el fsi interno | FSi | FinSi no es una palabra reservada; la lista del enunciado solo incluye FSi. |
| 4 –x con guion largo | 4 - x | Efecto tipográfico; véase la sección 3.3. |
| I = i + 1; | i = i + 1; | Se unifica el nombre; véase la decisión 1 de la tabla A.3. |

### A.3 Decisiones de interpretación

El enunciado deja abiertos los puntos siguientes. Este documento los resolvió así, para que todos los grupos trabajen con las mismas reglas.

| N.º | Tema | Qué dice el enunciado | Decisión |
| --- | --- | --- | --- |
| 1 | Mayúsculas | El ejemplo mezcla Si, si, FSi, fsi, y usa I = i + 1. | Palabras reservadas e identificadores no distinguen mayúsculas de minúsculas. |
| 2 | Tipo Lógico | La lista escribe «Lógico» con tilde. | También se acepta Logico. |
| 3 | Letras de los identificadores | «Carácter alfabético», sin precisar. | Solo A–Z y a–z. |
| 4 | ; después de FSi, FMQ, FPara | La sintaxis los escribe a veces con ; y a veces sin él. | Ambas formas son válidas. |
| 5 | Sentencias por línea | El ejemplo escribe Lea(a);Lea(b); en una línea. | Una línea puede tener varias sentencias, y una sentencia varias líneas. |
| 6 | Orden de las declaraciones | Solo el ejemplo las muestra al inicio. | Las declaraciones van antes de las demás sentencias. |
| 7 | Anidamiento | Dice que un bloque contiene «otros bloques» del mismo tipo. | Se permite anidar bloques de cualquier tipo. |
| 8 | Índices del Para | Dice que var, ind1 e ind2 son variables enteras; el ejemplo usa 1. | var es variable Entero; ind1 e ind2 son variables Entero o números enteros. |
| 9 | Operandos de una condición | Dice «dos variables»; el ejemplo usa a < 5. | Cada operando es un identificador o un número. |
| 10 | Código fuera de Inicio–Fin | No lo menciona. | No se admite. |
| 11 | Declaración repetida | No lo menciona. | Es un error (E15). |
| 12 | Cantidad de errores a reportar | Pide tipificar el error y la línea. | Obligatorio el primero; recomendable todos. |