use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageError,
    ImageFormat,
};

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImaggeFromPath(std::io::Error),
    UnableToReadImaggeFromImage(String),
    UableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

/// `FloatingImage`结构体,包含合并图片信息
pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    //创建结构体并赋值宽高和路径,图片的大小
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        //try_into转换类型
        //with_capacity填充Vec容器大小
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    /// 将图片数据设置给data
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

/// 根据传入的[`String`]类型的路径,返回图像的rgba和图像格式
pub fn find_imange_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    //根据传进的路径进行文件读取并猜测格式
    match Reader::open(&path) {
        //读取到文件格式进入
        Ok(image_reader) => {
            //format获取确定的格式
            if let Some(image_format) = image_reader.format() {
                //获取到确定的格式
                //decode读取图片
                match image_reader.decode() {
                    //成功返回一个rgba和图像格式
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UableToDecodeImage(e)),
                }
            } else {
                return Err(ImageDataErrors::UnableToReadImaggeFromImage(path));
            }
        }
        Err(e) => Err(ImageDataErrors::UnableToReadImaggeFromPath(e)),
    }
}

/// 根据传入两个图片的长宽([`u32`]元组),返回最小面积的图片长宽([`u32`]元组)
fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

/// 将两个图片调整为其中最小的那个的大小并返回([`DynamicImage`]元组)
pub fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    //dimension用于获取图片长宽
    //get_smallest_dimensions用于获取最小长宽
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());

    //将两个图片中大的调整为小的长宽
    if image_2.dimensions() == (width, height) {
        //resize_exact用指定的过滤器算法调整大小
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

/// 对图像进行处理后,获得合并的图片,返回[`Vec<u8>`]
pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    //to_rgba8返回此图像的副本为rgba图像
    //into_vec将图像基础数据最为拥有的缓冲区返回
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

/// 将两个图片合并,返回[`Vec<u8>`]
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    //如果vec_1.len()==5, [0,0,0,0,0]
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            //在i取模8==0时拼接图片的一部分
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            //其余部分拼接另一个图片
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

/// 返回[`Vec<u8>`]用于替换图片的一部分
fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("指针越界"),
        };
        rgba.push(val);
    }
    rgba
}