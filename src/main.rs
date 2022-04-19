use std::fs;
use std::fs::File;
use std::io::{Write};

macro_rules! one_bar_note_on {
    ( $t:expr, $f:expr, $x:expr ) => {
        for t in $t.iter() {
            $f.write_all(&t.to_be_bytes())?;   // スタートTick
        }

        $f.write_all(&144_u8.to_be_bytes())?;   // 90 ノートONは9スタート
        $f.write_all(&$x.to_be_bytes())?;
        $f.write_all(&100_u8.to_be_bytes())?;   // 64 ヴェロシティ
    };
}

macro_rules! one_bar_note_off {
    ( $t:expr, $f:expr, $x:expr ) => {
        for t in $t.iter() {
            $f.write_all(&t.to_be_bytes())?;   // スタートTick
        }

        $f.write_all(&128_u8.to_be_bytes())?;   // 80 ノートOFFは8スタート
        $f.write_all(&$x.to_be_bytes())?;
        $f.write_all(&100_u8.to_be_bytes())?;   // 64 ヴェロシティ
    };
}

macro_rules! write_major_on {
    ( $t:expr , $f:expr, $root:expr) => {
        one_bar_note_on!($t, $f, $root);
        one_bar_note_on!([&0_u8], $f, $root + 4);
        one_bar_note_on!([&0_u8], $f, $root + 7);
    }
}
macro_rules! write_major_off {
    ( $t:expr , $f:expr, $root:expr) => {
        one_bar_note_off!($t, $f, $root);
        one_bar_note_off!([&0_u8], $f, $root + 4);
        one_bar_note_off!([&0_u8], $f, $root + 7);
    }
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    match fs::create_dir("./output") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("{}", "directory exists or created");

    let mut file = File::create("./output/f.mid")?;

    macro_rules! write_i16 {
        ($i:expr) => {
            file.write_all(&$i.to_be_bytes())?;
        }
    }

    macro_rules! write {
        ($b:expr) => {

            file.write_all(&$b.as_bytes())?;
            // match type_of(&$b) {
            //     i16 => file.write_all(&$b.to_be_bytes())?,
            //     _ => (),
            // }
        }
    }

    let mthd: &str = "MThd";

    write!(mthd);

    let buf2: i32 = 6;
    write_i16!(buf2);

    let file_format: i16 = 1;
    write_i16!(file_format);

    let track_amount: i16 = 2;
    write_i16!(track_amount);

    let resolution: i16 = 960;
    write_i16!(resolution);

    //　コンダクタートラック

    let start_track: &str = "MTrk";
    write!(start_track);

    let track_length: i32 = 23;
    write_i16!(track_length);

    let before_start_track_title: i16 = 255;
    // 00FFxx   を表現したいが今回は分割して記述
    write_i16!(before_start_track_title);
    let before_start_track_title2: i8 = 3;
    write_i16!(before_start_track_title2);


    let track_title: &str = "Track1";
    let track_title_length: i8 = track_title.as_bytes().len() as i8;
    write_i16!(track_title_length);
    write!(track_title);

    // Tick
    let tick: u8 = 135;  // 960 / 128
    let tick_b: u8 = 64;
    // 960 % 128
    write_i16!(tick);
    write_i16!(tick_b);

    // Tempo
    write_i16!(0_i8);
    // 00
    write_i16!(255_u8);
    // FF
    write_i16!(81_u8);
    // 51
    write_i16!(03_u8);
    // 03   続く3バイトでテンポ情報を送る
    write_i16!(06_i8);
    write_i16!(138_u8);
    write_i16!(27_u8);

    // 拍子
    write_i16!(0_i8);
    write_i16!(255_u8);
    write_i16!(88_u8);
    // 58
    write_i16!(4_u8);
    // 04
    write_i16!(4_u8);
    // 04
    write_i16!(2_u8);
    // 02
    write_i16!(26_u8);
    // 18
    write_i16!(8_u8); // 08


    // End of Track
    let zero: i8 = 0;
    let ff: u8 = 255;
    let two_f: i8 = 47;
    write_i16!(zero);
    write_i16!(ff);
    write_i16!(two_f);
    write_i16!(zero);


    // 演奏トラック
    let start_track: &str = "MTrk";
    file.write_all(start_track.as_bytes())?;

    // トラックの長さ
    let track_length: i32 = 43;
    write_i16!(track_length);

    // トラックタイトル（なし）
    let before_start_track_title_1: i16 = 255;
    // 00FFxx   を表現したいが今回は分割して記述
    write_i16!(before_start_track_title_1);
    let before_start_track_title1_2: i8 = 3;
    write_i16!(before_start_track_title1_2);
    write_i16!(0_i8);

    // ポート
    write_i16!(0_i8);
    // 00
    write_i16!(255_u8);
    // ff
    write_i16!(33_u8);
    // 21
    write_i16!(1_u8);
    // 01
    write_i16!(0_u8);   // 00

    // リセットオールコントローラーCC#21
    write_i16!(0_i8);
    // 00
    write_i16!(176_u8);
    // B0 （チャンネル1）
    write_i16!(121_u8);
    // 79
    write_i16!(0_i8);   // 00

    // バンクセレクトMSB(CC#0)
    write_i16!(0_i8);
    // 00
    write_i16!(176_u8);
    // B0
    write_i16!(0_i8);
    // 00
    write_i16!(0_i8);   // 00

    // バンクセレクトLSB(CC#32)
    write_i16!(0_i8);
    // 00
    write_i16!(176_u8);
    // B0
    write_i16!(32_u8);
    // 20
    write_i16!(0_i8);   // 00

    // プログラムチェンジ
    write_i16!(0_i8);
    // 00
    write_i16!(192_u8);
    // C0
    write_i16!(41_u8);   // 28  1始まり

    // ボリューム
    write_i16!(0_i8);
    // 00
    write_i16!(176_u8);
    // B0
    write_i16!(7_i8);
    // 07
    write_i16!(100_u8);   // 64 ボリューム100

    // Cを鳴らす
    write_major_on!([&0_u8], file, &60_u8);

    // 止める
    write_major_off!([&158_u8, &0_u8], file, &60_u8);

    // F
    write_major_on!([&0_u8], file, &65_u8);
    write_major_off!([&158_u8, &0_u8], file, &65_u8);

    // G
    write_major_on!([&0_u8], file, &67_u8);
    write_major_off!([&158_u8, &0_u8], file, &67_u8);

    // C
    write_major_on!([&0_u8], file, &60_u8);
    write_major_off!([&158_u8, &0_u8], file, &60_u8);

    // End of Track
    let zero: i8 = 0;
    let ff: u8 = 255;
    let two_f: i8 = 47;
    write_i16!(zero);
    write_i16!(ff);
    write_i16!(two_f);
    write_i16!(zero);

    file.flush()?;
    println!("complete");
    Ok(())
}


