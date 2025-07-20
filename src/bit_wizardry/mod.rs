//! Модуль битового волшебства 🧙‍♂️✨
//! Низкоуровневые операции для арифметического кодирования

pub mod bit_manipulation_spells;

// Экспорт основных типов и констант

pub use bit_manipulation_spells::{
    BitMagicReader,             // Читатель битовых потоков
    BitMagicWriter,             // Писатель битовых потоков
    ARITHMETIC_PRECISION_LIMIT, // Предел точности арифметического кодирования
};
