extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate stdweb;

use std::collections::HashMap;
use tdv_msg::{DataFrame, PlotParam, Trace};

#[derive(Serialize)]
struct PlotlyData(Vec<PlotlyTrace>);

#[derive(Serialize)]
enum PlotlyTrace {
    PlotlyScatter(PlotlyScatter),
}

#[derive(Serialize)]
struct PlotlyScatter {
    _type: String,
    x: Vec<f64>,
    y: Vec<f64>,
}

js_serializable!( PlotlyTrace );
js_serializable!( PlotlyScatter );
js_serializable!( PlotlyData );

pub fn plot(data: &HashMap<String, DataFrame>, params: &PlotParam) {
    let data = PlotlyData(
        params.traces.iter().map(|trace| {
            match trace {
                Trace::Scatter(scatter) => {
                    let df = &data.get(&scatter.df_name).unwrap();
                    PlotlyTrace::PlotlyScatter(PlotlyScatter {
                        _type: "scatter".to_string(),
                        x: df.get_col(&scatter.col_name_x).unwrap().clone(),
                        y: df.get_col(&scatter.col_name_y).unwrap().clone(),
                    })
                }
            }
        }).collect()
    );

    // TODO: Move area_name to "layout" variable
    let area_name = &params.area_name;

    js! {
        var data1 = @{ data };

        // Unwrap PlotlyTrace objects
        var data2 = data1.map(v => v[Object.keys(v)[0]]);

        // Replace property "_type" with "type"
        data2.forEach(function (v) {
            v["type"] = v["_type"];
            Reflect.deleteProperty(v, "_type");
        });

        Plotly.plot(
            document.getElementById(@{ &area_name }),
            data2,
            {
                margin: { t: 0 }
            }
        );
    }
}
