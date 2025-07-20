//! # Модуль Декомпрессии 🔮
//!
//! Этот модуль восстанавливает исходные данные из сжатых артефактов.
//! Демонстрирует математическую связь между алгоритмами сжатия и декомпрессии.
//!
//! ## Как работает декомпрессия:
//! Проходим те же интервалы вероятности, что и при сжатии, но в обратном порядке.
//! Сжатый битовый поток указывает путь обратно к исходным символам.
//!
//! ## Возможности Rust в этом модуле:
//! - **Pattern matching**: Восстановление символов с помощью match
//! - **Итераторы**: Функциональная обработка данных
//! - **Безопасность памяти**: Нет переполнений буфера
//! - **Обработка ошибок**: Graceful fallback с Option/Result
//! - **Ownership**: Эффективная передача данных
//! - **Типобезопасность**: Предотвращение ошибок декомпрессии
//! - **Точная арифметика**: Идеальное восстановление данных

use crate::bit_wizardry::bit_manipulation_spells::{BitMagicReader, ARITHMETIC_PRECISION_LIMIT};
use crate::compression_engine::compression_conjurer::CompressionArtifact;

/// Основная функция декомпрессии 🔮
///
/// Восстанавливает исходные данные из сжатого артефакта.
/// Функция принимает владение артефактом и возвращает исходные байты.
///
/// ## Алгоритм:
/// 1. Инициализируем декодер из битового потока
/// 2. Восстанавливаем символы по таблице частот
/// 3. Навигируем по интервалам вероятности
/// 4. Преобразуем символы обратно в байты
///
/// ## Важно:
/// Эта функция должна быть точной математической противоположностью сжатия.
/// Любая ошибка приведет к повреждению данных.
///
/// ## Параметры:
/// - `enchanted_artifact`: Сжатый артефакт с данными для восстановления
///
/// ## Возвращает:
/// - `Vec<u8>`: Восстановленная последовательность байтов
pub fn unweave_compression_spell(enchanted_artifact: CompressionArtifact) -> Vec<u8> {
    // Извлекаем компоненты артефакта
    let CompressionArtifact {
        mystical_frequency_codex,
        total_frequency_essence,
        compressed_bit_stream,
        mystical_word_grimoire,
    } = enchanted_artifact;

    // Показываем таблицу частот для отладки
    let original_size = total_frequency_essence as usize;
    display_frequency_codex_wisdom(&mystical_frequency_codex, original_size);

    // Создаем читатель битов (передаем владение данными)
    let mut mystical_bit_reader = BitMagicReader::conjure_from_scroll(compressed_bit_stream);

    // Инициализируем состояние арифметического кодирования
    let mut interval_low = 0u32;
    let mut interval_high = ARITHMETIC_PRECISION_LIMIT;

    // Заранее резервируем память под результат
    let mut decoded_symbols = Vec::with_capacity(total_frequency_essence as usize);

    // Декодируем точно столько символов, сколько было закодировано
    for _symbol_position in 0..total_frequency_essence {
        // Определяем, какой символ соответствует текущей позиции в битовом потоке
        let target_position = mystical_bit_reader.decode_mystical_target(
            total_frequency_essence as u32,
            interval_low,
            interval_high,
        );

        // Ищем символ по позиции в таблице частот
        let discovered_symbol = mystical_frequency_codex
            .iter()
            .find(|&&(_, symbol_frequency, cumulative_start)| {
                let symbol_end = cumulative_start + symbol_frequency;
                target_position >= cumulative_start as u32 && target_position < symbol_end as u32
            })
            .map(|&(symbol_id, _, _)| symbol_id)
            .unwrap_or_else(|| {
                // Если символ не найден, берем первый доступный
                mystical_frequency_codex
                    .first()
                    .map(|&(symbol_id, _, _)| symbol_id)
                    .unwrap_or(0)
            });

        // Обновляем интервалы кодирования для найденного символа
        if let Some((_, symbol_frequency, cumulative_start)) = mystical_frequency_codex
            .iter()
            .find(|&&(symbol_id, _, _)| symbol_id == discovered_symbol)
        {
            let symbol_start = *cumulative_start as u32;
            let symbol_end = (*cumulative_start + *symbol_frequency) as u32;
            let total_mass = total_frequency_essence as u32;

            // Обновляем состояние декодера
            mystical_bit_reader.update_mystical_intervals(
                &mut interval_low,
                &mut interval_high,
                symbol_start,
                symbol_end,
                total_mass,
            );
        }

        // Добавляем декодированный символ в результат
        decoded_symbols.push(discovered_symbol);
    }

    // Преобразуем символы обратно в исходные байты
    reconstruct_original_manuscript(&decoded_symbols, &mystical_word_grimoire)
}

