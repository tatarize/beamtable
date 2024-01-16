use beamtable::geometry::Geomstr;
use beamtable::scanbeam::ScanBeam;
use clap::Parser;
use std::path::PathBuf;
use vsvg::{DocumentTrait, Draw, LayerTrait, PathTrait};

#[derive(clap::Parser, Debug)]

struct Args {
    /// SVG file
    path: PathBuf,

    /// Display the result
    #[clap(long)]
    show: bool,

    /// Save the result as SVG
    #[clap(long)]
    save: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use clap
    let args = Args::parse();

    // load and flatten the document as we only support polyline
    let doc = vsvg::Document::from_svg(&args.path, true)?.flatten(vsvg::DEFAULT_TOLERANCE);

    // convert everything to lines
    let mut segments = Geomstr::new();
    let _: Vec<_> = doc
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
        .map(|(i, (p0, p1))| segments.line(p0, p1, i as f64))
        .collect();

    // run scan beam algorithm
    let mut scanbeam = ScanBeam::new(segments);
    let beam_table = scanbeam.build();
    let mask1 = beam_table.evenodd_fill(1.0);
    let mask2 = beam_table.evenodd_fill(2.0);
    let mask10 = beam_table.evenodd_fill(10.0);
    let q = mask1 | mask2 | mask10;

    let geom = beam_table.create(q);

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
        layer.circle(event.x, event.y, 0.5);
    }

    if let Some(path) = args.save {
        // work around https://github.com/abey79/vsvg/issues/114
        doc.metadata_mut().source = None;

        doc.to_svg_file(path)?;
    }

    if args.show {
        vsvg_viewer::show(doc.into())?;
    }

    Ok(())
}
