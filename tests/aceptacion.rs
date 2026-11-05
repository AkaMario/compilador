use compilador::sintactico::analizar;

fn programa(sentencia: &str) -> String {
    format!("Inicio\nDeclare a,b,c,x,y Entero;\n{sentencia}\nFin")
}

#[test]
fn p01_a_p14_del_documento() {
    let casos = [
        ("P01", "Inicio Declare a1 entero; Fin".into(), None),
        ("P02", "Inicio Declare x,y,z real; Fin".into(), None),
        ("P03", "Inicio Declare 1x entero; Fin".into(), Some("E02")),
        ("P04", "Inicio Declare x&1 entero; Fin".into(), Some("E01")),
        ("P05", "Inicio Declare x,,y real; Fin".into(), Some("E06")),
        ("P06", "Inicio Declare p; Fin".into(), Some("E06")),
        ("P07", "Inicio Declare x numerico; Fin".into(), Some("E06")),
        ("P08", "Inicio\nDeclare a entero\nFin".into(), Some("E05")),
        ("P09", programa("X = (5 + y);"), None),
        ("P10", programa("A = 5 * (8 + x * (c + b));"), None),
        ("P11", programa("x = ( 5 + x;"), Some("E12")),
        ("P12", programa("Y = 8 + 3 * ( b+c));"), Some("E12")),
        ("P13", programa("z = 4;"), Some("E14")),
        ("P14", programa("A = ( b - ) 8M"), Some("E13")),
    ];
    for (id, entrada, esperado) in casos {
        let real = analizar(&entrada).err().map(|e| e.codigo.to_string());
        assert_eq!(real.as_deref(), esperado, "{id}: {entrada}");
    }
}

#[test]
fn acepta_el_programa_completo_del_anexo() {
    let entrada = include_str!("../ejemplos/anexo_a.txt");
    let resumen = analizar(entrada).unwrap();
    assert_eq!(resumen.variables, 7);
}

#[test]
fn bloques_del_documento_con_lineas_absolutas() {
    let casos = [
        ("Si a > b\na=1;\nFSi", "E09", 3),
        ("MQ a >= 5 hga\nb=b*a;\na=a*1;\nFMQ", "E09", 3),
        ("Para a = 1 b haga\nx=2*a;\nFPara", "E09", 3),
        (
            "Para a=1 Hasta b Haga\nPara c=1 Hasta b Haga\nx=x+1;\nFPara",
            "E10",
            3,
        ),
        (
            "Si a>b Entonces\na=1;\nSino\na=2;\nSino\na=3;\nFSi",
            "E11",
            7,
        ),
    ];
    for (fragmento, codigo, linea) in casos {
        let error = analizar(&programa(fragmento)).unwrap_err();
        assert_eq!(error.codigo.to_string(), codigo, "{fragmento}");
        assert_eq!(error.linea, linea, "{fragmento}");
    }
}

#[test]
fn acepta_mayusculas_tipos_y_sentencias_multilinea() {
    let casos = [
        "iNiCiO Declare A Entero; a=1; Escriba(A); fIn",
        "Inicio Declare a Logico; Declare b LÓGICO; Declare c Cadena; Declare d Fecha; Declare e Real; Fin",
        "Inicio\nDeclare a,\nb Entero;\na\n=\n(5\n+ b); Lea (a);Escriba(b);\nFin",
        "Inicio Declare i Entero; Si i=1 Entonces MQ i<2 Haga Para i=1 Hasta 2 Haga FPara; FMQ FSi; Fin",
        "Inicio Fin",
        "Inicio Declare a Entero; a=1.5/0; Fin",
        "Inicio Declare a1,x_3,x1234567890,a123yx0 Entero; Fin",
        "Inicio Declare a Entero; Si 1<>2 Entonces Sino FSi Fin",
    ];
    for entrada in casos {
        assert!(
            analizar(entrada).is_ok(),
            "{entrada}: {:?}",
            analizar(entrada)
        );
    }
}

