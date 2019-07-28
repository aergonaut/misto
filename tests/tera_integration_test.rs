use std::error::Error;

#[test]
fn test_asset_path() {
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

#[test]
fn test_asset_path_with_bad_asset_name() {
    let mut tera = tera::compile_templates!("tests/templates/**/*");

    let manifest = misto::Manifest::from_file("tests/mix-manifest.json").unwrap();
    misto::tera::configure(&mut tera, manifest);

    let rendered = tera.render("bad_asset_name.html.tera", &tera::Context::new());

    assert!(rendered.is_err());

    if let Err(err) = rendered {
        match err
            .source()
            .unwrap()
            .downcast_ref::<tera::Error>()
            .unwrap()
            .kind()
        {
            tera::ErrorKind::Msg(m) => assert_eq!(m, "Unknown asset"),
            _ => unreachable!(),
        }
    } else {
        unreachable!();
    }
}

#[test]
fn test_asset_path_with_missing_argument() {
    let mut tera = tera::compile_templates!("tests/templates/**/*");

    let manifest = misto::Manifest::from_file("tests/mix-manifest.json").unwrap();
    misto::tera::configure(&mut tera, manifest);

    let rendered = tera.render("missing_argument.html.tera", &tera::Context::new());

    assert!(rendered.is_err());

    if let Err(err) = rendered {
        match err
            .source()
            .unwrap()
            .downcast_ref::<tera::Error>()
            .unwrap()
            .kind()
        {
            tera::ErrorKind::Msg(m) => assert_eq!(m, "Missing argument `name`"),
            _ => unreachable!(),
        }
    } else {
        unreachable!();
    }
}
