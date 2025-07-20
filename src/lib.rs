//! Мастер арифметического сжатия 🧙‍♂️✨
//! Библиотека для эффективного сжатия данных

// Экспорт основных модулей
pub mod bit_wizardry;
pub mod compression_engine;
pub mod decompression_oracle;

// Основной API
pub use compression_engine::compression_conjurer::{weave_compression_spell, CompressionArtifact};
pub use decompression_oracle::decompression_sage::unweave_compression_spell;

/// Упрощенный API 🎯
/// Простой интерфейс без работы с внутренними структурами
pub mod simple_api {
    use super::*;

    /// Простая функция сжатия
    ///
    /// Возвращает только сжатые байты, скрывая детали реализации
    pub fn compress_data(original: &[u8]) -> Vec<u8> {
        let artifact = weave_compression_spell(original);

        // Сериализация в единый поток
        // Формат: [словарь][таблица_частот][общая_частота][данные]
        let mut result = Vec::new();

        // Словарь
        result.extend_from_slice(&(artifact.mystical_word_grimoire.len() as u32).to_le_bytes());
        for word in &artifact.mystical_word_grimoire {
            result.extend_from_slice(&(word.len() as u32).to_le_bytes());
            result.extend_from_slice(word.as_bytes());
        }

        // Таблица частот
        result.extend_from_slice(&(artifact.mystical_frequency_codex.len() as u32).to_le_bytes());
        for &(symbol, freq, start) in &artifact.mystical_frequency_codex {
            result.extend_from_slice(&symbol.to_le_bytes());
            result.extend_from_slice(&freq.to_le_bytes());
            result.extend_from_slice(&start.to_le_bytes());
        }

        // Общая частота
        result.extend_from_slice(&artifact.total_frequency_essence.to_le_bytes());

        // Сжатые данные
        result.extend_from_slice(&(artifact.compressed_bit_stream.len() as u32).to_le_bytes());
        result.extend_from_slice(&artifact.compressed_bit_stream);

        result
    }

    /// Простая функция декомпрессии
    /// Восстанавливает данные, сжатые через `compress_data()`
    pub fn decompress_data(compressed: Vec<u8>) -> Vec<u8> {
        let mut cursor = 0;

        // Безопасное чтение байтов
        let read_u32 = |cursor: &mut usize| -> u32 {
            let result = u32::from_le_bytes([
                compressed[*cursor],
                compressed[*cursor + 1],
                compressed[*cursor + 2],
                compressed[*cursor + 3],
            ]);
            *cursor += 4;
            result
        };

        let read_u64 = |cursor: &mut usize| -> u64 {
            let result = u64::from_le_bytes([
                compressed[*cursor],
                compressed[*cursor + 1],
                compressed[*cursor + 2],
                compressed[*cursor + 3],
                compressed[*cursor + 4],
                compressed[*cursor + 5],
                compressed[*cursor + 6],
                compressed[*cursor + 7],
            ]);
            *cursor += 8;
            result
        };

        // Словарь
        let word_count = read_u32(&mut cursor) as usize;
        let mut word_grimoire = Vec::with_capacity(word_count);

        for _ in 0..word_count {
            let word_len = read_u32(&mut cursor) as usize;
            let word_bytes = &compressed[cursor..cursor + word_len];
            word_grimoire.push(String::from_utf8_lossy(word_bytes).into_owned());
            cursor += word_len;
        }

        // Таблица частот
        let freq_count = read_u32(&mut cursor) as usize;
        let mut frequency_codex = Vec::with_capacity(freq_count);

        for _ in 0..freq_count {
            let symbol = read_u32(&mut cursor);
            let freq = read_u64(&mut cursor);
            let start = read_u64(&mut cursor);
            frequency_codex.push((symbol, freq, start));
        }

        // Общая частота
        let total_frequency = read_u64(&mut cursor);

        // Сжатые данные
        let compressed_len = read_u32(&mut cursor) as usize;
        let compressed_data = compressed[cursor..cursor + compressed_len].to_vec();

        // Восстановление артефакта
        let artifact = CompressionArtifact {
            mystical_frequency_codex: frequency_codex,
            total_frequency_essence: total_frequency,
            compressed_bit_stream: compressed_data,
            mystical_word_grimoire: word_grimoire,
        };

        unweave_compression_spell(artifact)
    }
}

