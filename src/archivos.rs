use std::path::Path;

/// Solo carga texto; los problemas de E/S no son errores E01-E16.
pub fn cargar(ruta: impl AsRef<Path>) -> Result<String, String> {
    let ruta = ruta.as_ref();
    std::fs::read_to_string(ruta).map_err(|error| {
        if error.kind() == std::io::ErrorKind::InvalidData {
            format!(
                "El archivo '{}' no es texto UTF-8. Guárdalo con esa codificación.",
                ruta.display()
            )
        } else {
            format!("No se pudo abrir '{}': {error}", ruta.display())
        }
    })
}
