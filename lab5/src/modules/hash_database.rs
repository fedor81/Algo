use std::{
    fs::{File, OpenOptions},
    hash::{DefaultHasher, Hash, Hasher},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

// ### TODO: Возможные оптимизации:
// - Сжатие строк
// - Кэширование
// - Многопоточность
// - Разбиение файла данных на несколько
// - Компактификация: Периодически перезаписывать файл, удаляя "дыры" от удаленных записей
// - Дедупликация ключей/значений (Хранить уникальные строки в отдельном файле. В основной таблице использовать ссылки (u32 или u64))

// TODO: База не умеет удалять мертвые записи
/// База данных в формате ключ - значение для хранения строк не превосходящих u16
#[derive(Debug)]
pub struct HashDatabase {
    file: File,
    addresses: Vec<Option<u64>>,
    capacity: u64,
}

// TODO: Можно связать Entry с базой с помощью и при вызове Drop обновлять файл
#[derive(Debug)]
struct Entry {
    is_deleted: bool,
    // key_size: u16,
    // value_size: u16,
    next: Option<u64>,
    key: String,
    value: String,
}

impl HashDatabase {
    pub fn new<P: AsRef<Path>>(path: P, capacity: u64) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Self {
            file,
            addresses: vec![None; capacity as usize],
            capacity,
        })
    }

    fn get_bucket(&self, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % self.capacity
    }

    pub fn add(&mut self, key: String, value: String) -> io::Result<()> {
        let bucket = self.get_bucket(&key) as usize;

        if self.check_exists(&key, bucket) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "key already exists",
            ));
        }

        let next = self.addresses[bucket];
        self.write_entry(&Entry::new(key, value, next), bucket)?;
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> io::Result<()> {
        let bucket = self.get_bucket(&key) as usize;
        let mut next = self.addresses[bucket];

        while let Some(address) = next {
            let entry = self.read_entry(address)?;
            if entry.key == key && !entry.is_deleted {
                self.mark_deleted(address)?;
                return Ok(());
            }
            next = entry.next;
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "key does not exists",
        ))
    }

    pub fn update(&mut self, key: &str, value: String) -> io::Result<()> {
        let bucket = self.get_bucket(&key) as usize;
        let mut next = self.addresses[bucket];

        while let Some(address) = next {
            let mut entry = self.read_entry(address)?;
            if entry.key == key && !entry.is_deleted {
                entry.value = value;
                self.mark_deleted(address)?;
                self.write_entry(&entry, bucket)?;
                return Ok(());
            }
            next = entry.next;
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "key does not exists",
        ))
    }

    pub fn get(&mut self, key: &str) -> io::Result<Option<(String, String)>> {
        let bucket = self.get_bucket(&key) as usize;

        let mut offset = match self.addresses[bucket] {
            Some(offset) => offset,
            None => return Ok(None),
        };

        loop {
            let entry = self.read_entry(offset)?;
            if entry.key == key && !entry.is_deleted {
                return Ok(Some((entry.key, entry.value)));
            }
            match entry.next {
                Some(next) => offset = next,
                None => return Ok(None),
            }
        }
    }

    fn mark_deleted(&mut self, address: u64) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(address))?;
        self.file.write_all(&[1])?;
        Ok(())
    }

    fn write_entry(&mut self, entry: &Entry, bucket: usize) -> io::Result<()> {
        let bytes = Self::serialize_entry(&entry);
        let offset = self.file.seek(SeekFrom::End(0))?; // Записываем в конец файла
        self.file.write_all(&bytes)?;
        self.addresses[bucket] = Some(offset);
        Ok(())
    }

    fn check_exists(&mut self, key: &str, bucket: usize) -> bool {
        let mut next = self.addresses[bucket];

        while let Some(address) = next {
            let entry = self.read_entry(address).unwrap();
            if entry.key == key && !entry.is_deleted {
                return true;
            }
            next = entry.next;
        }

        false
    }

    fn read_entry(&mut self, offset: u64) -> std::io::Result<Entry> {
        self.file.seek(SeekFrom::Start(offset))?;

        let mut buf = [0; 1]; // Читаем is_deleted (1 байт)
        self.file.read_exact(&mut buf)?;
        let is_deleted = buf[0] == 1;

        let mut buf = [0; 2]; // Читаем key_size (2 байта)
        self.file.read_exact(&mut buf)?;
        let key_size = u16::from_le_bytes(buf);

        self.file.read_exact(&mut buf)?; // Читаем value_size (2 байта)
        let value_size = u16::from_le_bytes(buf);

        // Читаем next (8 байт)
        let mut next_bytes = [0; 8];
        self.file.read_exact(&mut next_bytes)?;
        let next = match u64::from_le_bytes(next_bytes) {
            0 => None,
            val => Some(val),
        };

        let mut key_buf = vec![0; key_size as usize]; // Читаем key
        self.file.read_exact(&mut key_buf)?;
        let key = String::from_utf8(key_buf).unwrap();

        let mut value_buf = vec![0; value_size as usize]; // Читаем value
        self.file.read_exact(&mut value_buf)?;
        let value = String::from_utf8(value_buf).unwrap();

        Ok(Entry {
            key,
            value,
            next,
            is_deleted,
        })
    }

    fn serialize_entry(entry: &Entry) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Удалена - 1 байт
        match entry.is_deleted {
            true => bytes.push(1),
            false => bytes.push(0),
        }

        // Длина ключа и значения
        bytes.extend((entry.key.len() as u16).to_le_bytes()); // 2 Байта
        bytes.extend((entry.value.len() as u16).to_le_bytes()); // 2 Байта

        // Записываем next (8 байт)
        match entry.next {
            Some(offset) => bytes.extend(&offset.to_le_bytes()),
            None => bytes.extend(&0u64.to_le_bytes()),
        }

        // Записываем key, value
        bytes.extend(entry.key.as_bytes());
        bytes.extend(entry.value.as_bytes());
        bytes
    }
}

