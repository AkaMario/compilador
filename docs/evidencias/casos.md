# Casos ejecutados contra el binario real

35 casos correctos. Líneas absolutas desde 1. Cada entrada es un programa completo.

## P01

```text
Inicio Declare a1 entero; Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## P02

```text
Inicio Declare x,y,z real; Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## P03

```text
Inicio Declare 1x entero; Fin
```

Esperado: E02. Estado del proceso: 1.

```text
Error [E02] Léxico - Línea 1: Identificador inválido '1x'; debe empezar por una letra ASCII y no ser una palabra reservada.
```

## P04

```text
Inicio Declare x&1 entero; Fin
```

Esperado: E01. Estado del proceso: 1.

```text
Error [E01] Léxico - Línea 1: Carácter no permitido '&'.
```

## P05

```text
Inicio Declare x,,y real; Fin
```

Esperado: E06. Estado del proceso: 1.

```text
Error [E06] Sintáctico - Línea 1: Declaración inválida: falta un identificador en la lista.
```

## P06

```text
Inicio Declare p; Fin
```

Esperado: E06. Estado del proceso: 1.

```text
Error [E06] Sintáctico - Línea 1: Declaración inválida: falta un tipo válido (Entero, Real, Cadena, Fecha o Lógico) o una coma entre variables.
```

## P07

```text
Inicio Declare x numerico; Fin
```

Esperado: E06. Estado del proceso: 1.

```text
Error [E06] Sintáctico - Línea 1: Declaración inválida: falta un tipo válido (Entero, Real, Cadena, Fecha o Lógico) o una coma entre variables.
```

## P08

```text
Inicio
Declare a entero
Fin
```

Esperado: E05. Estado del proceso: 1.

```text
Error [E05] Sintáctico - Línea 2: Falta ';' al final de la sentencia.
```

## P09

```text
Inicio
Declare a,b,c,x,y Entero;
X = (5 + y);
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## P10

```text
Inicio
Declare a,b,c,x,y Entero;
A = 5 * (8 + x * (c + b));
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## P11

```text
Inicio
Declare a,b,c,x,y Entero;
x = ( 5 + x;
Fin
```

Esperado: E12. Estado del proceso: 1.

```text
Error [E12] Sintáctico - Línea 3: Paréntesis de apertura sin cierre en la expresión.
```

## P12

```text
Inicio
Declare a,b,c,x,y Entero;
Y = 8 + 3 * ( b+c));
Fin
```

Esperado: E12. Estado del proceso: 1.

```text
Error [E12] Sintáctico - Línea 3: Paréntesis de cierre sin apertura en la expresión.
```

## P13

```text
Inicio
Declare a,b,c,x,y Entero;
z = 4;
Fin
```

Esperado: E14. Estado del proceso: 1.

```text
Error [E14] Semántico - Línea 3: Variable 'z' no declarada.
```

## P14

```text
Inicio
Declare a,b,c,x,y Entero;
A = ( b - ) 8M
Fin
```

Esperado: E13. Estado del proceso: 1.

```text
Error [E13] Sintáctico - Línea 3: Expresión inválida: falta un operando antes de ')'.
```

## B01

