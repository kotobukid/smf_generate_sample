use std::fs;
use std::fs::File;
use std::io::{Write};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

fn detect_root(name: &str) -> Result<u8, String> {
    let mut adjust: i8 = 0;
    let mut root: i8 = -1;

    if name.len() > 1 {
        match name.chars().nth(1) {
            Some('#') => adjust = 1,
            Some('b') => adjust = -1,
            _ => ()
        }
    }

    match name.chars().nth(0) {
        Some('C') => root = 60,
        Some('D') => root = 62,
        Some('E') => root = 64,
        Some('F') => root = 65,
        Some('G') => root = 67,
        Some('A') => root = 69,
        Some('B') => root = 71,
        _ => ()
    }
    if root == -1 {
        return Err("note not detected".to_string());
    }

    Ok((root + adjust) as u8)
}

#[test]
fn test_detect_root() {
    assert_eq!(60, detect_root("C").unwrap());
    assert_eq!(61, detect_root("C#").unwrap());
    assert_eq!(61, detect_root("Db").unwrap());
    assert_eq!(62, detect_root("D").unwrap());
    assert_eq!(63, detect_root("D#").unwrap());
    assert_eq!(63, detect_root("Eb").unwrap());
    assert_eq!(70, detect_root("Bb").unwrap());
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn note_in_range(note: u8) -> u8 {
    if note > 72 {
        note_in_range(note - 12)
    } else if note < 60 {
        note_in_range(note + 12)
    } else {
        note
    }
}

fn note_to_chord(note: u8, chord_type: &str) -> Vec<u8> {
    match chord_type {
        "" => [note + 4, note + 7].to_vec(),
        "m" => [note + 3, note + 10].to_vec(),
        "7" => [note + 4, note + 7, note + 10].to_vec(),
        "M7" => [note + 4, note + 7, note + 11].to_vec(),
        "m7" => [note + 3, note + 7, note + 10].to_vec(),
        "mM7" => [note + 3, note + 7, note + 11].to_vec(),
        _ => [].to_vec()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match fs::create_dir("./output") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    let mut file = File::create("./output/chord.mid")?;

    macro_rules! one_bar_note_on {
        ( $t:expr, $x:expr ) => {
            for t in $t.iter() {
                file.write_all(&t.to_be_bytes())?;   // スタートTick
            }

            file.write_all(&144_u8.to_be_bytes())?;   // 90 ノートONは9スタート
            file.write_all(&$x.to_be_bytes())?;
            file.write_all(&100_u8.to_be_bytes())?;   // 64 ヴェロシティ
        };
    }

    macro_rules! one_bar_note_off {
        ( $t:expr, $x:expr ) => {
            for t in $t.iter() {
                file.write_all(&t.to_be_bytes())?;   // スタートTick
            }

            file.write_all(&128_u8.to_be_bytes())?;   // 80 ノートOFFは8スタート
            file.write_all(&$x.to_be_bytes())?;
            file.write_all(&100_u8.to_be_bytes())?;   // 64 ヴェロシティ
        };
    }

    macro_rules! chord_on {
        ($time:expr, $root:expr, $type:expr) => {
            let root:u8 = detect_root($root).unwrap();
            let notes: Vec<u8> = note_to_chord(root, $type);

            one_bar_note_on!($time, &note_in_range(root));
            for n in notes {
                one_bar_note_on!([&0_u8], &note_in_range(n));
            }

            one_bar_note_on!([&0_u8], root - 24);
            one_bar_note_on!([&0_u8], root - 12);
        }
    }

    macro_rules! chord_off {
        ($time:expr, $root:expr, $type:expr) => {
            let root:u8 = detect_root($root).unwrap();
            let notes: Vec<u8> = note_to_chord(root, $type);

            one_bar_note_off!($time, &note_in_range(root));
            for n in notes {
                one_bar_note_off!([&0_u8], &note_in_range(n));
            }

            one_bar_note_off!([&0_u8], root - 24);
            one_bar_note_off!([&0_u8], root - 12);
        }
    }

    macro_rules! write_num {
        ($i:expr) => {
            for i in $i.iter() {
                file.write_all(&i.to_be_bytes())?
            }
        }
    }

    macro_rules! write {
        ($b:expr) => {

            file.write_all(&$b.as_bytes())?;
        }
    }

    let mthd: &str = "MThd";

    write!(mthd);

    let buf2: i32 = 6;
    write_num!([buf2]);

    let file_format: i16 = 1;
    let track_amount: i16 = 2;
    let resolution: i16 = 960;
    write_num!([file_format, track_amount, resolution]);

    //　コンダクタートラック

    let start_track: &str = "MTrk";
    write!(start_track);

    let track_length: i32 = 23;
    write_num!([track_length]);

    // 00FFxx   を表現したいが今回は分割して記述
    let before_start_track_title: i16 = 255;
    let before_start_track_title2: i8 = 3;
    write_num!([before_start_track_title]);
    write_num!([before_start_track_title2]);


    let track_title: &str = "Track1";
    let track_title_length: i8 = track_title.as_bytes().len() as i8;
    write_num!([track_title_length]);
    write!(track_title);

    // Tick
    let tick: u8 = 135;  // 960 / 128
    let tick_b: u8 = 64;
    // 960 % 128
    write_num!([tick, tick_b]);

    // Tempo
    write_num!([0_u8, 255_u8, 81_u8, 03_u8]);
    // 00 FF 51 03(続く3バイトでテンポ情報を送る)
    write_num!([06_u8, 138_u8, 27_u8]);

    // 拍子
    write_num!([0_u8, 255_u8]);
    write_num!([88_u8, 4_u8, 4_u8, 2_u8, 26_u8, 8_u8]); // 58 04 04 02 18 08


    // End of Track
    let zero: u8 = 0;
    let ff: u8 = 255;
    let two_f: u8 = 47;
    write_num!([zero, ff, two_f, zero]);


    // 演奏トラック
    let start_track: &str = "MTrk";
    write!(start_track);

    // トラックの長さ
    let track_length: i32 = 43;
    write_num!([track_length]);

    // トラックタイトル（なし）
    let before_start_track_title_1: i16 = 255;
    // 00FFxx   を表現したいが今回は分割して記述
    write_num!([before_start_track_title_1]);
    let before_start_track_title1_2: i8 = 3;
    write_num!([before_start_track_title1_2]);
    write_num!([0_i8]);

    // ポート
    write_num!([0_u8, 255_u8, 33_u8, 1_u8, 0_u8]);   // 00 FF 21 01 00

    // リセットオールコントローラーCC#21
    write_num!([0_u8, 176_u8, 121_u8, 0_u8]);   // 00 B0(チャンネル1) 79 00

    // バンクセレクトMSB(CC#0)
    write_num!([0_u8, 176_u8, 0_u8, 0_u8]);   // 00 B0 00 00

    // バンクセレクトLSB(CC#32)
    write_num!([0_u8, 176_u8, 32_u8, 0_u8]);   // 00 B0 20 00

    // プログラムチェンジ
    write_num!([0_u8, 192_u8]);
    // C0
    write_num!([41_u8]);   // 28  1始まり

    // ボリューム
    write_num!([0_u8, 176_u8, 7_u8, 100_u8]);   // 00 B0 07 64 (ボリューム100)

    // C
    chord_on!([&0_u8], "C", "mM7");
    chord_off!([&158_u8, &0_u8], "C", "mM7");

    // F
    chord_on!([&0_u8], "F", "");
    chord_off!([&158_u8, &0_u8], "F", "");

    // Am
    chord_on!([&0_u8], "A", "m");
    chord_off!([&158_u8, &0_u8], "A", "m");

    // // G
    // chord_on!([&0_u8], "G", "major");
    // major_off!([&158_u8, &0_u8], "G", "major");

    // C
    chord_on!([&0_u8], "C", "m7");
    chord_off!([&158_u8, &0_u8], "C", "m7");

    // End of Track
    let zero: u8 = 0;
    let ff: u8 = 255;
    let two_f: u8 = 47;
    write_num!([zero, ff ,two_f, zero]);

    file.flush()?;
    println!("complete");
    Ok(())
}


