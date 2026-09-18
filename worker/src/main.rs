use ffmpeg_next as ffmpeg;

fn main() -> Result<(), ffmpeg::Error> {
    // инициализация FFmpeg
    ffmpeg::init()?;

    // Открываем видео

    let video_path = "NVR_ch1_main_20241230100951_20241230101209.mp4";

    let input = ffmpeg::format::input(video_path)?;




    // // Инициализируем FFmpeg
    // ffmpeg::init()?;
    //
    // // Открываем видео
    // let input = ffmpeg::format::input("NVR_ch1_main_20241230100951_20241230101209.mp4")?;
    //
    // // Ищем видеопоток
    // let stream = input
    //     .streams()
    //     .best(ffmpeg::media::Type::Video)
    //     .ok_or(ffmpeg::Error::StreamNotFound)?;
    //
    // println!("stream index: {}", stream.index());

    Ok(())
}
