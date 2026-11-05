use compilador::archivos::cargar;

#[test]
fn carga_utf8_y_no_confunde_un_error_de_archivo_con_error_del_programa() {
    let texto = cargar("ejemplos/anexo_a.txt").unwrap();
    assert!(compilador::sintactico::analizar(&texto).is_ok());
    assert!(
        cargar("ejemplos/no_existe.txt")
            .unwrap_err()
            .contains("No se pudo abrir")
    );
}

#[test]
fn rechaza_archivos_que_no_son_utf8() {
    let ruta = std::env::temp_dir().join(format!("compilador-utf8-{}.txt", std::process::id()));
    std::fs::write(&ruta, [0xff, 0xfe]).unwrap();
    let resultado = cargar(&ruta);
    std::fs::remove_file(ruta).unwrap();
    assert!(resultado.unwrap_err().contains("UTF-8"));
}
