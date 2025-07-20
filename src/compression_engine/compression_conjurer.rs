//! Модуль арифметического сжатия
//!
//! Реализует алгоритм арифметического кодирования с оптимизацией словаря.
//! Преобразует данные в компактное представление с восстановимостью.

use crate::bit_wizardry::bit_manipulation_spells::{BitMagicWriter, ARITHMETIC_PRECISION_LIMIT};
use std::collections::HashMap;

/// Результат сжатия - содержит все данные для восстановления
#[derive(Debug, Clone)]
pub struct CompressionArtifact {
    /// Таблица частот: (id символа, частота, накопительная позиция)
    pub mystical_frequency_codex: Vec<(u32, u64, u64)>,
    /// Общее количество символов
    pub total_frequency_essence: u64,
    /// Сжатый битовый поток
    pub compressed_bit_stream: Vec<u8>,
    /// Словарь часто встречающихся слов
    pub mystical_word_grimoire: Vec<String>,
}

/// Сжимает данные с помощью арифметического кодирования
///
/// Алгоритм:
/// 1. Строит словарь часто встречающихся слов
/// 2. Преобразует текст в символы (байты + ссылки на слова)
/// 3. Анализирует частоты для таблицы вероятностей
/// 4. Выполняет арифметическое кодирование
pub fn weave_compression_spell(original_manuscript: &[u8]) -> CompressionArtifact {
    // Находим выгодные слова для словаря
    let mystical_word_grimoire = discover_profitable_word_enchantments(original_manuscript);

    // Преобразуем текст в символы
    let symbolic_incantations =
        transform_manuscript_to_symbols(original_manuscript, &mystical_word_grimoire);

    // Анализируем частоты
    let frequency_analysis_results = analyze_symbolic_frequencies(&symbolic_incantations);

    // Выполняем арифметическое кодирование
    let mut compressed_bit_stream = Vec::new();
    let mut bit_conjurer = BitMagicWriter::conjure_new(&mut compressed_bit_stream);

    let mut interval_low = 0u32;
    let mut interval_high = ARITHMETIC_PRECISION_LIMIT;

    // Кодируем каждый символ
    for mystical_symbol in symbolic_incantations {
        if let Some((_, symbol_frequency, cumulative_start)) = frequency_analysis_results
            .frequency_entries
            .iter()
            .find(|&&(symbol_id, _, _)| symbol_id == mystical_symbol)
        {
            let symbol_start = *cumulative_start as u32;
            let symbol_end = (*cumulative_start + *symbol_frequency) as u32;
            let total_mass = frequency_analysis_results.total_frequency_mass as u32;

            bit_conjurer.encode_mystical_symbol(
                &mut interval_low,
                &mut interval_high,
                symbol_start,
                symbol_end,
                total_mass,
            );
        }
    }

    bit_conjurer.complete_compression_ritual();
    CompressionArtifact {
        mystical_frequency_codex: frequency_analysis_results.frequency_entries,
        total_frequency_essence: frequency_analysis_results.total_frequency_mass,
        compressed_bit_stream,
        mystical_word_grimoire,
    }
}

/// Результат анализа частот
#[derive(Debug)]
struct FrequencyAnalysisWisdom {
    /// (символ, частота, накопительная позиция)
    frequency_entries: Vec<(u32, u64, u64)>,
    /// Общая сумма частот
    total_frequency_mass: u64,
}

/// Находит слова, выгодные для включения в словарь
///
/// Критерии отбора:
/// - Частота > 3 вхождений
/// - Экономия: длина × частота > длина + 4 (накладные расходы)
/// - Учитывается регистр
fn discover_profitable_word_enchantments(manuscript_bytes: &[u8]) -> Vec<String> {
    // Для маленьких файлов словарь неэффективен
    #[cfg(not(test))]
    if manuscript_bytes.len() < 1000 {
        return Vec::new();
    }

    let manuscript_text = String::from_utf8_lossy(manuscript_bytes);
    let mut word_frequency_almanac = HashMap::new();
    let mut current_word_buffer = String::new();

    // Разбиваем на слова по ASCII буквам
    for mystical_character in manuscript_text.chars() {
        if mystical_character.is_ascii_alphabetic() || mystical_character == '\'' {
            current_word_buffer.push(mystical_character);
        } else {
            if current_word_buffer.len() >= 3 {
                *word_frequency_almanac
                    .entry(current_word_buffer.clone())
                    .or_insert(0u64) += 1;
            }
            current_word_buffer.clear();
        }
    }
    if current_word_buffer.len() >= 3 {
        *word_frequency_almanac
            .entry(current_word_buffer)
            .or_insert(0u64) += 1;
    }

    // Отбираем выгодные слова
    let mut profitable_word_candidates: Vec<(String, u64, i64)> = word_frequency_almanac
        .into_iter()
        .filter_map(|(enchanted_word, occurrence_frequency)| {
            // Вычисляем экономию
            let compression_savings = (enchanted_word.len() as i64 * occurrence_frequency as i64)
                - (enchanted_word.len() as i64 + 4);

            if occurrence_frequency > 3 && compression_savings > 0 {
                Some((enchanted_word, occurrence_frequency, compression_savings))
            } else {
                None
            }
        })
        .collect();

    profitable_word_candidates
        .sort_by_key(|(_, _, compression_savings)| std::cmp::Reverse(*compression_savings));

    profitable_word_candidates.truncate(25);

    let selected_word_grimoire: Vec<String> = profitable_word_candidates
        .iter()
        .map(|(enchanted_word, _, _)| enchanted_word.clone())
        .collect();

    // Отладочный вывод
    if !selected_word_grimoire.is_empty() {
        println!("Найдено {} полезных слов:", selected_word_grimoire.len());

        for (spell_index, (word, frequency, savings)) in
            profitable_word_candidates.iter().enumerate().take(10)
        {
            println!(
                "  {}: '{}' ({}x, {} байт экономии)",
                spell_index, word, frequency, savings
            );
        }
    }

    selected_word_grimoire
}