```text
Inicio
Declare a,b,c,x,y Entero;
Si a<5 Entonces a=3; Si b>1 Entonces b=a*(b+(4-x)); Sino b=b-(a+2); FSi FSi;
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## B02

```text
Inicio
Declare a,b,c,x,y Entero;
Si a>b
a=1;
FSi
Fin
```

Esperado: E09. Estado del proceso: 1.

```text
Error [E09] Sintáctico - Línea 3: Falta 'entonces' en el bloque Si.
```

## B03

```text
Inicio
Declare a,b,c,x,y Entero;
MQ a>=5 hga
b=b*a;
a=a*1;
FMQ
Fin
```

Esperado: E09. Estado del proceso: 1.

```text
Error [E09] Sintáctico - Línea 3: Falta 'haga' en el bloque MQ.
```

## B04

```text
Inicio
Declare a,b,c,x,y Entero;
Para a=1 b haga
x=2*a;
FPara
Fin
```

Esperado: E09. Estado del proceso: 1.

```text
Error [E09] Sintáctico - Línea 3: Falta 'hasta' en el bloque Para.
```

## B05

```text
Inicio
Declare a,b,c,x,y Entero;
Para a=1 Hasta b Haga
Para c=1 Hasta b Haga
x=x+1;
FPara
Fin
```

Esperado: E10. Estado del proceso: 1.

```text
Error [E10] Sintáctico - Línea 3: Bloque Para sin su FPara.
```

## B06

```text
Inicio
Declare a,b,c,x,y Entero;
Si a>b Entonces
a=1;
Sino
a=2;
Sino
a=3;
FSi
Fin
```

Esperado: E11. Estado del proceso: 1.

```text
Error [E11] Sintáctico - Línea 7: 'Sino' fuera de un Si o repetido en el mismo bloque.
```

## variables_valido

```text
Inicio
Declare a Entero;
a=1;
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## variables_invalido

```text
Inicio
z=1;
Fin
```

Esperado: E14. Estado del proceso: 1.

```text
Error [E14] Semántico - Línea 2: Variable 'z' no declarada.
```

## declare_valido

```text
Inicio
Declare x,y Real;
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## declare_invalido

```text
Inicio
Declare x,,y Real;
Fin
```

Esperado: E06. Estado del proceso: 1.

```text
Error [E06] Sintáctico - Línea 2: Declaración inválida: falta un identificador en la lista.
```

## asignacion_valido

```text
Inicio
Declare a Entero;
a=5+2*3;
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## asignacion_invalido

```text
Inicio
Declare a Entero;
a=5+;
Fin
```

Esperado: E13. Estado del proceso: 1.

```text
Error [E13] Sintáctico - Línea 3: Expresión inválida: falta un operando.
```

## parentesis_valido

```text
Inicio
Declare a Entero;
a=5*(8+(3+2));
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## parentesis_invalido

```text
Inicio
Declare a Entero;
a=(5+2;
Fin
```

Esperado: E12. Estado del proceso: 1.

```text
Error [E12] Sintáctico - Línea 3: Paréntesis de apertura sin cierre en la expresión.
```

## si_valido

```text
Inicio
Si 1<2 Entonces
Sino
FSi
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## si_invalido

```text
Inicio
Si 1<2
FSi
Fin
```

Esperado: E09. Estado del proceso: 1.

```text
Error [E09] Sintáctico - Línea 2: Falta 'entonces' en el bloque Si.
```

## para_valido

```text
Inicio
Declare i Entero;
Para i=1 Hasta 3 Haga
FPara
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## para_invalido

```text
Inicio
Declare i Real;
Para i=1 Hasta 3 Haga
FPara
Fin
```

Esperado: E16. Estado del proceso: 1.

```text
Error [E16] Semántico - Línea 3: La variable 'i' del Para debe ser Entero.
```

## mq_valido

```text
Inicio
MQ 1<2 Haga
FMQ
Fin
```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```

## mq_invalido

```text
Inicio
MQ 1<2 hga
FMQ
Fin
```

Esperado: E09. Estado del proceso: 1.

```text
Error [E09] Sintáctico - Línea 2: Falta 'haga' en el bloque MQ.
```

## anexo_a

```text
Inicio
Declare a, b, c1 Entero;
Declare x Real;
Declare p Cadena;
Declare j, i Entero;
Lea(a); Lea(b);
Si a < 5 Entonces
    a = 3;
    Si b > 1 Entonces
        b = a * (b + (4 - x));
    Sino
        b = b - (a + 2);
    FSi
FSi;
Para i = 1 Hasta j Haga
    c1 = c1 + 1;
    Para b = 1 Hasta i Haga
        x = x + 2 * c1;
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

```

Esperado: Correcto. Estado del proceso: 0.

```text
El programa es sintácticamente correcto.
```
