use std::fs::*;
use std::io::*;

/// 型名を出力
pub fn typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

/// ファイルに内容を一気に書き出す
pub fn write_file(path: &str,content: &str) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
