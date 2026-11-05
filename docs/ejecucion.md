# Consola e intérprete de pseudocódigo

Esta función se añadió a petición del usuario después del analizador. El documento original excluye ejecutar programas; por eso **Analizar** conserva los requisitos originales y **Ejecutar** es una ampliación explícita. Ningún archivo se ejecuta al cargarlo o analizarlo.

## Uso

1. Escribir el programa o abrir un ejemplo.
2. Pulsar **Ejecutar** o **F5**. Primero se valida la sintaxis y las declaraciones. Si hay un error E01-E16, no comienza la ejecución.
3. Leer la salida en **Consola de ejecución**. `Escriba(a);` agrega el valor actual de `a` en una línea. `a = 10;` por sí solo almacena el valor.
4. Cuando aparece **Lea(a) - esperando un dato**, escribirlo en **Entrada** y pulsar **Enviar dato** o Enter.
5. Si la entrada es incorrecta, corregirla y volver a enviarla; el programa sigue esperando en la misma instrucción.
6. Usar **Detener** para interrumpir ciclos o una espera de datos. **Copiar salida** copia únicamente las líneas impresas.

Cada nueva ejecución comienza desde el principio, borra su salida y descarta los valores anteriores. Editar el código descarta la ejecución y su consola; así no se confunden resultados viejos con código modificado. Los estados son EnCurso, Esperando, Finalizada, Detenida y Error.

## Ejemplo que imprime 10

```text
Inicio
Declare a Entero;
a = 10;
Escriba(a);
Fin
```

Salida:

```text
10
```

El archivo listo para abrir es `ejemplos/consola_10.txt`. Para iniciarlo en el editor:

```bash
cargo run --locked -- --abrir ejemplos/consola_10.txt
```

Después pulsa **Ejecutar**. El botón **Analizar** sigue mostrando solo el resultado de la validación.

## Entrada interactiva y ciclos

`ejemplos/consola_entrada.txt` lee dos enteros, los suma y escribe el resultado. Si introduces 10 y 2, imprime 12. La espera de entrada no bloquea la aplicación.

`ejemplos/consola_bloques.txt` combina Para, Si/Sino y MQ y produce, en este orden:

```text
1
3
6
12
```

La salida no incluye automáticamente los nombres de las variables ni todas las asignaciones intermedias: solo las instrucciones Escriba que realmente se ejecutan.

## Semántica de la ampliación

- **Variables:** mantienen el tratamiento de mayúsculas del analizador. Empiezan sin inicializar; consultar una variable antes de asignarle un valor o leerla provoca un error de ejecución con línea.
- **Expresiones:** multiplicación y división preceden a suma y resta. Operadores del mismo nivel se asocian a la izquierda. Los paréntesis alteran el orden. No se añaden nuevas formas al pseudocódigo.
- **Números:** se representan con `f64`. La impresión elimina `.0` si el valor es entero. La aritmética decimal puede tener el redondeo habitual de punto flotante; no se ofrece precisión decimal arbitraria. Se rechazan números o resultados no finitos y divisiones por cero.
- **Lea Entero:** acepta un número finito sin fracción, entre -9007199254740991 y 9007199254740991. Se admiten signos en los datos ingresados; esto no agrega signos unarios a la gramática fuente.
- **Lea Real:** acepta un número finito. Se usa punto como separador decimal.
- **Lea Cadena o Fecha:** conserva el texto escrito. No se impone un formato de calendario que la especificación no definió.
- **Lea Lógico/Logico:** admite Verdadero/Falso y true/false sin distinguir mayúsculas.
- **Asignaciones:** no imponen compatibilidad entre el tipo declarado y el valor copiado, para conservar la validación exigida. La aritmética sí necesita valores numéricos en ejecución. El tipo declarado determina cómo Lea interpreta una entrada posterior.
- **Condiciones:** comparan números entre sí, textos entre sí o lógicos entre sí. El orden de textos es lexicográfico, no cronológico para fechas. Comparar categorías distintas da un error de ejecución. No hay operadores lógicos ni evaluación de expresiones dentro de condiciones.
- **Si/Sino:** ejecuta solo la rama seleccionada; una rama que no se toma no consulta sus valores ni calcula sus expresiones.
- **MQ:** vuelve a comprobar su condición antes de cada iteración.
- **Para:** asigna el inicio, comprueba el límite inclusivo y aumenta el contador en uno. Evalúa la variable usada como límite en cada iteración. Si el inicio supera al límite, no entra al cuerpo. Al terminar normalmente, el contador conserva el valor siguiente al último procesado. Los valores efectivos de contador e índices deben ser enteros dentro del rango seguro, además de la validación estática del tipo Entero.

No se ejecutan comandos del sistema ni se usa `eval`. El intérprete solo opera sobre las instrucciones de pseudocódigo reconocidas por el analizador.

## Implementación y límites

`Ejecucion::nueva` valida el programa y construye instrucciones internas con saltos. `expresion` utiliza shunting-yard para convertir operaciones a notación posfija. `evaluar` usa una pila de valores; no hay recursión por cada paréntesis o bloque. Las instrucciones de control usan destinos explícitos, por lo que los bloques anidados se ejecutan sin una pila de llamadas proporcional a su profundidad.

`avanzar(500)` ejecuta como máximo 500 instrucciones por cuadro de la ventana. Un estado Esperando conserva la instrucción Lea hasta que `aportar_entrada` recibe un dato válido. Un presupuesto total de 100000 instrucciones y un límite de 5000 líneas evitan que un ciclo infinito o una salida ilimitada bloquee la aplicación. Alcanzar esos límites termina con un mensaje de ejecución; no cambia la validez sintáctica del programa.

Los errores de ejecución tienen el formato `Error de ejecución - Línea N: causa`. No usan nuevos códigos E17 ni cambian el significado de E01-E16. La línea corresponde a la instrucción que se estaba ejecutando; en una expresión multilínea se informa la línea de la asignación.

## Verificación

`tests/ejecucion.rs` cubre salida 10, precedencia, asociación, valores reales, ramas, anidamiento, MQ, Para, lectura de los cinco tipos, pausa/reanudación, valores no inicializados, división por cero, números no finitos y límites. Las pruebas del editor incluyen F5, entrada por Enter, código inválido y descarte al editar. El registro actualizado está en `docs/evidencias/verificacion.txt`.

Las capturas de la consola se obtienen de la ventana real, mediante el ejemplo `capturar`:

```bash
cargo run --locked --example capturar -- ejemplos/consola_10.txt /tmp/consola10.ppm --ejecutar
```

El ejemplo captura únicamente la aplicación y se cierra. En esta ampliación, las pruebas de consola se verificaron mediante el intérprete y eventos reales del sistema de widgets de egui, sin renderizador nativo. No se adjunta una captura nueva: la revisión automática para acceder a la sesión gráfica expiró. Las capturas de ventana ya incluidas corresponden a la versión previa del analizador.
