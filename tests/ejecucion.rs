use compilador::ejecucion::{Ejecucion, Estado};

fn ejecutar(codigo: &str) -> Ejecucion {
    let mut ejecucion = Ejecucion::nueva(codigo).unwrap();
    ejecucion.avanzar(100_001);
    ejecucion
}

#[test]
fn escriba_imprime_el_valor_diez_y_respeta_precedencia_y_parentesis() {
    let e = ejecutar(
        "Inicio Declare a Entero; a=10; Escriba(a); a=2+3*4; Escriba(a); a=(2+3)*4; Escriba(a); a=20/2/2; Escriba(a); a=10-3-2; Escriba(a); Fin",
    );
    assert_eq!(e.salida, ["10", "14", "20", "5", "5"]);
    assert_eq!(e.estado, Estado::Finalizada);
}

#[test]
fn asignar_no_imprime_hasta_escriba_y_las_variables_ignoran_mayusculas() {
    let e = ejecutar("Inicio Declare A,b Real; a=10; b=A/4; Escriba(B); Fin");
    assert_eq!(e.salida, ["2.5"]);
    assert_eq!(e.estado, Estado::Finalizada);
}

#[test]
fn si_y_sino_ejecutan_solo_la_rama_correspondiente() {
    for (comparacion, esperado) in [
        ("1<2", "10"),
        ("1>2", "20"),
        ("2=2", "10"),
        ("2<>2", "20"),
        ("2>=2", "10"),
        ("1<=0", "20"),
    ] {
        let e = ejecutar(&format!(
            "Inicio Declare a Entero; Si {comparacion} Entonces a=10; Sino a=20; FSi Escriba(a); Fin"
        ));
        assert_eq!(e.salida, [esperado]);
    }
    let e = ejecutar("Inicio Declare a Entero; a=3; Si 1>2 Entonces a=1/0; FSi Escriba(a); Fin");
    assert_eq!(e.salida, ["3"]);
}

#[test]
fn ciclos_y_bloques_anidados_producen_sus_salidas_en_orden() {
    let e = ejecutar(
        "Inicio Declare i,j,s Entero; s=0; Para i=1 Hasta 3 Haga Para j=1 Hasta 2 Haga s=s+i; FPara FPara MQ s<15 Haga s=s+1; FMQ Escriba(s); Escriba(i); Fin",
    );
    assert_eq!(e.salida, ["15", "4"]);
    assert_eq!(e.estado, Estado::Finalizada);
    let e = ejecutar("Inicio Declare i Entero; Para i=4 Hasta 1 Haga Escriba(i); FPara Fin");
    assert!(e.salida.is_empty());
    assert_eq!(e.estado, Estado::Finalizada);
}

#[test]
fn lea_pausa_y_continua_sin_perder_el_estado() {
    let mut e =
        Ejecucion::nueva("Inicio\nDeclare a,b Entero;\nLea(a);\nLea(b);\na=a+b;\nEscriba(a);\nFin")
            .unwrap();
    e.avanzar(100);
    assert!(matches!(&e.estado, Estado::Esperando { variable, linea: 3 } if variable == "a"));
    assert!(e.aportar_entrada("hola").is_err());
    assert!(matches!(e.estado, Estado::Esperando { .. }));
    e.aportar_entrada("10").unwrap();
    e.avanzar(100);
    assert!(matches!(&e.estado, Estado::Esperando { variable, linea: 4 } if variable == "b"));
    e.aportar_entrada("-3").unwrap();
    e.avanzar(100);
    assert_eq!(e.salida, ["7"]);
    assert_eq!(e.estado, Estado::Finalizada);
}

#[test]
fn lea_admite_los_cinco_tipos_y_rechaza_entradas_incorrectas() {
    for (tipo, entrada, salida) in [
        ("Cadena", "Hola mundo", "Hola mundo"),
        ("Fecha", "03/11/2026", "03/11/2026"),
        ("Lógico", "verdadero", "Verdadero"),
        ("Real", "2.5", "2.5"),
    ] {
        let mut e =
            Ejecucion::nueva(&format!("Inicio Declare a {tipo}; Lea(a); Escriba(a); Fin")).unwrap();
        e.avanzar(100);
        e.aportar_entrada(entrada).unwrap();
        e.avanzar(100);
        assert_eq!(e.salida, [salida]);
    }
    let mut e = Ejecucion::nueva("Inicio Declare a Entero; Lea(a); Fin").unwrap();
    e.avanzar(100);
    for invalida in ["1.5", "NaN", "inf"] {
        assert!(e.aportar_entrada(invalida).is_err());
    }
}

#[test]
fn errores_de_ejecucion_no_modifican_la_validacion_sintactica() {
    for (cuerpo, texto, linea) in [
        ("a=1;\na=a/0;", "División por cero", 4),
        ("Escriba(a);", "sin inicializar", 3),
        ("a=b+1;", "sin inicializar", 3),
    ] {
        let codigo = format!("Inicio\nDeclare a,b Entero;\n{cuerpo}\nFin");
        assert!(compilador::sintactico::analizar(&codigo).is_ok());
        let e = ejecutar(&codigo);
        let Estado::Error(error) = e.estado else {
            panic!("Faltó error: {cuerpo}");
        };
        assert!(error.mensaje.contains(texto), "{error}");
        assert_eq!(error.linea, linea);
    }
    assert!(Ejecucion::nueva("Inicio Escriba(z); Fin").is_err());
}

#[test]
fn ejecutar_por_turnos_y_detener_no_bloquea_la_interfaz() {
    let mut e = Ejecucion::nueva("Inicio MQ 1<2 Haga FMQ Fin").unwrap();
    e.avanzar(10);
    assert_eq!(e.estado, Estado::EnCurso);
    e.detener();
    e.avanzar(100);
    assert_eq!(e.estado, Estado::Detenida);
    let e = ejecutar("Inicio MQ 1<2 Haga FMQ Fin");
    assert!(matches!(e.estado, Estado::Error(_)));
}

#[test]
fn rechaza_resultados_no_finitos_y_contadores_para_fraccionarios() {
    let e = ejecutar(
        "Inicio Declare a Real; a=999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999; Fin",
    );
    assert!(matches!(e.estado, Estado::Error(_)));
    let e = ejecutar("Inicio Declare i,n Entero; n=1.5; Para i=1 Hasta n Haga FPara Fin");
    assert!(matches!(e.estado, Estado::Error(_)));
}

#[test]
fn la_salida_ilimitada_se_detiene_y_las_asignaciones_pueden_copiar_texto() {
    let e = ejecutar("Inicio Declare a Entero; a=1; MQ 1<2 Haga Escriba(a); FMQ Fin");
    assert_eq!(e.salida.len(), 5000);
    let Estado::Error(error) = e.estado else {
        panic!("Debe limitar la salida");
    };
    assert!(error.mensaje.contains("líneas de salida"));
    let mut e =
        Ejecucion::nueva("Inicio Declare a Cadena; Declare b Entero; Lea(a); b=a; Escriba(b); Fin")
            .unwrap();
    e.avanzar(100);
    e.aportar_entrada("Hola").unwrap();
    e.avanzar(100);
    assert_eq!(e.salida, ["Hola"]);
    assert_eq!(e.estado, Estado::Finalizada);
}
