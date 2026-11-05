//! Catálogo de errores E01-E16 de la especificación, sección 6.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodigoError {
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    E16,
}

impl CodigoError {
    pub fn tipo(self) -> &'static str {
        match self {
            Self::E01 | Self::E02 => "Léxico",
            Self::E14 | Self::E15 | Self::E16 => "Semántico",
            _ => "Sintáctico",
        }
    }
}

impl fmt::Display for CodigoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCompilacion {
    pub codigo: CodigoError,
    pub linea: usize,
    pub mensaje: String,
}

impl ErrorCompilacion {
    pub fn nuevo(codigo: CodigoError, linea: usize, mensaje: impl Into<String>) -> Self {
        Self {
            codigo,
            linea,
            mensaje: mensaje.into(),
        }
    }
}

impl fmt::Display for ErrorCompilacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error [{}] {} - Línea {}: {}",
            self.codigo,
            self.codigo.tipo(),
            self.linea,
            self.mensaje
        )
    }
}

impl std::error::Error for ErrorCompilacion {}
