use beamtable::geometry::Geomstr;
use beamtable::table::BeamTable;
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
    let mut idx = 0;
    doc.layers.values().for_each(|layer| {
        layer.paths.iter().for_each(|path| {
            path.data.points().windows(2).for_each(|p| {
                segments.line((p[0].x(), p[0].y()), (p[1].x(), p[1].y()), idx as f64);
            });
            idx += 1;
        });
    });

    // run scan beam algorithm
    let mut beamtable = BeamTable::new(segments);
    beamtable.build();
    // let mask = beamtable.evenodd_fill(20.0);
    let mask = beamtable.union_all();
    let geom = beamtable.create(mask);

    //
    // visualize the result
    //

    // convert back to regular (not flattened) document, merge everything to layer 0 and normalize
    // line width and color
    let mut doc = vsvg::Document::default();

    let layer = doc.get_mut(1);
    for line in geom.segments {
        layer.line(line.0 .0, line.0 .1, line.4 .0, line.4 .1);
    }

    doc.merge_layers();
    doc.for_each(|layer| {
        layer.for_each(|path| {
            path.metadata_mut().stroke_width = 0.5;
            path.metadata_mut().color = vsvg::Color::LIGHT_RED;
        });
    });

    // let layer = doc.get_mut(1);
    // for event in beam_table.events {
    //     layer.circle(event.x, event.y, 0.5);
    // }

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
