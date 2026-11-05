#!/usr/bin/env python3
"""Prepara la entrega sin incluir cachés, credenciales ni historial Git."""
from pathlib import Path
import hashlib
import subprocess
import zipfile

raiz = Path(__file__).resolve().parents[1]
entrega = raiz / 'entrega'
entrega.mkdir(exist_ok=True)
binario = entrega / 'compilador-linux-x86_64'
subprocess.run(['strip', '--strip-debug', '-o', str(binario), str(raiz / 'target/debug/compilador')], check=True)
binario.chmod(0o755)
for ruta, estado in [('anexo_a.txt', 0), ('error_sintactico.txt', 1), ('error_semantico.txt', 1)]:
    salida = subprocess.run([str(binario), '--analizar', str(raiz / 'ejemplos' / ruta)], capture_output=True, text=True)
    assert salida.returncode == estado, salida.stdout + salida.stderr
archivo = entrega / 'proyecto_compilador.zip'
with zipfile.ZipFile(archivo, 'w', compression=zipfile.ZIP_DEFLATED, compresslevel=6) as paquete:
    for nombre in ['Cargo.toml', 'Cargo.lock', 'README.md', '.gitignore']:
        paquete.write(raiz / nombre, f'compilador/{nombre}')
    for directorio in ['src', 'tests', 'examples', 'ejemplos', 'context', 'docs', 'scripts', 'output/pdf']:
        for ruta in sorted((raiz / directorio).rglob('*')):
            if ruta.is_file() and '__pycache__' not in ruta.parts:
                paquete.write(ruta, 'compilador/' + str(ruta.relative_to(raiz)))
    paquete.write(binario, 'compilador/compilador-linux-x86_64')
with zipfile.ZipFile(archivo) as paquete:
    assert paquete.testzip() is None
    assert 'compilador/output/pdf/informe_tecnico.pdf' in paquete.namelist()
    assert 'compilador/src/sintactico.rs' in paquete.namelist()
    assert all('/target/' not in nombre and '/.git/' not in nombre for nombre in paquete.namelist())
registros = []
for ruta in [binario, archivo]:
    resumen = hashlib.file_digest(ruta.open('rb'), 'sha256').hexdigest()
    registros.append(f'{resumen}  {ruta.name}')
(entrega / 'SHA256SUMS.txt').write_text('\n'.join(registros) + '\n')
print('Entrega creada y comprobada:', archivo)
print('Ejecutable Linux verificado con los tres archivos de ejemplo.')
