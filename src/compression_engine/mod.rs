//! Модуль движка сжатия 🚀
//! Основные алгоритмы компрессии данных

pub mod compression_conjurer;

// Экспорт основных типов и функций

pub use compression_conjurer::{
    weave_compression_spell, // Главная функция сжатия
    CompressionArtifact,     // Результат сжатия
};
