extern crate immeta;

use immeta::{Dimensions, load_from_buf};
use immeta::formats::{png, gif, jpeg};
use immeta::markers::{Png, Gif, Jpeg, Webp};

const OWLET_DIM: Dimensions = Dimensions {
    width: 1280,
    height: 857
};

const DROP_DIM: Dimensions = Dimensions {
    width: 238,
    height: 212
};

const CHERRY_DIM: Dimensions = Dimensions {
    width: 1024,
    height: 772
};

#[test]
fn test_jpeg() {
    let md = immeta::load_from_file("tests/images/owlet.jpg").unwrap();

    assert_eq!(md.mime_type(), "image/jpeg");
    assert_eq!(md.dimensions(), OWLET_DIM);

    // let md = Jpeg::from(md).ok()
    let md = md.into::<Jpeg>().ok().expect("not JPEG metadata");
    assert_eq!(md.dimensions, OWLET_DIM);
    assert_eq!(md.sample_precision, 8);
    assert_eq!(md.coding_process, jpeg::CodingProcess::DctSequential);
    assert_eq!(md.entropy_coding, jpeg::EntropyCoding::Huffman);
    assert!(md.baseline);
    assert!(!md.differential);
}

#[test]
fn test_png() {
    let md = immeta::load_from_file("tests/images/owlet.png").unwrap();

    assert_eq!(md.mime_type(), "image/png");
    assert_eq!(md.dimensions(), OWLET_DIM);

    let md = md.into::<Png>().ok().expect("not PNG metadata");
    assert_eq!(md.dimensions, OWLET_DIM);
    assert_eq!(md.color_type, png::ColorType::Rgb);
    assert_eq!(md.color_depth, 24);
    assert_eq!(md.compression_method, png::CompressionMethod::DeflateInflate);
    assert_eq!(md.filter_method, png::FilterMethod::AdaptiveFiltering);
    assert_eq!(md.interlace_method, png::InterlaceMethod::Disabled);
}

#[test]
fn test_gif_plain() {
    let md = immeta::load_from_file("tests/images/owlet.gif").unwrap();

    assert_eq!(md.mime_type(), "image/gif");
    assert_eq!(md.dimensions(), OWLET_DIM);

    let md = md.into::<Gif>().ok().expect("not GIF metadata");
    assert_eq!(md.version, gif::Version::V89a);
    assert_eq!(md.dimensions, OWLET_DIM);
    assert_eq!(md.global_color_table, Some(gif::ColorTable {
        size: 256,
        sorted: false
    }));
    assert_eq!(md.color_resolution, 256);
    assert_eq!(md.background_color_index, 0);
    assert_eq!(md.pixel_aspect_ratio, 0);
    assert_eq!(md.is_animated(), false);
    assert_eq!(md.blocks, vec![
        gif::Block::GraphicControlExtension(gif::GraphicControlExtension {
            disposal_method: gif::DisposalMethod::None,
            user_input: false,
            transparent_color_index: None,
            delay_time: 0
        })
    ])
}

#[test]
fn test_gif_animated() {
    let md = immeta::load_from_file("tests/images/drop.gif").unwrap();

    assert_eq!(md.mime_type(), "image/gif");
    assert_eq!(md.dimensions(), DROP_DIM);

    let md = md.into::<Gif>().ok().expect("not GIF metadata");
    assert_eq!(md.version, gif::Version::V89a);
    assert_eq!(md.dimensions, DROP_DIM);
    assert_eq!(md.global_color_table, Some(gif::ColorTable {
        size: 256,
        sorted: false
    }));
    assert_eq!(md.color_resolution, 128);
    assert_eq!(md.background_color_index, 255);
    assert_eq!(md.pixel_aspect_ratio, 0);

    let mut blocks = md.blocks.iter();

    assert_eq!(
        blocks.next().unwrap(),
        &gif::Block::ApplicationExtension(gif::ApplicationExtension {
            application_identifier: *b"NETSCAPE",
            authentication_code: *b"2.0"
        })
    );
}

#[test]
fn test_webp() {
    let md = immeta::load_from_file("tests/images/cherry.webp").unwrap();

    assert_eq!(md.mime_type(), "image/webp");
    assert_eq!(md.dimensions(), CHERRY_DIM);

    let md = md.into::<Webp>().ok().expect("not WEBP metadata");

    println!("{:?}", md);
}

#[test]
fn test_incomplete_gif() {
    let response = reqwest::blocking::get("https://sample-videos.com/gif/2.gif").unwrap();
    let chunk = response.bytes().unwrap().slice(0..8192);
    let metadata = load_from_buf(&chunk).unwrap();
    assert_eq!(metadata.dimensions().width, 400);
    assert_eq!(metadata.dimensions().height, 200);
}

#[test]
fn test_incomplete_jpeg() {
    let response = reqwest::blocking::get("https://www.learningcontainer.com/bfd_download/large-sample-image-file-download-for-testing/").unwrap();
    let chunk = response.bytes().unwrap().slice(0..8192);
    let metadata = load_from_buf(&chunk).unwrap();
    assert_eq!(metadata.dimensions().width, 7200);
    assert_eq!(metadata.dimensions().height, 5400);
}

#[test]
fn test_incomplete_png() {
    let response = reqwest::blocking::get("https://www.learningcontainer.com/bfd_download/sample-png-file-for-testing/").unwrap();
    let chunk = response.bytes().unwrap().slice(0..8192);
    let metadata = load_from_buf(&chunk).unwrap();
    assert_eq!(metadata.dimensions().width, 7200);
    assert_eq!(metadata.dimensions().height, 5400);
}
