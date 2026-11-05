#!/usr/bin/env python3
"""Genera imágenes del código real y un PDF de nueve páginas de contenido."""
from pathlib import Path
import html
import json
import re
import textwrap

from PIL import Image, ImageDraw, ImageFont
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image as PDFImage, PageBreak, Preformatted, KeepTogether
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from pypdf import PdfReader

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / 'docs'
EVIDENCIA = DOCS / 'evidencias'
FONT = '/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf'
BOLD = '/usr/share/fonts/dejavu-sans-fonts/DejaVuSans-Bold.ttf'
MONO = '/usr/share/fonts/google-noto/NotoSansMono-Regular.ttf'


def imagen_codigo(nombre, archivo, ancla, cantidad):
    lineas = (ROOT / archivo).read_text().splitlines()
    inicio = next(i for i, linea in enumerate(lineas) if ancla in linea)
    seleccion = lineas[inicio:inicio + cantidad]
    filas = []
    for numero, linea in enumerate(seleccion, inicio + 1):
        partes = textwrap.wrap(linea.expandtabs(4), width=86, replace_whitespace=False, drop_whitespace=False) or ['']
        for indice, parte in enumerate(partes):
            filas.append((str(numero) if indice == 0 else '', parte))
    fuente = ImageFont.truetype(MONO, 20)
    cabecera = ImageFont.truetype(FONT, 18)
    ancho = 1140
    alto = 64 + 27 * len(filas) + 18
    imagen = Image.new('RGB', (ancho, alto), '#101b2a')
    dibujo = ImageDraw.Draw(imagen)
    dibujo.rectangle([0, 0, ancho, 45], fill='#21354c')
    dibujo.text((20, 12), f'{archivo}  |  líneas {inicio + 1}-{inicio + cantidad}', font=cabecera, fill='#e4eef8')
    for indice, (numero, linea) in enumerate(filas):
        y = 61 + 27 * indice
        dibujo.text((14, y), numero.rjust(3), font=fuente, fill='#91a7bf')
        dibujo.text((73, y), linea, font=fuente, fill='#f2f5fa')
    imagen.save(EVIDENCIA / f'codigo_{nombre}.png')


def capturas_codigo():
    for nombre, archivo, ancla, cantidad in [
        ('variables', 'src/sintactico.rs', 'fn declarada(', 11),
        ('regex', 'src/lexico.rs', 'pub const PATRON_IDENTIFICADOR', 4),
        ('declare', 'src/sintactico.rs', 'fn declaracion(', 12),
        ('asignacion', 'src/sintactico.rs', 'fn asignacion(', 10),
        ('parentesis', 'src/sintactico.rs', 'parentesis.push(token.linea);', 19),
        ('si', 'src/sintactico.rs', 'fn bloque_condicional(', 16),
        ('para', 'src/sintactico.rs', 'fn bloque_para(', 17),
        ('mq', 'src/sintactico.rs', 'fn bloque_condicional(', 16),
    ]:
        imagen_codigo(nombre, archivo, ancla, cantidad)


