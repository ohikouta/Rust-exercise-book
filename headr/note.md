# 4章 headコマンド

## 4.1 headの動作

## 4.2 プロジェクトの作成

### 4.2.1 文字列を数値に変換する機能の単体テスト

### 4.2.2 文字列をエラーに変換する

- `cargo test dies`で3つのテストを通すことがゴール
- `tests/cli.rs`に実装されているテストコードを読み、パスするように`lib.rs`を編集する
- 実装範囲が広いので難しい

### 4.2.3 引数の定義

- サンプルコードが載っているので、自分で書いたコードと見比べることができる
- `parse_positive_int`関数で`lines`と`bytes`を検証している部分については、コードについてデータ型の推移まで細かく説明されているので理解しやすい
- `.unwrap()`はNone/Errの場合で即panicするので、安全に保証できる時だけ使うようにする
- 処理としては簡潔だが、これを`lib.rs`にそのまま書いても`cargo test dies`の3つのテストは通らない。通すためには、`cli.rs`に書かれている想定出力に合わせる作業が必要

### 4.2.4 入力ファイルの処理

### 4.2.5 バイトと文字の読み込み

- `head`は有効でないUTF-8の文字列を受け取った場合、Unicodeに変換できず特殊文字を出力するので、`headr`でも実装する。
- テスト（`cli.rs`）を全て通すためにはどのように実装する必要があるのかを考えると進めやすい。（テスト駆動開発）

#### 1. `cli.rs`のテスト項目を確認（[`cli.rs`のテスト項目まとめ](#clirsのテスト項目)）

`cli.rs`には計57件のテストが存在し、現時点でパスしているのは以下の4件のみである。

- `dies_bad_lines`
- `dies_bad_bytes`
- `dies_bytes_and_lines`
- `skips_bad_file`