#[test]
fn rechaza_las_infracciones_de_la_gramatica() {
    let casos = [
        ("", "E03"),
        ("Fin", "E03"),
        ("a=1; Inicio Fin", "E03"),
        ("Inicio", "E04"),
        ("Inicio Fin Fin", "E04"),
        ("Inicio Inicio Fin", "E03"),
        ("Inicio; Fin", "E07"),
        ("Inicio Fin;", "E04"),
        ("Inicio Declare ,a Entero; Fin", "E06"),
        ("Inicio Declare a, Entero; Fin", "E06"),
        ("Inicio Declare Inicio Entero; Fin", "E02"),
        ("Inicio Declare _a Entero; Fin", "E02"),
        ("Inicio Declare ñ Entero; Fin", "E02"),
        ("Inicio Declare a 1 Entero; Fin", "E02"),
        ("Inicio Declare a Entero; Declare A Real; Fin", "E15"),
        ("Inicio Declare a Entero; a=1; Declare b Real; Fin", "E06"),
        ("Inicio Si 1<2 Entonces Declare a Entero; FSi Fin", "E06"),
        ("Inicio Lea(x); Fin", "E14"),
        ("Inicio Escriba(x); Fin", "E14"),
        ("Inicio Lea(1); Fin", "E07"),
        ("Inicio Escriba(1+2); Fin", "E07"),
        ("Inicio Declare a Entero; Lea a; Fin", "E07"),
        ("Inicio Declare a Entero; a 5; Fin", "E07"),
        ("Inicio Declare a Entero; a=5 Fin", "E05"),
        ("Inicio Declare a Entero; a=5+; Fin", "E13"),
        ("Inicio Declare a Entero; a=(); Fin", "E13"),
        ("Inicio Declare a Entero; a=5 6; Fin", "E13"),
        ("Inicio Declare a Entero; a=-5; Fin", "E13"),
        ("Inicio Declare a Entero; a=3.; Fin", "E13"),
        ("Inicio Declare a Entero; a=(1)); Fin", "E12"),
        ("Inicio Declare a Entero; a=)(1; Fin", "E12"),
        ("Inicio Si 1+2 Entonces FSi Fin", "E08"),
        ("Inicio Si 1<2<3 Entonces FSi Fin", "E08"),
        ("Inicio Si (1)<2 Entonces FSi Fin", "E08"),
        ("Inicio Si 1==2 Entonces FSi Fin", "E08"),
        ("Inicio Si 1<2 FSi Fin", "E09"),
        ("Inicio MQ 1<2 FMQ Fin", "E09"),
        ("Inicio Si 1<2 Entonces Fin", "E10"),
        ("Inicio MQ 1<2 Haga Fin", "E10"),
        ("Inicio FSi Fin", "E11"),
        ("Inicio FMQ Fin", "E11"),
        ("Inicio FPara Fin", "E11"),
        ("Inicio Sino Fin", "E11"),
        ("Inicio Si 1<2 Entonces MQ 1<2 Haga FSi FMQ Fin", "E11"),
        (
            "Inicio Declare i Real; Para i=1 Hasta 2 Haga FPara Fin",
            "E16",
        ),
        (
            "Inicio Declare i Entero; Para i=1.0 Hasta 2 Haga FPara Fin",
            "E16",
        ),
        (
            "Inicio Declare i Entero; Declare n Real; Para i=1 Hasta n Haga FPara Fin",
            "E16",
        ),
        ("Inicio Para i=1 Hasta 2 Haga FPara Fin", "E14"),
        (
            "Inicio Declare i Entero; Para i=1 Hasta n Haga FPara Fin",
            "E14",
        ),
        (
            "Inicio Declare i Entero; Para i 1 Hasta 2 Haga FPara Fin",
            "E09",
        ),
        ("Inicio Declare i Entero; Para i=1 Hasta 2 FPara Fin", "E09"),
        (
            "Inicio Declare i Entero; Para i=1+2 Hasta 3 Haga FPara Fin",
            "E09",
        ),
        ("Inicio Declare a Entero; a=\"hola\"; Fin", "E01"),
    ];
    for (entrada, esperado) in casos {
        let error = analizar(entrada).expect_err(entrada);
        assert_eq!(error.codigo.to_string(), esperado, "{entrada}: {error}");
    }
}

#[test]
fn conserva_lineas_y_repite_el_mismo_resultado() {
    let entrada = "Inicio\r\nDeclare a Entero;\r\na=1;\r\n\r\nz=2;\r\nFin";
    let error = analizar(entrada).unwrap_err();
    assert_eq!(error.linea, 5);
    assert!(error.to_string().contains("Semántico"));
    for _ in 0..10 {
        assert_eq!(analizar(entrada).unwrap_err(), error);
    }
    let error = analizar("Inicio\nSi 1<2 Entonces\nMQ 1<2 Haga\nFMQ\nFin").unwrap_err();
    assert_eq!((error.codigo.to_string(), error.linea), ("E10".into(), 2));
}
