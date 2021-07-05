struct Image
{
    width: u32,
    height: u32,
    data: Vec<u32>
}

impl Image {
    fn load(path: String) -> Result<Image, &'static str>
    {

        return Err("Can't load image")
    }
    
}