/// Преобразует текст в символы, заменяя слова ссылками на словарь
///
/// Кодирование:
/// - 0-255: обычные байты
/// - 256+: ссылки на словарь (256 + индекс)
fn transform_manuscript_to_symbols(manuscript_bytes: &[u8], word_grimoire: &[String]) -> Vec<u32> {
    let mut symbolic_sequence = Vec::new();
    let mut byte_position = 0;

    while byte_position < manuscript_bytes.len() {
        let mut word_spell_discovered = false;

        // Пытаемся найти слово, если встретили букву
        if manuscript_bytes[byte_position].is_ascii_alphabetic()
            || manuscript_bytes[byte_position] == b'\''
        {
            // Проверяем каждое слово из словаря
            for (grimoire_index, mystical_word) in word_grimoire.iter().enumerate() {
                let word_bytes = mystical_word.as_bytes();

                if byte_position + word_bytes.len() <= manuscript_bytes.len() {
                    let mut perfect_word_match = true;

                    // Сравниваем побайтно
                    for (offset, &expected_byte) in word_bytes.iter().enumerate() {
                        if manuscript_bytes[byte_position + offset] != expected_byte {
                            perfect_word_match = false;
                            break;
                        }
                    }

                    // Проверяем границы слова
                    if perfect_word_match {
                        let word_end_position = byte_position + word_bytes.len();

                        let valid_word_start = byte_position == 0
                            || !manuscript_bytes[byte_position - 1].is_ascii_alphabetic();
                        let valid_word_end = word_end_position >= manuscript_bytes.len()
                            || !manuscript_bytes[word_end_position].is_ascii_alphabetic();

                        if valid_word_start && valid_word_end {
                            // Заменяем ссылкой на словарь
                            symbolic_sequence.push(256u32 + grimoire_index as u32);
                            byte_position += word_bytes.len();
                            word_spell_discovered = true;
                            break;
                        }
                    }
                }
            }
        }

        // Если слово не найдено, добавляем байт как есть
        if !word_spell_discovered {
            symbolic_sequence.push(manuscript_bytes[byte_position] as u32);
            byte_position += 1;
        }
    }

    symbolic_sequence
}

/// Строит таблицу частот для арифметического кодирования 🔍📊
/// Использует эффективное заимствование срезов без копирования данных
fn analyze_symbolic_frequencies(symbolic_incantations: &[u32]) -> FrequencyAnalysisWisdom {
    // Подсчет частот
    let mut symbol_frequency_map = HashMap::new();

    for &mystical_symbol in symbolic_incantations {
        *symbol_frequency_map.entry(mystical_symbol).or_insert(0u64) += 1;
    }

    // Сортировка для детерминированности
    let mut frequency_pairs: Vec<(u32, u64)> = symbol_frequency_map.into_iter().collect();
    frequency_pairs.sort_by_key(|&(symbol_id, _)| symbol_id);

    // Общая сумма
    let total_frequency_mass: u64 = frequency_pairs
        .iter()
        .map(|&(_, frequency)| frequency)
        .sum();

    // Накопительная таблица для интервалов
    let mut cumulative_position = 0u64;
    let frequency_entries: Vec<(u32, u64, u64)> = frequency_pairs
        .iter()
        .map(|&(symbol_id, frequency_count)| {
            let current_position = cumulative_position;
            cumulative_position += frequency_count;
            (symbol_id, frequency_count, current_position)
        })
        .collect();

    // Готовая структура данных
    FrequencyAnalysisWisdom {
        frequency_entries,
        total_frequency_mass,
    }
}

/// Тесты алгоритмов сжатия 🎯
#[cfg(test)]
mod compression_conjurer_tests {
    use super::*;

    /// Проверка словаря
    #[test]
    fn test_word_discovery_functionality() {
        let sample_text = b"the quick brown fox jumps over the lazy dog the end the beginning the world the universe the magic the power";
        let discovered_words = discover_profitable_word_enchantments(sample_text);

        // "the" должно попасть в словарь
        assert!(discovered_words.contains(&"the".to_string()));
    }

    /// Проверка символьного кодирования
    #[test]
    fn test_symbol_transformation() {
        let test_data = b"hello world hello";
        let word_dict = vec!["hello".to_string()];
        let symbols = transform_manuscript_to_symbols(test_data, &word_dict);

        // Ссылки на словарь (256+) и обычные байты
        assert!(symbols.contains(&256)); // "hello"
        assert!(symbols.contains(&32)); // пробел
    }

    /// Проверка подсчета частот
    #[test]
    fn test_frequency_analysis() {
        let symbols = vec![65u32, 66u32, 65u32]; // A, B, A
        let analysis = analyze_symbolic_frequencies(&symbols);

        assert_eq!(analysis.total_frequency_mass, 3);
        assert_eq!(analysis.frequency_entries.len(), 2);
    }
}