/// Восстанавливает исходные байты из символов 📜
///
/// Преобразует декодированные символы обратно в последовательность байтов.
/// Использует ссылки на словарь для восстановления слов.
///
/// ## Типы символов:
/// - 0-255: Обычные байты (копируются как есть)
/// - 256+: Ссылки на слова из словаря
///
/// ## Параметры:
/// - `decoded_mystical_symbols`: Декодированные символы
/// - `word_grimoire`: Словарь слов для восстановления
fn reconstruct_original_manuscript(
    decoded_mystical_symbols: &[u32],
    word_grimoire: &[String],
) -> Vec<u8> {
    // Создаем буфер для результата
    let mut reconstructed_manuscript = Vec::new();

    // Обрабатываем каждый символ
    for &mystical_symbol in decoded_mystical_symbols {
        // Определяем тип символа
        match mystical_symbol {
            // Обычный байт
            0..=255 => {
                // Добавляем байт как есть
                reconstructed_manuscript.push(mystical_symbol as u8);
            }
            // Ссылка на слово из словаря
            word_reference => {
                // Вычисляем индекс в словаре
                let grimoire_index = (word_reference - 256) as usize;

                // Безопасно получаем слово из словаря
                if let Some(enchanted_word) = word_grimoire.get(grimoire_index) {
                    // Добавляем все байты слова в результат
                    reconstructed_manuscript.extend_from_slice(enchanted_word.as_bytes());
                }
                // Недействительные ссылки игнорируются (защитное программирование)
            }
        }
    }

    reconstructed_manuscript
}

/// Отображает таблицу частот символов 📊
///
/// Показывает результаты анализа частот в табличном формате.
/// Полезно для отладки и понимания эффективности сжатия.
///
/// ## Формат отображения:
/// - ID символа: Числовой идентификатор (байт или ссылка на слово)
/// - Частота: Количество появлений символа
/// - Начало: Позиция начала в накопительной таблице
/// - Конец: Позиция конца в накопительной таблице
fn display_frequency_codex_wisdom(
    mystical_frequency_codex: &[(u32, u64, u64)],
    original_size: usize,
) {
    // Выводим заголовок таблицы
    println!("📊 Original data: {} bytes", original_size);
    println!("🔮 Mystical Frequency Codex:");
    println!(
        "{:<8} {:<12} {:<12} {}",
        "Symbol", "Frequency", "Start", "End"
    );
    println!("{}", "━".repeat(45)); // Разделительная линия

    // Сортируем по частоте и показываем только топ символов
    let mut sorted_entries: Vec<_> = mystical_frequency_codex.iter().collect();
    sorted_entries.sort_by_key(|(_, frequency_count, _)| std::cmp::Reverse(*frequency_count));

    let max_entries = 20; // Показываем только топ-20 символов
    let entries_to_show = sorted_entries.len().min(max_entries);

    // Выводим записи таблицы
    for &(symbol_id, frequency_count, cumulative_start) in
        sorted_entries.iter().take(entries_to_show)
    {
        let cumulative_end = cumulative_start + frequency_count;

        // Форматированный вывод строки таблицы
        println!(
            "{:<8} {:<12} {:<12} {}",
            symbol_id, frequency_count, cumulative_start, cumulative_end
        );
    }

    if mystical_frequency_codex.len() > max_entries {
        println!(
            "... and {} more symbols with lower frequencies",
            mystical_frequency_codex.len() - max_entries
        );
    }

    println!(); // Empty line for visual separation
}

/// Модульные тесты
#[cfg(test)]
mod decompression_sage_tests {
    use super::*;
    use crate::compression_engine::compression_conjurer::weave_compression_spell;

    /// Тест полного цикла сжатие-декомпрессия
    #[test]
    fn test_compression_decompression_roundtrip() {
        let original_data = b"Hello, magical world of Rust compression!";

        // Выполняем сжатие и декомпрессию
        let compressed_artifact = weave_compression_spell(original_data);
        let reconstructed_data = unweave_compression_spell(compressed_artifact);

        // Проверяем, что данные восстановлены точно
        assert_eq!(original_data.as_slice(), reconstructed_data.as_slice());
    }

    #[test]
    fn test_symbol_reconstruction_with_words() {
        // Test data with repeated words for dictionary compression
        let test_text = b"the quick brown fox jumps over the lazy dog";

        let compressed = weave_compression_spell(test_text);
        let reconstructed = unweave_compression_spell(compressed);

        // Проверяем побайтовое равенство
        assert_eq!(test_text.as_slice(), reconstructed.as_slice());
    }

    #[test]
    fn test_empty_data_handling() {
        let empty_data: &[u8] = b"";

        let compressed = weave_compression_spell(empty_data);
        let reconstructed = unweave_compression_spell(compressed);

        // Проверяем обработку граничного случая
        assert_eq!(empty_data, reconstructed.as_slice());
    }

    #[test]
    fn test_single_byte_compression() {
        let single_byte = b"A";

        let compressed = weave_compression_spell(single_byte);
        let reconstructed = unweave_compression_spell(compressed);

        assert_eq!(single_byte.as_slice(), reconstructed.as_slice());
    }

    #[test]
    fn test_non_ascii_character_preservation() {
        // Include non-ASCII characters to test extended byte handling
        let mixed_data = b"Caf\xc3\xa9 with non-breaking space\xa0here";

        let compressed = weave_compression_spell(mixed_data);
        let reconstructed = unweave_compression_spell(compressed);

        // Проверяем сохранение расширенных символов
        assert_eq!(mixed_data.as_slice(), reconstructed.as_slice());
    }
}
