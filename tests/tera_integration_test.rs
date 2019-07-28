#[test]
fn test_tera_integration() {
    let mut tera = tera::compile_templates!("tests/templates/**/*");

    let manifest = misto::Manifest::from_file("tests/mix-manifest.json").unwrap();
    misto::tera::configure(&mut tera, manifest);

    let rendered = tera
        .render("test.html.tera", &tera::Context::new())
        .unwrap();

    assert_eq!(
        "<script type=\"text/javascript\" src=\"public/js/app-06fd465b135293c687c152ec96567f5d.js\"></script>\n",
        rendered
    );
}
