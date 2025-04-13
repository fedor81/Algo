pub fn run() {
    let config = input();

    for k in solve(config) {
        println!("{}", k);
    }
}

fn solve(config: Config) -> Vec<usize> {
    let p: u64 = 911382629; // Число взаимно простое с u32 или u64 и большее config.colors
    let mut powers = vec![1u64];

    // Подсчет степеней
    for i in 1..config.count() {
        powers.push(powers[i - 1].wrapping_mul(p));
    }

    let mut hashes = vec![config.tiles[0] as u64];
    let mut rev_hashes = vec![config.tiles[config.count() - 1] as u64];

    // Подсчет хэшей
    for i in 1..config.count() {
        let tile = config.tiles[i] as u64;
        hashes.push(hashes[i - 1].wrapping_add(tile.wrapping_mul(powers[i])));
    }

    // Подсчет хэшей для инвертированной строки
    for i in 1..config.count() {
        let tile = config.tiles[config.count() - i - 1] as u64;
        rev_hashes.push(rev_hashes[i - 1].wrapping_add(tile.wrapping_mul(powers[i])));
    }

    rev_hashes.reverse(); // Инвертируем для удобства индексации
    let mut tiles_count = vec![config.count()];

    for end_str in 0..(config.count() / 2) {
        let end_rev_str = 2 * end_str + 1;
        let mut hash_str = hashes[end_str];
        let hash_rev_str = rev_hashes[end_str + 1];

        hash_str = powers[config.count() - end_rev_str - 1].wrapping_mul(hash_str);
        hash_str =
            hash_str.wrapping_add(rev_hashes.get(end_rev_str + 1).cloned().unwrap_or_default());

        if hash_str == hash_rev_str {
            tiles_count.push(config.count() - end_str - 1);
        }
    }

    tiles_count
}

fn input() -> Config {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut config = Config {
        colors: 0,
        tiles: Vec::new(),
    };

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let count = iter.next().unwrap().parse().unwrap();
    config.colors = iter.next().unwrap().parse().unwrap();

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        config.tiles.push(buf.trim().parse().unwrap());
    }

    config
}

struct Config {
    colors: u16,
    tiles: Vec<u16>,
}

impl Config {
    fn new(colors: u16, tiles: Vec<u16>) -> Self {
        Self { colors, tiles }
    }

    fn count(&self) -> usize {
        self.tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task10() {
        assert_eq!(solve(Config::new(2, vec![1, 1, 2])), vec![3, 2]);
        assert_eq!(solve(Config::new(2, vec![1, 1, 2, 2, 1, 1])), vec![6, 5, 3]);
    }

    #[test]
    fn bin_tests() {
        assert_eq!(solve(Config::new(2, vec![1, 2])), vec![2]);
        assert_eq!(solve(Config::new(2, vec![2, 1, 2])), vec![3]);
        assert_eq!(solve(Config::new(2, vec![1, 2, 2])), vec![3]);
        assert_eq!(solve(Config::new(2, vec![2, 2, 1])), vec![3, 2]);
        assert_eq!(solve(Config::new(2, vec![1, 2, 2, 1])), vec![4, 2]);
        assert_eq!(solve(Config::new(2, vec![2, 1, 2, 1])), vec![4]);
        assert_eq!(solve(Config::new(2, vec![2, 2, 1, 1])), vec![4, 3]);
        assert_eq!(solve(Config::new(2, vec![1, 2, 2, 1, 1])), vec![5, 3]);
        assert_eq!(solve(Config::new(2, vec![2, 1, 2, 1, 1])), vec![5]);
        assert_eq!(solve(Config::new(2, vec![2, 2, 1, 1, 1])), vec![5, 4]);
        assert_eq!(solve(Config::new(2, vec![2, 2, 2, 1, 1])), vec![5, 4]);
    }

    #[test]
    fn triple_tests() {
        assert_eq!(solve(Config::new(3, vec![1, 2, 3])), vec![3]);
        assert_eq!(solve(Config::new(3, vec![3, 2, 1])), vec![3]);
        assert_eq!(solve(Config::new(3, vec![3, 2, 1, 1])), vec![4]);
        assert_eq!(solve(Config::new(3, vec![1, 1, 3, 2, 2])), vec![5, 4]);
        assert_eq!(solve(Config::new(3, vec![3, 2, 1, 1, 2, 3])), vec![6, 3]);
        assert_eq!(solve(Config::new(3, vec![3, 2, 2, 1, 1, 2])), vec![6]);
        assert_eq!(solve(Config::new(3, vec![3, 1, 1, 2, 2, 2])), vec![6]);
        assert_eq!(solve(Config::new(3, vec![3, 1, 1, 3, 2, 2])), vec![6, 4]);
    }

    #[test]
    fn basic_tests() {
        assert_eq!(solve(Config::new(2, vec![1])), vec![1]);
        assert_eq!(solve(Config::new(2, vec![1, 1])), vec![2, 1]);
        assert_eq!(solve(Config::new(2, vec![1, 1, 1])), vec![3, 2]);
        assert_eq!(solve(Config::new(2, vec![1, 1, 1, 1])), vec![4, 3, 2]);
        assert_eq!(solve(Config::new(2, vec![1, 1, 1, 1, 1])), vec![5, 4, 3]);
        assert_eq!(
            solve(Config::new(2, vec![1, 1, 1, 1, 1, 1])),
            vec![6, 5, 4, 3]
        );
        assert_eq!(
            solve(Config::new(2, vec![1, 1, 1, 1, 1, 1, 1])),
            vec![7, 6, 5, 4]
        );
        assert_eq!(
            solve(Config::new(2, vec![1, 1, 1, 1, 1, 1, 1, 1])),
            vec![8, 7, 6, 5, 4]
        );
    }
}