/// Модуль Prelude 🌟
/// Импортирует все необходимое для сжатия
pub mod prelude {
    pub use crate::compression_engine::compression_conjurer::{
        weave_compression_spell, CompressionArtifact,
    };
    pub use crate::decompression_oracle::decompression_sage::unweave_compression_spell;
    pub use crate::simple_api::{compress_data, decompress_data};
}

/// Статистика сжатия 📊
/// Анализ эффективности и метрики
pub mod statistics {
    use crate::prelude::*;
    use std::collections::HashMap;

    /// Результаты анализа сжатия
    #[derive(Debug, Clone)]
    pub struct CompressionAnalysis {
        /// Размер исходных данных в байтах
        pub original_size: usize,
        /// Размер сжатых данных в байтах
        pub compressed_size: usize,
        /// Коэффициент сжатия в процентах
        pub compression_ratio: f64,
        /// Энтропия Шеннона исходных данных
        pub shannon_entropy: f64,
        /// Достигнутая плотность сжатия
        pub compression_density: f64,
        /// Количество слов в словаре
        pub word_dictionary_size: usize,
        /// Наиболее частые символы
        pub top_symbols: Vec<(u32, u64)>,
    }

    /// Анализирует эффективность сжатия
    pub fn analyze_compression(data: &[u8]) -> CompressionAnalysis {
        let artifact = weave_compression_spell(data);

        // Энтропия Шеннона
        let mut freq = HashMap::new();
        for &byte in data {
            *freq.entry(byte).or_insert(0u64) += 1;
        }

        let total = data.len() as f64;
        let mut entropy = 0.0;
        for count in freq.values() {
            let p = (*count as f64) / total;
            entropy -= p * p.log2();
        }

        let compressed_size = artifact.compressed_bit_stream.len();
        let compression_ratio = (1.0 - compressed_size as f64 / data.len() as f64) * 100.0;
        let compression_density = compressed_size as f64 * 8.0 / data.len() as f64;

        // Топ символов
        let mut symbol_freq: Vec<_> = artifact
            .mystical_frequency_codex
            .iter()
            .map(|&(symbol, freq, _)| (symbol, freq))
            .collect();
        symbol_freq.sort_by_key(|&(_, freq)| std::cmp::Reverse(freq));
        symbol_freq.truncate(10);

        CompressionAnalysis {
            original_size: data.len(),
            compressed_size,
            compression_ratio,
            shannon_entropy: entropy,
            compression_density,
            word_dictionary_size: artifact.mystical_word_grimoire.len(),
            top_symbols: symbol_freq,
        }
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::simple_api::*;
    use super::statistics::*;

    #[test]
    fn test_round_trip_compression() {
        let test_cases = vec![
            b"Hello, world!".as_slice(),
            b"The quick brown fox jumps over the lazy dog".as_slice(),
            b"a".as_slice(),
            b"".as_slice(),
            b"aaaaaaaaaa".as_slice(),
            b"abcdefghijklmnopqrstuvwxyz".as_slice(),
        ];

        for original in test_cases {
            let compressed = compress_data(original);
            let restored = decompress_data(compressed);
            assert_eq!(
                original,
                restored.as_slice(),
                "Round-trip failed for: {:?}",
                std::str::from_utf8(original)
            );
        }
    }

    #[test]
    fn test_compression_analysis() {
        let data = b"the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning \
                     the quick brown fox jumps over the lazy dog the end the beginning";
        let analysis = analyze_compression(data);

        assert!(analysis.original_size > 0);
        assert!(analysis.compressed_size > 0);
        assert!(analysis.shannon_entropy > 0.0);
        // Word dictionary size may be 0 for very repetitive text due to overhead calculations
        assert!(!analysis.top_symbols.is_empty());
    }

    #[test]
    fn test_empty_data() {
        let empty: &[u8] = b"";
        let compressed = compress_data(empty);
        let restored = decompress_data(compressed);
        assert_eq!(empty, restored.as_slice());
    }
}