def main():
    EVIDENCIA.mkdir(parents=True, exist_ok=True)
    capturas_codigo()
    registros = {r['id']: r for r in json.loads((EVIDENCIA / 'casos.json').read_text())}
    texto = (DOCS / 'informe_base.md').read_text()

    def caso(coincidencia):
        r = registros[coincidencia.group(1)]
        return f"```text\n{r['entrada']}\n```\n\nSalida real: {r['salida']}"

    texto = re.sub(r'\{\{caso:([a-z_]+)\}\}', caso, texto)
    (DOCS / 'informe_tecnico.md').write_text(texto)
    paginas = texto.split('<!-- pagina -->')
    assert len(paginas) == 9
    for nombre, ruta in [('Texto', FONT), ('TextoBold', BOLD), ('Codigo', MONO)]:
        pdfmetrics.registerFont(TTFont(nombre, ruta))
    pdfmetrics.registerFontFamily('Texto', normal='Texto', bold='TextoBold', italic='Texto', boldItalic='TextoBold')
    estilos = {
        'h1': ParagraphStyle('h1', fontName='TextoBold', fontSize=19, leading=24, textColor=colors.HexColor('#123451'), spaceAfter=13),
        'h2': ParagraphStyle('h2', fontName='TextoBold', fontSize=12, leading=16, spaceBefore=8, spaceAfter=8, textColor=colors.HexColor('#245577')),
        'h3': ParagraphStyle('h3', fontName='TextoBold', fontSize=10, leading=13, spaceBefore=7, spaceAfter=4),
        'p': ParagraphStyle('p', fontName='Texto', fontSize=9.4, leading=13.6, spaceAfter=9, alignment=TA_LEFT),
        'caption': ParagraphStyle('caption', fontName='Texto', fontSize=8, leading=10, textColor=colors.HexColor('#4b5f70'), spaceAfter=9),
        'code': ParagraphStyle('code', fontName='Codigo', fontSize=8.4, leading=11, backColor=colors.HexColor('#f1f5f8'), borderPadding=6, spaceBefore=4, spaceAfter=8),
    }
    historia = []
    ancho = A4[0] - 96
    for numero, contenido in enumerate(paginas):
        if numero: historia.append(PageBreak())
        lineas = contenido.strip().splitlines()
        i = 0
        while i < len(lineas):
            linea = lineas[i]
            if not linea.strip(): i += 1; continue
            if linea.startswith('```'):
                bloque = []
                i += 1
                while i < len(lineas) and not lineas[i].startswith('```'):
                    bloque.extend(textwrap.wrap(lineas[i], width=92, replace_whitespace=False, drop_whitespace=False) or [''])
                    i += 1
                historia.append(Preformatted('\n'.join(bloque), estilos['code']))
            elif linea.startswith('!['):
                m = re.fullmatch(r'!\[(.*?)\]\((.*?)\)', linea)
                titulo, ruta = m.groups()
                archivo = DOCS / ruta
                with Image.open(archivo) as img: w, h = img.size
                # Las capturas de ventana se reducen para conservar espacio al marco teórico.
                maxalto = 176 if 'ventana_' in ruta else 235
                escala = min(ancho / w, maxalto / h)
                historia.append(KeepTogether([PDFImage(str(archivo), width=w * escala, height=h * escala), Spacer(1, 4), Paragraph(html.escape(titulo), estilos['caption'])]))
            elif linea.startswith('#'):
                n = len(linea) - len(linea.lstrip('#'))
                historia.append(Paragraph(html.escape(linea[n:].strip()), estilos[f'h{min(n, 3)}']))
            else:
                parrafo = [linea]
                while i + 1 < len(lineas) and lineas[i+1].strip() and not lineas[i+1].startswith(('#', '![', '```')):
                    i += 1
                    parrafo.append(lineas[i])
                historia.append(Paragraph(html.escape(' '.join(parrafo)), estilos['p']))
            i += 1
    destino = ROOT / 'output/pdf/informe_tecnico.pdf'
    destino.parent.mkdir(parents=True, exist_ok=True)

    def encabezado(canvas, doc):
        canvas.setStrokeColor(colors.HexColor('#d7e1ea'))
        canvas.line(48, A4[1] - 38, A4[0] - 48, A4[1] - 38)
        canvas.setFont('Texto', 8)
        canvas.setFillColor(colors.HexColor('#4b5f70'))
        canvas.drawString(48, A4[1] - 29, 'II PROYECTO  /  SIMULADOR DE COMPILADOR')
        canvas.drawString(48, 27, 'Informe técnico  |  Rust + eframe  |  Especificación del proyecto')
        canvas.drawRightString(A4[0] - 48, 27, f'{doc.page} / 9')

    doc = SimpleDocTemplate(str(destino), pagesize=A4, rightMargin=48, leftMargin=48, topMargin=55, bottomMargin=48,
        title='Informe técnico - Simulación de un compilador', author='Proyecto de simulador de compilador - desarrollo asistido')
    doc.build(historia, onFirstPage=encabezado, onLaterPages=encabezado)
    lector = PdfReader(destino)
    assert len(lector.pages) == 9, f'Se generaron {len(lector.pages)} páginas, deben ser nueve.'
    for pagina in lector.pages:
        assert len(pagina.extract_text()) > 500, 'Página con contenido insuficiente'
    print(f'PDF verificado: {len(lector.pages)} páginas. Texto editable e imágenes en docs/.')


if __name__ == '__main__':
    main()
