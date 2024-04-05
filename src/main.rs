use druid::widget::{Flex, Image, WidgetExt};
use druid::{AppLauncher, ImageBuf, WindowDesc};
use resvg::*;
use tiny_skia::Pixmap;
//use fontdb::Database;

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("SVG Viewer with Druid")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}

fn build_ui() -> impl druid::Widget<()> {
    // Path to your SVG file
    let svg_path = "five.svg";

    // Load and parse the SVG file
    let svg_data = std::fs::read(svg_path).unwrap();
    let opt = usvg::Options::default();
    let fdb = fontdb::Database::new();
    let rtree = usvg::Tree::from_data(&svg_data, &opt, &fdb).expect("Failed to parse SVG");

    // Create a pixmap where the SVG will be rendered
    //let pixmap_size = rtree.svg_node().size.to_screen_size();
    let pixmap_size = rtree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).expect("Failed to create pixmap");

    // Render the SVG
    //resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).expect("Failed to render SVG");
    resvg::render(&rtree, usvg::Transform::identity(), &mut pixmap.as_mut());

    // Convert pixmap to Druid ImageBuf
    let image_buf = ImageBuf::from_raw(pixmap.data(), druid::piet::ImageFormat::RgbaPremul, pixmap.width() as usize, pixmap.height() as usize);

    // Create and return the image widget
    let image_widget = Image::new(image_buf.clone()).fix_width(pixmap.width() as f64).fix_height(pixmap.height() as f64).center();
    Flex::column().with_child(image_widget)
}