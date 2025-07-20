//! Заклинания манипуляции битами 🧙‍♂️⚡
//!
//! Битовые операции для арифметического кодирования.
//! Демонстрирует силу Rust в системном программировании.
//!
//! Арифметическое кодирование представляет данные как число в интервале [0, 1).
//! Каждый символ сужает интервал пропорционально своей частоте.
//!
//! Возможности Rust:
//! - Безопасные битовые операции
//! - Константы времени компиляции
//! - Абстракции нулевой стоимости
//! - Типобезопасность

/// Максимальная точность арифметического кодирования (const время компиляции)
pub const ARITHMETIC_PRECISION_LIMIT: u32 = (1 << 24) - 1;
/// Первая четверть
pub const FIRST_QTR: u32 = (ARITHMETIC_PRECISION_LIMIT / 4) + 1;
/// Половина
pub const HALF: u32 = 2 * FIRST_QTR;
/// Третья четверть
pub const THIRD_QTR: u32 = 3 * FIRST_QTR;

/// Писатель битовой магии - превращает байты в сжатые потоки ✨
/// Использует параметры времени жизни для операций без копирования
pub struct BitMagicWriter<'enchanted_output> {
    mystical_output_scroll: &'enchanted_output mut Vec<u8>,
    bit_accumulation_cauldron: u8,
    bits_brewing_count: u8,
    pending_mystical_bits: u32,
}

impl<'enchanted_output> BitMagicWriter<'enchanted_output> {
    /// Конструктор (ассоциированная функция)
    pub fn conjure_new(mystical_output_scroll: &'enchanted_output mut Vec<u8>) -> Self {
        Self {
            mystical_output_scroll,
            bit_accumulation_cauldron: 0,
            bits_brewing_count: 0,
            pending_mystical_bits: 0,
        }
    }

    /// Записывает один бит в выходной поток
    pub fn write_bit(&mut self, bit: u8) {
        self.bit_accumulation_cauldron = (self.bit_accumulation_cauldron << 1) | (bit & 1);
        self.bits_brewing_count += 1;

        if self.bits_brewing_count == 8 {
            self.mystical_output_scroll
                .push(self.bit_accumulation_cauldron);
            self.bit_accumulation_cauldron = 0;
            self.bits_brewing_count = 0;
        }
    }

    /// Выводит бит и обрабатывает любые ожидающие следующие биты
    pub fn output_bit(&mut self, bit: u8) {
        self.write_bit(bit);

        // Вывод ожидающих битов
        for _ in 0..self.pending_mystical_bits {
            self.write_bit(1 - bit);
        }
        self.pending_mystical_bits = 0;
    }

    /// Выводит бит и обрабатывает ожидающие биты
    pub fn bit_plus_follow(&mut self, bit: u8) {
        self.output_bit(bit);
        for _ in 0..self.pending_mystical_bits {
            self.output_bit(1 - bit);
        }
        self.pending_mystical_bits = 0;
    }

    /// Завершает сжатие и сбрасывает биты
    pub fn complete_compression_ritual(mut self) {
        // Финальные биты
        self.pending_mystical_bits += 1;
        if self.pending_mystical_bits > 0 {
            self.bit_plus_follow(1);
        }

        // Дополнение последнего байта
        if self.bits_brewing_count > 0 {
            self.bit_accumulation_cauldron <<= 8 - self.bits_brewing_count;
            self.mystical_output_scroll
                .push(self.bit_accumulation_cauldron);
        }
    }

    /// Основное заклинание арифметического кодирования 🎯
    /// Кодирует символ, сужая интервал (изменяемая ссылка на self)
    pub fn encode_mystical_symbol(
        &mut self,
        current_low: &mut u32,
        current_high: &mut u32,
        symbol_frequency_start: u32,
        symbol_frequency_end: u32,
        total_frequency_mass: u32,
    ) {
        let range = (*current_high as u64) - (*current_low as u64) + 1;

        *current_high = (*current_low as u64
            + (range * symbol_frequency_end as u64) / total_frequency_mass as u64
            - 1) as u32;
        *current_low = (*current_low as u64
            + (range * symbol_frequency_start as u64) / total_frequency_mass as u64)
            as u32;

        self.normalize(current_low, current_high);
    }

