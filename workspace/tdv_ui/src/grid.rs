use tdv_msg::{GridLayout, GridElement};

fn remove_all_grids() {
    js! {
        var plot_area = document.getElementById("tdv_plot_area");
        while (plot_area.firstChild) {
            plot_area.removeChild(plot_area.firstChild);
        }
    };
}

fn remove_all_grid_styles() {
    js! {
        // Get style sheet named "tdv_style"
        var sheets = document.styleSheets;
        console.log(sheets);
        for (var i = 0; i < sheets.length; i++) {
            console.log(sheets[i].ownerNode.id);
            if (sheets[i].ownerNode.id == "tdv_style") {
                var sheet = sheets[i];
                break;
            }
        }

        var rules = sheet.cssRules;
        var n = rules.length;
        for (var i = 0; i < n; i++) {
            sheet.deleteRule(0)
        }
    }
}

fn wrapper(n_columns: u32) -> String {
    format!("#tdv_plot_area {{ \
display: grid; \
grid-template-columns: repeat({}, 1fr); \
grid-template-rows: repeat(2, 1fr); \
grid-gap: 10px; \
grid-auto-rows: minmax(100px, auto); }}", n_columns
    ).to_string()
}

fn element(e: &GridElement) -> String {
    let gr = if e.row_max == 0 {
        format!("{}", e.row)
    } else {
        format!("{} / {}", e.row, e.row_max + 1)
    };

    let gc = if e.col_max == 0 {
        format!("{}", e.col)
    } else {
        format!("{} / {}", e.col, e.col_max + 1)
    };

    format!("#{} {{ grid-row: {}; grid-column: {}; }}", e.id, gr, gc).to_string()
}

fn add_grid_styles(gl: &GridLayout) {
    let mut rules = vec![];

    rules.push(wrapper(gl.n_columns));

    for e in gl.elements.iter() {
        rules.push(element(e));
    }

    js! {
        // Get style sheet named "tdv_style"
        var sheets = document.styleSheets;
        console.log(sheets);
        for (var i = 0; i < sheets.length; i++) {
            console.log(sheets[i].ownerNode.id);
            if (sheets[i].ownerNode.id == "tdv_style") {
                var sheet = sheets[i];
                break;
            }
        }

        var rules = @{ rules };

        for (var i = 0; i < rules.length; i++) {
            console.log(rules[i]);
            sheet.insertRule(rules[i], i);
        }

        // For debug
        console.log(sheet);
    }
}

fn add_grids(gl: &GridLayout) {
    let mut divs = Vec::new();

    for e in gl.elements.iter() {
        divs.push(e.id.clone());
    }

    js! {
        var plot_area = document.getElementById("tdv_plot_area");
        var divs = @{ divs };
        for (var i = 0; i < divs.length; i++) {
            var node = document.createElement("div");
            node.id = divs[i];
            plot_area.appendChild(node);
        }

        console.log(plot_area);
    }
}

pub fn set_grid_layout(gl: &GridLayout) {
    remove_all_grids();
    remove_all_grid_styles();
    add_grid_styles(gl);
    add_grids(gl);
}
