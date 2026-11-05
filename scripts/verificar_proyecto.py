#!/usr/bin/env python3
"""Registra las comprobaciones completas. Termina si alguna falla."""
from pathlib import Path
import subprocess
import sys

raiz = Path(__file__).resolve().parents[1]
registro = raiz / 'docs/evidencias/verificacion.txt'
comandos = [
    ['rustc', '--version'],
    ['cargo', '--version'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'build', '--locked', '--offline'],
    [sys.executable, 'scripts/verificar_casos.py'],
]
registro.parent.mkdir(parents=True, exist_ok=True)
with registro.open('w') as archivo:
    archivo.write('Verificación del proyecto - Fedora Linux 44 - 3 de noviembre de 2026\n')
    for comando in comandos:
        salida = subprocess.run(comando, cwd=raiz, capture_output=True, text=True)
        archivo.write('\n$ ' + ' '.join(comando) + '\n')
        archivo.write(salida.stdout + salida.stderr)
        archivo.write(f'Estado: {salida.returncode}\n')
        archivo.flush()
        print(f"{'OK' if salida.returncode == 0 else 'ERROR'}: {' '.join(comando)}", flush=True)
        if salida.returncode:
            print(salida.stdout + salida.stderr)
            raise SystemExit(salida.returncode)
print('Registro completo: docs/evidencias/verificacion.txt')
