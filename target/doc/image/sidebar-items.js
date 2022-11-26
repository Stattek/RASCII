window.SIDEBAR_ITEMS = {"enum":[["ColorType","An enumeration over supported color types and bit depths"],["DynamicImage","A Dynamic Image"],["ExtendedColorType","An enumeration of color types encountered in image formats."],["ImageFormat","An enumeration of supported image formats. Not all formats support both encoding and decoding."],["ImageOutputFormat","An enumeration of supported image formats for encoding."]],"fn":[["guess_format","Guess image format from memory block"],["image_dimensions","Read a tuple containing the (width, height) of the image located at the specified path. This is faster than fully loading the image and then getting its dimensions."],["load","Create a new image from a Reader."],["load_from_memory","Create a new image from a byte slice"],["load_from_memory_with_format","Create a new image from a byte slice"],["open","Open the image located at the path specified. The image’s format is determined from the path’s file extension."],["save_buffer","Saves the supplied buffer to a file at the path specified."],["save_buffer_with_format","Saves the supplied buffer to a file at the path specified in the specified format."],["write_buffer_with_format","Writes the supplied buffer to a writer in the specified format."]],"mod":[["buffer","Iterators and other auxiliary structure for the `ImageBuffer` type."],["codecs","Encoding and decoding for various image file formats."],["error","Contains detailed error representation."],["flat","Image representations for ffi."],["imageops","Image Processing Functions"],["io","Input and output of images."],["math","Mathematical helper functions and types."]],"struct":[["Delay","The delay of a frame relative to the previous one."],["Frame","A single animation frame"],["Frames","An implementation dependent iterator, reading the frames as requested"],["ImageBuffer","Generic image buffer"],["Luma","Grayscale colors."],["LumaA","Grayscale colors + alpha channel"],["Pixels","Immutable pixel iterator"],["Progress","Represents the progress of an image operation."],["Rgb","RGB colors."],["Rgba","RGB colors + alpha channel"],["SubImage","A View into another image"]],"trait":[["AnimationDecoder","AnimationDecoder trait"],["EncodableLayout","Types which are safe to treat as an immutable byte slice in a pixel layout for image encoding."],["GenericImage","A trait for manipulating images."],["GenericImageView","Trait to inspect an image."],["ImageDecoder","The trait that all decoders implement"],["ImageDecoderRect","Specialized image decoding not be supported by all formats"],["ImageEncoder","The trait all encoders implement"],["Pixel","A generalized pixel."],["PixelWithColorType","The pixel with an associated `ColorType`. Not all possible pixels represent one of the predefined `ColorType`s."],["Primitive","The type of each channel in a pixel. For example, this can be `u8`, `u16`, `f32`."]],"type":[["GrayAlphaImage","Sendable grayscale + alpha channel image buffer"],["GrayImage","Sendable grayscale image buffer"],["Rgb32FImage","An image buffer for 32-bit float RGB pixels, where the backing container is a flattened vector of floats."],["RgbImage","Sendable Rgb image buffer"],["Rgba32FImage","An image buffer for 32-bit float RGBA pixels, where the backing container is a flattened vector of floats."],["RgbaImage","Sendable Rgb + alpha channel image buffer"]]};