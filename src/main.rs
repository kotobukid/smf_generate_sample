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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    match fs::create_dir("./output") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("{}", "directory exists or created");

    let mut file = File::create("./output/f.mid")?;

    let buf: &str = "MThd";

    file.write_all(buf.as_bytes())?;

    let buf2: i32 = 6;
    file.write_all(&buf2.to_be_bytes())?;

    let file_format: i16 = 1;
    file.write_all(&file_format.to_be_bytes())?;

    let track_amount: i16 = 2;
    file.write_all(&track_amount.to_be_bytes())?;

    let resolution: i16 = 960;
    file.write_all(&resolution.to_be_bytes())?;

    //　コンダクタートラック

    let start_track: &str = "MTrk";
    file.write_all(start_track.as_bytes())?;

    let track_length: i32 = 23;
    file.write_all(&track_length.to_be_bytes())?;

    let before_start_track_title: i16 = 255;    // 00FFxx   を表現したいが今回は分割して記述
    file.write_all(&before_start_track_title.to_be_bytes())?;
    let before_start_track_title2: i8 = 3;
    file.write_all(&before_start_track_title2.to_be_bytes())?;


    let track_title: &str = "Track1";
    let track_title_length: i8 = track_title.as_bytes().len() as i8;
    file.write_all(&track_title_length.to_be_bytes())?;
    file.write_all(&track_title.as_bytes())?;

    // Tick
    let tick: u8 = 135;  // 960 / 128
    let tick_b: u8 = 64; // 960 % 128
    file.write_all(&tick.to_be_bytes())?;
    file.write_all(&tick_b.to_be_bytes())?;

    // Tempo
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&255_u8.to_be_bytes())?; // FF
    file.write_all(&81_u8.to_be_bytes())?;  // 51
    file.write_all(&03_u8.to_be_bytes())?;  // 03   続く3バイトでテンポ情報を送る
    file.write_all(&06_i8.to_be_bytes())?;
    file.write_all(&138_u8.to_be_bytes())?;
    file.write_all(&27_u8.to_be_bytes())?;

    // 拍子
    file.write_all(&0_i8.to_be_bytes())?;
    file.write_all(&255_u8.to_be_bytes())?;
    file.write_all(&88_u8.to_be_bytes())?;  // 58
    file.write_all(&4_u8.to_be_bytes())?; // 04
    file.write_all(&4_u8.to_be_bytes())?; // 04
    file.write_all(&2_u8.to_be_bytes())?; // 02
    file.write_all(&26_u8.to_be_bytes())?; // 18
    file.write_all(&8_u8.to_be_bytes())?; // 08


    // End of Track
    let zero: i8 = 0;
    let ff: u8 = 255;
    let two_f: i8 = 47;
    file.write_all(&zero.to_be_bytes())?;
    file.write_all(&ff.to_be_bytes())?;
    file.write_all(&two_f.to_be_bytes())?;
    file.write_all(&zero.to_be_bytes())?;


    // 演奏トラック
    let start_track: &str = "MTrk";
    file.write_all(start_track.as_bytes())?;

    // トラックの長さ
    let track_length: i32 = 43;
    file.write_all(&track_length.to_be_bytes())?;

    // トラックタイトル（なし）
    let before_start_track_title_1: i16 = 255;    // 00FFxx   を表現したいが今回は分割して記述
    file.write_all(&before_start_track_title_1.to_be_bytes())?;
    let before_start_track_title1_2: i8 = 3;
    file.write_all(&before_start_track_title1_2.to_be_bytes())?;
    file.write_all(&0_i8.to_be_bytes())?;

    // ポート
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&255_u8.to_be_bytes())?;   // ff
    file.write_all(&33_u8.to_be_bytes())?;   // 21
    file.write_all(&1_u8.to_be_bytes())?;   // 01
    file.write_all(&0_u8.to_be_bytes())?;   // 00

    // リセットオールコントローラーCC#21
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&176_u8.to_be_bytes())?;   // B0 （チャンネル1）
    file.write_all(&121_u8.to_be_bytes())?;   // 79
    file.write_all(&0_i8.to_be_bytes())?;   // 00

    // バンクセレクトMSB(CC#0)
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&176_u8.to_be_bytes())?;   // B0
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&0_i8.to_be_bytes())?;   // 00

    // バンクセレクトLSB(CC#32)
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&176_u8.to_be_bytes())?;   // B0
    file.write_all(&32_u8.to_be_bytes())?;   // 20
    file.write_all(&0_i8.to_be_bytes())?;   // 00

    // プログラムチェンジ
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&192_u8.to_be_bytes())?;   // C0
    file.write_all(&41_u8.to_be_bytes())?;   // 28  1始まり

    // ボリューム
    file.write_all(&0_i8.to_be_bytes())?;   // 00
    file.write_all(&176_u8.to_be_bytes())?;   // B0
    file.write_all(&7_i8.to_be_bytes())?;   // 07
    file.write_all(&100_u8.to_be_bytes())?;   // 64 ボリューム100

    // Cを鳴らす
    one_bar_note_on!([&0_u8], file, &60_u8);
    one_bar_note_on!([&0_u8], file, &64_u8);
    one_bar_note_on!([&0_u8], file, &67_u8);

    // 止める
    one_bar_note_off!([&158_u8, &0_u8], file, &60_u8);
    one_bar_note_off!([&0_u8], file, &64_u8);
    one_bar_note_off!([&0_u8], file, &67_u8);

    // F
    one_bar_note_on!([&0_u8], file, &65_u8);
    one_bar_note_on!([&0_u8], file, &69_u8);
    one_bar_note_on!([&0_u8], file, &72_u8);

    one_bar_note_off!([&158_u8, &0_u8], file, &65_u8);
    one_bar_note_off!([&0_u8], file, &69_u8);
    one_bar_note_off!([&0_u8], file, &72_u8);

    // G
    one_bar_note_on!([&0_u8], file, &67_u8);
    one_bar_note_on!([&0_u8], file, &71_u8);
    one_bar_note_on!([&0_u8], file, &74_u8);

    one_bar_note_off!([&158_u8, &0_u8], file, &67_u8);
    one_bar_note_off!([&0_u8], file, &71_u8);
    one_bar_note_off!([&0_u8], file, &74_u8);

    // C
    one_bar_note_on!([&0_u8], file, &60_u8);
    one_bar_note_on!([&0_u8], file, &64_u8);
    one_bar_note_on!([&0_u8], file, &67_u8);

    one_bar_note_off!([&158_u8, &0_u8], file, &60_u8);
    one_bar_note_off!([&0_u8], file, &64_u8);
    one_bar_note_off!([&0_u8], file, &67_u8);

    // End of Track
    let zero: i8 = 0;
    let ff: u8 = 255;
    let two_f: i8 = 47;
    file.write_all(&zero.to_be_bytes())?;
    file.write_all(&ff.to_be_bytes())?;
    file.write_all(&two_f.to_be_bytes())?;
    file.write_all(&zero.to_be_bytes())?;

    file.flush()?;
    Ok(())
}


