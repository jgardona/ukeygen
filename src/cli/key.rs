use clap::ValueEnum;
use lazy_static::lazy_static;
use rand::{
    distributions::Uniform,
    seq::SliceRandom,
    thread_rng, Rng,
};

lazy_static! {
    static ref SYMBOLS: Uniform<u8> = Uniform::new_inclusive(58, 64);
    static ref UPPERCASE: Uniform<u8> = Uniform::new_inclusive(65, 90);
    static ref LOWERCASE: Uniform<u8> = Uniform::new_inclusive(97, 122);
    static ref NUMERIC: Uniform<u8> = Uniform::new_inclusive(48, 57);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
pub enum KeySize {
    /// 8 characters keys
    Small,
    /// 16 characters keys
    Medium,
    /// 20 characters keys
    Large,
}

fn get_size(keysize: KeySize) -> usize {
    match keysize {
        KeySize::Small => 8,
        KeySize::Medium => 16,
        KeySize::Large => 20,
    }
}

fn collect_data(distr: &Uniform<u8>, size: usize) -> Vec<u8> {
    let rng = thread_rng();
    let data: Vec<u8> = rng.sample_iter(distr).take(size).collect();
    data
}

pub fn get_key(
    symbol: bool,
    uppercase: bool,
    lowercase: bool,
    numeric: bool,
    keysize: KeySize,
) -> String {
    let mut buffer = Vec::<u8>::new();
    let key_size = get_size(keysize);

    if symbol {
        let data = collect_data(&SYMBOLS, key_size);
        buffer.extend(data);
    }

    if uppercase {
        let data = collect_data(&UPPERCASE, key_size);
        buffer.extend(data);
    }

    if lowercase {
        let data = collect_data(&LOWERCASE, key_size);
        buffer.extend(data);
    }

    if numeric {
        let data = collect_data(&NUMERIC, key_size);
        buffer.extend(data);
    }

    let mut rng = thread_rng();
    let mut result: Vec<u8> = buffer
        .choose_multiple(&mut rng, key_size)
        .copied()
        .collect();

    result.shuffle(&mut rng);

    String::from_utf8_lossy(result.as_slice()).to_string()
}
