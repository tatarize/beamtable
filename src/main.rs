use beamtable::geometry::Line;
use beamtable::scanbeam::ScanBeam;
use vsvg::{DocumentTrait, LayerTrait, PathTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load a SVG if provided
    let path = std::env::args().nth(1).map(std::path::PathBuf::from);
    let doc = if let Some(path) = path {
        vsvg::Document::from_svg(&path, true)?
    } else {
        vsvg::Document::default()
    };

    // flatten the document as we only support polyline
    let doc = doc.flatten(vsvg::DEFAULT_TOLERANCE);

    // convert everything to lines
    let lines: Vec<_> = doc
        .layers
        .values()
        .flat_map(|layer| {
            layer.paths.iter().flat_map(|path| {
                path.data
                    .points()
                    .windows(2)
                    .map(|p| ((p[0].x(), p[0].y()), (p[1].x(), p[1].y())))
            })
        })
        .enumerate()
        .map(|(i, (p0, p1))| Line::new(p0, p1, i))
        .collect();

    // run scan beam algorithm
    let mut scanbeam = ScanBeam::new(lines);
    let beam_table = scanbeam.build();

    //
    // visualize the result
    //

    // convert back to regular (not flattened) document, merge everything to layer 0 and normalize
    // line width and color
    let mut doc = vsvg::Document::from(doc);
    doc.merge_layers();
    doc.for_each(|layer| {
        layer.for_each(|path| {
            path.metadata_mut().stroke_width = 0.5;
            path.metadata_mut().color = vsvg::Color::LIGHT_RED;
        });
    });

    let layer = doc.get_mut(1);
    for event in beam_table.events {
        // TODO: this is annoying to do, `Layer` should implement `Draw` https://github.com/abey79/vsvg/issues/109
        let path = vsvg::Path::from_metadata(
            kurbo::Circle::new(
                kurbo::Point {
                    x: event.x,
                    y: event.y,
                },
                0.5,
            ),
            vsvg::PathMetadata::default(),
        );

        layer.push_path(path);
    }

    vsvg_viewer::show(doc.into())?;

    Ok(())
}
