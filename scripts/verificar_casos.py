#!/usr/bin/env python3
"""Ejecuta el binario real y registra evidencias del informe sin usar mocks."""
import json
from pathlib import Path
import re
import subprocess
import tempfile

RAIZ = Path(__file__).resolve().parents[1]
BINARIO = RAIZ / 'target/debug/compilador'


def envolver(fragmento):
    return f'Inicio\nDeclare a,b,c,x,y Entero;\n{fragmento}\nFin'


def casos():
    tabla = [
        ('P01', 'Inicio Declare a1 entero; Fin', None),
        ('P02', 'Inicio Declare x,y,z real; Fin', None),
        ('P03', 'Inicio Declare 1x entero; Fin', 'E02'),
        ('P04', 'Inicio Declare x&1 entero; Fin', 'E01'),
        ('P05', 'Inicio Declare x,,y real; Fin', 'E06'),
        ('P06', 'Inicio Declare p; Fin', 'E06'),
        ('P07', 'Inicio Declare x numerico; Fin', 'E06'),
        ('P08', 'Inicio\nDeclare a entero\nFin', 'E05'),
        ('P09', envolver('X = (5 + y);'), None),
        ('P10', envolver('A = 5 * (8 + x * (c + b));'), None),
        ('P11', envolver('x = ( 5 + x;'), 'E12'),
        ('P12', envolver('Y = 8 + 3 * ( b+c));'), 'E12'),
        ('P13', envolver('z = 4;'), 'E14'),
        ('P14', envolver('A = ( b - ) 8M'), 'E13'),
        ('B01', envolver('Si a<5 Entonces a=3; Si b>1 Entonces b=a*(b+(4-x)); Sino b=b-(a+2); FSi FSi;'), None),
        ('B02', envolver('Si a>b\na=1;\nFSi'), 'E09'),
        ('B03', envolver('MQ a>=5 hga\nb=b*a;\na=a*1;\nFMQ'), 'E09'),
        ('B04', envolver('Para a=1 b haga\nx=2*a;\nFPara'), 'E09'),
        ('B05', envolver('Para a=1 Hasta b Haga\nPara c=1 Hasta b Haga\nx=x+1;\nFPara'), 'E10'),
        ('B06', envolver('Si a>b Entonces\na=1;\nSino\na=2;\nSino\na=3;\nFSi'), 'E11'),
        ('variables_valido', 'Inicio\nDeclare a Entero;\na=1;\nFin', None),
        ('variables_invalido', 'Inicio\nz=1;\nFin', 'E14'),
        ('declare_valido', 'Inicio\nDeclare x,y Real;\nFin', None),
        ('declare_invalido', 'Inicio\nDeclare x,,y Real;\nFin', 'E06'),
        ('asignacion_valido', 'Inicio\nDeclare a Entero;\na=5+2*3;\nFin', None),
        ('asignacion_invalido', 'Inicio\nDeclare a Entero;\na=5+;\nFin', 'E13'),
        ('parentesis_valido', 'Inicio\nDeclare a Entero;\na=5*(8+(3+2));\nFin', None),
        ('parentesis_invalido', 'Inicio\nDeclare a Entero;\na=(5+2;\nFin', 'E12'),
        ('si_valido', 'Inicio\nSi 1<2 Entonces\nSino\nFSi\nFin', None),
        ('si_invalido', 'Inicio\nSi 1<2\nFSi\nFin', 'E09'),
        ('para_valido', 'Inicio\nDeclare i Entero;\nPara i=1 Hasta 3 Haga\nFPara\nFin', None),
        ('para_invalido', 'Inicio\nDeclare i Real;\nPara i=1 Hasta 3 Haga\nFPara\nFin', 'E16'),
        ('mq_valido', 'Inicio\nMQ 1<2 Haga\nFMQ\nFin', None),
        ('mq_invalido', 'Inicio\nMQ 1<2 hga\nFMQ\nFin', 'E09'),
        ('anexo_a', (RAIZ / 'ejemplos/anexo_a.txt').read_text(), None),
    ]
    return tabla


def main():
    if not BINARIO.exists():
        raise SystemExit('Primero ejecutar cargo build --locked.')
    registros = []
    with tempfile.TemporaryDirectory(prefix='compilador-casos-') as carpeta:
        ruta = Path(carpeta) / 'programa.txt'
        for nombre, codigo, esperado in casos():
            ruta.write_text(codigo, encoding='utf-8')
            salida = subprocess.run([str(BINARIO), '--analizar', str(ruta)], capture_output=True, text=True, cwd=RAIZ)
            mensaje = (salida.stdout + salida.stderr).strip()
            encontrado = re.search(r'Error \[(E\d{2})\]', mensaje)
            real = encontrado.group(1) if encontrado else None
            assert salida.returncode == (1 if esperado else 0), (nombre, mensaje)
            assert real == esperado, (nombre, esperado, mensaje)
            registros.append(dict(id=nombre, entrada=codigo, esperado=esperado or 'Correcto', salida=mensaje, estado=salida.returncode))
    destino = RAIZ / 'docs/evidencias'
    destino.mkdir(parents=True, exist_ok=True)
    (destino / 'casos.json').write_text(json.dumps(registros, ensure_ascii=False, indent=2) + '\n')
    md = ['# Casos ejecutados contra el binario real', '', f'{len(registros)} casos correctos. Líneas absolutas desde 1. Cada entrada es un programa completo.', '']
    for registro in registros:
        md += [f"## {registro['id']}", '', '```text', registro['entrada'], '```', '', f"Esperado: {registro['esperado']}. Estado del proceso: {registro['estado']}.", '', '```text', registro['salida'], '```', '']
    (destino / 'casos.md').write_text('\n'.join(md))
    print(f'{len(registros)}/{len(registros)} casos verificados; evidencias en docs/evidencias/.')


if __name__ == '__main__':
    main()