[残りの53件のテスト](####clirsのテスト項目)にパスできるように、`lib.rs`の`run()`を改修していく。
以下の改修後コードは独自で実装しているので、テキストの[4.3解答例]()とは内容が異なる。

改修前コード

```rust
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}
```

改修後コード

```rust
pub fn run(config: Config) -> MyResult<()> {
    let multiple = config.files.len() > 1;

    if let Some(n) = config.bytes {
        for (i, filename) in config.files.iter().enumerate() {
            if multiple {
                if i > 0 { print!("\n") }
                print!("==> {filename} <==\n");
            }

            match open(filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(mut reader ) => {
                    let mut buffer = vec![0; n];
                    let bytes_read = reader.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }
            }
        }
    } else {
        for (i, filename) in config.files.iter().enumerate() {
            if multiple {
                if i > 0 { print!("\n"); }
                print!("==> {filename} <==\n");
            }

            match open(filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(mut reader) => {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = reader.read_line(&mut line)?;
                        if bytes == 0 { break; }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
        
    }

    Ok(())
}
```

## 補足

### cli.rsのテスト項目

#### 入力ファイルの中身が空であるケース

| テスト名 | 期待挙動 |
| --- | --- |
| `empty` | 空ファイルを指定したときに出力が空になる（デフォルトは先頭10行だが空なので何も出ない） |
| `empty_n2` | `-n 2` 指定でも空出力になる |
| `empty_n4` | `-n 4` 指定でも空出力になる |
| `empty_c2` | `-c 2` 指定でも空出力になる |
| `empty_c4` | `-c 4` 指定でも空出力になる |

#### 入力ファイルの中身が1行のテキストであるケース

```txt
Öne line, four words.

```

| テスト名 | 期待挙動 |
| --- | --- |
| `one` | デフォルト（`-n`なし）で先頭10行を出すが、1行しかないので全文出力 |
| `one_n2` | `-n 2` でも1行だけ出力される |
| `one_n4` | `-n 4` でも1行だけ出力される |
| `one_c1` | 先頭1バイト出力（UTF-8の途中で切れるため文字化けになる） |
| `one_c2` | 先頭2バイト出力（先頭の`Ö`が正しく出る） |
| `one_c4` | 先頭4バイト出力（`Ö`＋次の文字まで） |
| `one_stdin` | stdin入力で `one` と同じ結果（全文出力） |
| `one_n2_stdin` | stdin入力で `-n 2` の結果 |
| `one_n4_stdin` | stdin入力で `-n 4` の結果 |
| `one_c1_stdin` | stdin入力で `-c 1` の結果 |
| `one_c2_stdin` | stdin入力で `-c 2` の結果 |
| `one_c4_stdin` | stdin入力で `-c 4` の結果 |

#### 入力ファイルの中身が2行のテキストであるケース

```txt
Two lines.
Four words.

```

| テスト名 | 期待挙動 |
| --- | --- |
| `two` | デフォルトで先頭10行を出すが、2行しかないので全文出力 |
| `two_n2` | `-n 2` で2行出力される |
| `two_n4` | `-n 4` でも2行だけ出力される |
| `two_c2` | 先頭2バイト出力（ASCIIなので2文字相当） |
| `two_c4` | 先頭4バイト出力（ASCIIなので4文字相当） |
| `two_stdin` | stdin入力で `two` と同じ結果（全文出力） |
| `two_n2_stdin` | stdin入力で `-n 2` の結果 |
| `two_n4_stdin` | stdin入力で `-n 4` の結果 |
| `two_c2_stdin` | stdin入力で `-c 2` の結果 |
| `two_c4_stdin` | stdin入力で `-c 4` の結果 |

※ASCIIは1文字=1バイト

#### 入力ファイルの中身が3行のテキストであるケース

```txt
Three
lines,
four words.

```

| テスト名 | 期待挙動 |
| --- | --- |
| `three` | デフォルトで先頭10行を出すが、3行しかないので全文出力（CRLF含む） |
| `three_n2` | `-n 2` で2行出力される |
| `three_n4` | `-n 4` でも3行だけ出力される |
| `three_c2` | 先頭2バイト出力（ASCIIなので2文字相当） |
| `three_c4` | 先頭4バイト出力（ASCIIなので4文字相当） |
| `three_stdin` | stdin入力で `three` と同じ結果（全文出力） |
| `three_n2_stdin` | stdin入力で `-n 2` の結果 |
| `three_n4_stdin` | stdin入力で `-n 4` の結果 |
| `three_c2_stdin` | stdin入力で `-c 2` の結果 |
| `three_c4_stdin` | stdin入力で `-c 4` の結果 |

※CRLF:`\r\n`windowsで一般的に使われる改行

#### 入力ファイルの中身が12行のテキストであるケース

```txt
one
two
three
four
five
six
seven
eight
nine
ten
eleven
twelve

```

| テスト名 | 期待挙動 |
| --- | --- |
| `twelve` | デフォルトで先頭10行を出力（12行あるので10行まで） |
| `twelve_n2` | `-n 2` で2行出力 |
| `twelve_n4` | `-n 4` で4行出力 |
| `twelve_c2` | 先頭2バイト出力（ASCIIなので2文字相当） |
| `twelve_c4` | 先頭4バイト出力（ASCIIなので4文字相当） |
| `twelve_stdin` | stdin入力で `twelve` と同じ結果 |
| `twelve_n2_stdin` | stdin入力で `-n 2` の結果 |
| `twelve_n4_stdin` | stdin入力で `-n 4` の結果 |
| `twelve_c2_stdin` | stdin入力で `-c 2` の結果 |
| `twelve_c4_stdin` | stdin入力で `-c 4` の結果 |

#### 複数ファイルが指定されるケース

| テスト名 | 期待挙動 |
| --- | --- |
| `multiple_files` | 複数ファイル指定でデフォルト（先頭10行）を順番に出力。ファイルごとのヘッダ `==> file <==` 付き。 |
| `multiple_files_n2` | 複数ファイル指定 + `-n 2`。各ファイル先頭2行、ヘッダ付き。 |
| `multiple_files_n4` | 複数ファイル指定 + `-n 4`。各ファイル先頭4行、ヘッダ付き。 |
| `multiple_files_c1` | 複数ファイル指定 + `-c 1`。各ファイル先頭1バイト、ヘッダ付き。 |
| `multiple_files_c2` | 複数ファイル指定 + `-c 2`。各ファイル先頭2バイト、ヘッダ付き。 |
| `multiple_files_c4` | 複数ファイル指定 + `-c 4`。各ファイル先頭4バイト、ヘッダ付き。 |

#### 2つのヘルパー関数（`run` `run_stdin`）

##### `run`

```rust
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    let expected = String::from_utf8_lossy(&buffer);

    let output = Command::cargo_bin(name: PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}
```

- 文字列スライスの配列への参照とファイルパスの参照を引数にとる
- 返り値`Result<()>`は成功なら`Ok(())`、失敗なら`Err(e)`を返すことを意味する
- `String::from_utf8_lossy(&buffer)`はバイト列を文字列として扱えるように変換する。※ UTF-8に変換できない場合にエラーを出さずに特殊文字に変換してくれる。
- `assert!(output.status.success());`で正常終了したかどうかを確認し、`assert_eq!(String::from_utf8_lossy(&output.stdout), expected);`で出力が期待通りであるかどうかを確認している。

##### `run_stdin`

```rust
fn run_stdin(
    args: &[&str],
    input_file: &str,
    expected_file: &str,
) -> Result<()> {
    let mut file = File::open(expected_file);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}
```

- `input_file`の内容をユーザーの標準入力として仮定する。
- `.write_stdin(input)`で起動したプロセスのstdinに文字列を流し込んでいる。
