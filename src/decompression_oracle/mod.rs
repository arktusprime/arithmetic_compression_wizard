//! Модуль оракула декомпрессии 🔮
//! Восстановление данных из сжатых артефактов

pub mod decompression_sage;

// Экспорт основной функции декомпрессии

pub use decompression_sage::{
    unweave_compression_spell, // Восстановление из артефакта
};