    /// Нормализует интервал арифметического кодирования во время кодирования
    pub fn normalize(&mut self, low: &mut u32, high: &mut u32) {
        loop {
            if *high < HALF {
                self.bit_plus_follow(0);
            } else if *low >= HALF {
                self.bit_plus_follow(1);
                *low -= HALF;
                *high -= HALF;
            } else if *low >= FIRST_QTR && *high < THIRD_QTR {
                self.pending_mystical_bits += 1;
                *low -= FIRST_QTR;
                *high -= FIRST_QTR;
            } else {
                break;
            }

            *low = 2 * *low;
            *high = 2 * *high + 1;
        }
    }
}

/// Читатель битовой магии - восстанавливает данные из сжатых потоков 🔮
pub struct BitMagicReader {
    compressed_mystical_scroll: Vec<u8>,
    byte_pos: usize,
    bit_pos: u8,
    interval_position_tracker: u32,
}

impl BitMagicReader {
    /// Конструктор (принимает владение Vec<u8>)
    pub fn conjure_from_scroll(compressed_mystical_scroll: Vec<u8>) -> Self {
        let mut mystical_reader = Self {
            compressed_mystical_scroll,
            byte_pos: 0,
            bit_pos: 0,
            interval_position_tracker: 0,
        };

        // Инициализация первыми 24 битами
        for _ in 0..24 {
            mystical_reader.interval_position_tracker = (mystical_reader.interval_position_tracker
                << 1)
                | (mystical_reader.read_bit() as u32);
        }

        mystical_reader
    }

    /// Читает один бит из потока
    pub fn read_bit(&mut self) -> u8 {
        if self.byte_pos >= self.compressed_mystical_scroll.len() {
            return 0;
        }

        let bit = (self.compressed_mystical_scroll[self.byte_pos] >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;

        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        bit
    }

    /// Декодирует целевое значение (возвращает вычисленное значение)
    pub fn decode_mystical_target(
        &self,
        total_frequency_mass: u32,
        current_low: u32,
        current_high: u32,
    ) -> u32 {
        let range = (current_high as u64) - (current_low as u64) + 1;
        (((self.interval_position_tracker as u64 - current_low as u64 + 1)
            * total_frequency_mass as u64
            - 1)
            / range) as u32
    }

    /// Обновляет интервалы (изменяемый метод)
    pub fn update_mystical_intervals(
        &mut self,
        current_low: &mut u32,
        current_high: &mut u32,
        symbol_frequency_start: u32,
        symbol_frequency_end: u32,
        total_frequency_mass: u32,
    ) {
        let range = (*current_high as u64) - (*current_low as u64) + 1;

        *current_high = (*current_low as u64
            + (range * symbol_frequency_end as u64) / total_frequency_mass as u64
            - 1) as u32;
        *current_low = (*current_low as u64
            + (range * symbol_frequency_start as u64) / total_frequency_mass as u64)
            as u32;

        self.normalize(current_low, current_high);
    }

    /// Нормализует интервал при декодировании
    pub fn normalize(&mut self, low: &mut u32, high: &mut u32) {
        loop {
            if *high < HALF {
                // Ничего не делать
            } else if *low >= HALF {
                self.interval_position_tracker -= HALF;
                *low -= HALF;
                *high -= HALF;
            } else if *low >= FIRST_QTR && *high < THIRD_QTR {
                self.interval_position_tracker -= FIRST_QTR;
                *low -= FIRST_QTR;
                *high -= FIRST_QTR;
            } else {
                break;
            }

            *low = 2 * *low;
            *high = 2 * *high + 1;
            self.interval_position_tracker =
                2 * self.interval_position_tracker + (self.read_bit() as u32);
        }
    }

    /// Показывает внутреннее состояние (инспекция)
    pub fn reveal_mystical_position(&self) -> u32 {
        self.interval_position_tracker
    }
}

/// Тесты битовых операций 🎯
#[cfg(test)]
mod mystical_bit_tests {
    use super::*;

    #[test]
    fn test_bit_writer_basic_functionality() {
        let mut output_scroll = Vec::new();
        let writer = BitMagicWriter::conjure_new(&mut output_scroll);

        writer.complete_compression_ritual();

        assert!(!output_scroll.is_empty());
    }

    #[test]
    fn test_bit_reader_initialization() {
        let test_data = vec![0xFF, 0x00, 0xFF, 0x00];
        let reader = BitMagicReader::conjure_from_scroll(test_data);

        let position = reader.reveal_mystical_position();
        assert!(position > 0); // Загрузка начальных битов при инициализации
    }
}
