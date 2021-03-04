pub mod plot {

    // read in csv and make svg plot

    use clap::value_t;
    use csv::ReaderBuilder;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::prelude::*;

    // plot is main function
    // takes a csv (currently where there is only one telomeric repeat present,
    // so not suitable for tidk find where multiple telomeric repeats are queried)
    // and produces an SVG

    pub fn plot(matches: &clap::ArgMatches) -> std::io::Result<()> {
        let csv = matches.value_of("csv").unwrap();
        let chromosome_cutoff =
            value_t!(matches.value_of("length_chromosome"), i32).unwrap_or_else(|e| e.exit());
        let output = matches.value_of("output").unwrap();

        let parsed_csv = parse_csv(csv);

        let width = 1000;
        // allow height 150 per chromosome?
        let height_per_plot = 200;
        let chromosome_number = chromosome_number(&parsed_csv, chromosome_cutoff);

        let height: i32 = height_per_plot * chromosome_number as i32 + height_per_plot + MARGIN;

        // pass height constant here
        let plot_data = generate_plot_data(parsed_csv, height, width, height_per_plot);

        let plot_data_filtered: Vec<PlotData> = plot_data
            .clone()
            .into_iter()
            .filter(|x| x.max > chromosome_cutoff as usize)
            .collect();

        let out_filename = format!("{}.svg", output);

        let mut svg_file = File::create(out_filename)?;

        let hover_stroke_width = 3;

        let svg = format!(
            "<?xml version='1.0' encoding='UTF-8'  standalone='no' ?> <!DOCTYPE svg \
                 PUBLIC '-//W3C//DTD SVG 1.0//EN' \
                 'http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd'> <svg version='1.0' \
                 width='{}' height='{}' xmlns='http://www.w3.org/2000/svg' \
                 xmlns:xlink='http://www.w3.org/1999/xlink'> \

                 <style type='text/css'> \
                 .chromosome_line:hover {{ stroke-opacity: 1.0; stroke: crimson; stroke-width: {}; }} \
                 .chromosome_text {{ font: 15px sans-serif; }}
                 </style> \

                 {} \
                 </svg>",
            width,
            height,
            hover_stroke_width,
            add_all_path_elements(plot_data_filtered)
        );

        svg_file.write_all(svg.as_bytes()).expect("unable to write");

        Ok(())
    }

    // the CSV records
    #[derive(Debug, Deserialize)]
    pub struct TelomericRepeatRecord {
        pub ID: String,
        pub window: i32,
        pub forward_repeat_number: i32,
        pub reverse_repeat_number: i32,
        pub telomeric_repeat: String,
    }

    fn parse_csv(path: &str) -> Vec<TelomericRepeatRecord> {
        let mut csv_reader = ReaderBuilder::new().from_path(path).unwrap();
        let mut plot_coords_vec = Vec::new();
        for result in csv_reader.deserialize() {
            let record: TelomericRepeatRecord = result.unwrap();
            plot_coords_vec.push(record);
        }
        plot_coords_vec
    }

    fn chromosome_number(parsed_csv: &Vec<TelomericRepeatRecord>, size: i32) -> usize {
        // so we can break the loop
        let file_length = parsed_csv.len();
        // the iteration of the loop
        let mut it = 0usize;
        // a mutable vector to calculate svg path attribute
        let mut max_sizes = Vec::new();

        loop {
            if it == file_length - 1 {
                break;
            }

            if parsed_csv[it].ID == parsed_csv[it + 1].ID {
                it += 1;
                continue;
            } else {
                if parsed_csv[it].window > size {
                    max_sizes.push(parsed_csv[it].window);
                }
                it += 1;
            }
        }
        max_sizes.len()
    }
    // scale a range [min,max] to [a,b]
    fn scale_y(y: f64, a: f64, b: f64, min: f64, max: f64) -> f64 {
        (((b - a) * (y - min)) / (max - min)) + a
    }

    const MARGIN: i32 = 40;

    fn make_path_element(
        path_vec: Vec<(i32, i32)>,
        x_max: usize,
        y_max: usize,
        height: i32,
        width: i32,
        height_per_plot: i32,
    ) -> String {
        // somehow going to have to scale the lines to fit on all graphs
        let mut path = String::new();
        let width_incl_margins = width - MARGIN;
        //let height_incl_margins = height - MARGIN;

        let x_bin: f64 = width_incl_margins as f64 / x_max as f64;

        // add the first move to point
        path += &format!(
            "M{},{}",
            (MARGIN / 2) as f64 + x_bin,
            // y is height - repeat number, a = 0, b = height_per_plot
            // min is again zero (no negative repeats), max is greatest repeats per chromosome
            //
            height as f64
                - scale_y(
                    path_vec[0].1 as f64,
                    0.0,
                    height_per_plot as f64,
                    0.0,
                    y_max as f64
                )
        );

        let mut bin: f64 = 0.0;
        for element in path_vec.iter().skip(1) {
            path += &format!(
                "L{},{}",
                bin + (MARGIN as f64 / 2.0),
                //
                height as f64
                    - scale_y(
                        element.1 as f64,
                        0.0,
                        height_per_plot as f64,
                        0.0,
                        y_max as f64
                    )
            );
            bin += x_bin;
        }
        path
    }

    fn add_all_path_elements(plot_data: Vec<PlotData>) -> String {
        let mut all_paths = String::new();
        for (i, row) in plot_data.iter().enumerate() {
            // add text tag here
            all_paths += &format!(
                "<text x='25' y='{}' class='chromosome_label'>{}\n↓</text>",
                (1 + i as isize * 200) + 255,
                row.id
            );
            all_paths += &format!("<path d='{}' class='chromosome_line' stroke='black' fill='none' stroke-width='1' transform='translate(0,{})'/>\n", row.path, i as isize * -200);
        }
        all_paths
    }

    #[derive(Debug, Clone)]
    pub struct PlotData {
        // chromosome
        pub id: String,
        // svg path attribute
        pub path: String,
        // max length of chromosome
        pub max: usize,
        // name of the telomeric repeat (not needed?)
        pub sequence: String,
    }

    fn generate_plot_data(
        parsed_csv: Vec<TelomericRepeatRecord>,
        height: i32,
        width: i32,
        height_per_plot: i32,
    ) -> Vec<PlotData> {
        // so we can break the loop
        let file_length = parsed_csv.len();
        // the iteration of the loop
        let mut it = 0usize;
        // a mutable vector to calculate svg path attribute
        let mut path_vec = Vec::new();
        let mut plot_data = Vec::new();
        let mut y_max = 0;

        loop {
            if it == file_length - 1 {
                plot_data.push(PlotData {
                    id: parsed_csv[it].ID.clone(),
                    path: make_path_element(
                        path_vec.clone(),
                        path_vec.clone().len(),
                        y_max as usize,
                        height,
                        width,
                        height_per_plot,
                    ),
                    max: parsed_csv[it].window as usize,
                    sequence: parsed_csv[it].telomeric_repeat.clone(),
                });
                break;
            }

            if parsed_csv[it].ID == parsed_csv[it + 1].ID {
                // calculate y max
                if y_max
                    <= parsed_csv[it].forward_repeat_number + parsed_csv[it].reverse_repeat_number
                {
                    y_max =
                        parsed_csv[it].forward_repeat_number + parsed_csv[it].reverse_repeat_number;
                }
                // window (i.e x)
                // forward + reverse counts
                path_vec.push((
                    parsed_csv[it].window,
                    parsed_csv[it].forward_repeat_number + parsed_csv[it].reverse_repeat_number,
                ));
                it += 1;
            } else {
                // calculate the svg path element from path_vec here

                plot_data.push(PlotData {
                    id: parsed_csv[it].ID.clone(),
                    path: make_path_element(
                        path_vec.clone(),
                        path_vec.clone().len(),
                        y_max as usize,
                        height,
                        width,
                        height_per_plot,
                    ),
                    max: parsed_csv[it].window as usize,
                    sequence: parsed_csv[it].telomeric_repeat.clone(),
                });
                path_vec.clear();
                it += 1;
                y_max = 0;
            }
        }

        plot_data
    }
}