impl Entry {
    pub fn new(key: String, value: String, next: Option<u64>) -> Self {
        Self {
            is_deleted: false,
            next,
            key,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    struct TestDatabase {
        inner: HashDatabase,
        path: PathBuf,
    }

    impl TestDatabase {
        fn new(path: &str, capacity: u64) -> std::io::Result<Self> {
            Ok(Self {
                inner: HashDatabase::new(path, capacity)?,
                path: PathBuf::from(path),
            })
        }
    }

    impl Drop for TestDatabase {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    #[test]
    #[ignore]
    fn test_hash_database() {
        let path = "test.db";
        let mut db = TestDatabase::new(path, 10).unwrap();

        db.inner
            .add("key1".to_string(), "value1".to_string())
            .unwrap();
        assert_eq!(
            db.inner.get("key1").unwrap(),
            Some(("key1".to_string(), "value1".to_string()))
        );

        db.inner
            .add("key2".to_string(), "value2".to_string())
            .unwrap();
        db.inner
            .add("key3".to_string(), "value3".to_string())
            .unwrap();

        assert_eq!(
            db.inner.get("key1").unwrap(),
            Some(("key1".to_string(), "value1".to_string()))
        );
        assert_eq!(
            db.inner.get("key2").unwrap(),
            Some(("key2".to_string(), "value2".to_string()))
        );
        assert_eq!(
            db.inner.get("key3").unwrap(),
            Some(("key3".to_string(), "value3".to_string()))
        );

        db.inner.remove("key2").unwrap();

        assert_eq!(db.inner.get("key2").unwrap(), None);

        db.inner.update("key1", "new_value1".to_string()).unwrap();

        assert_eq!(
            db.inner.get("key1").unwrap(),
            Some(("key1".to_string(), "new_value1".to_string()))
        );
    }
}
