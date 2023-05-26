mod args;
mod cb_image;

use args::Args;
use cb_image::{ImageDataErrors,find_imange_from_path,FloatingImage,standardise_size,combine_images};

/// 合并图片,返回成功的Result或自定义的异常
fn main() -> Result<(), ImageDataErrors> {
    //创建结构体args用于获取命令行参数
    let args = Args::new();
    //获取图片rgba及其格式(如png等)
    let (image_1, image_format_1) = find_imange_from_path(args.image_1)?;
    let (image_2, image_format_2) = find_imange_from_path(args.image_2)?;

    //只有两个图片格式相同才继续,否则抛出异常
    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    //将两个图片调整为其中最小的大小
    let (image_1, image_2) = standardise_size(image_1, image_2);
    //创建FloatingImage结构体,为了合并图片,并赋值长宽,合并后路径即大小
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    //合并图片
    let combined_data = combine_images(image_1, image_2);
    //
    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format_1,
    ) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())
    }
}


