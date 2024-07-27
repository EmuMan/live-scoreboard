use gtk::prelude::*;

use crate::SharedState;

pub fn build_box(_window: &gtk::ApplicationWindow, shared_state: SharedState) -> gtk::Box {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    
    let bracket = build_bracket(shared_state.clone());

    gtk_box.append(&bracket);
    
    println!("Connecting notify...");
    gtk_box.connect_visible_notify(move |gtk_box| {
        if gtk_box.is_visible() {
            println!("Bracket box is visible");
        }
    });

    gtk_box
}

pub fn build_bracket(shared_state: SharedState) -> gtk::Box {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let model;

    let mut team_names_with_none = vec![""];
    {
        let state = shared_state.lock().unwrap();
        let teams = &state.division.teams;
        let mut team_names: Vec<&str> = teams.iter().map(|team| team.name.as_str()).collect();
        team_names_with_none.append(&mut team_names);
        model = gtk::StringList::new(&team_names_with_none);
    }

    // first column, 8 teams
    let column1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    for i in 0..8 {
        let dropdown = make_dropdown(&model, 0, i, shared_state.clone());
        column1.append(&dropdown);
    };

    // second column, 4 teams
    let column2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    for i in 0..4 {
        let dropdown = make_dropdown(&model, 1, i, shared_state.clone());
        column2.append(&dropdown);
    };

    // third column, 2 teams
    let column3 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    for i in 0..2 {
        let dropdown = make_dropdown(&model, 2, i, shared_state.clone());
        column3.append(&dropdown);
    };

    // fourth column, 1 team
    let column4 = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    for i in 0..1 {
        let dropdown = make_dropdown(&model, 3, i, shared_state.clone());
        column4.append(&dropdown);
    };

    gtk_box.append(&column1);
    gtk_box.append(&column2);
    gtk_box.append(&column3);
    gtk_box.append(&column4);

    gtk_box
}

fn make_dropdown(model: &gtk::StringList, col: u32, row: u32, shared_state: SharedState) -> gtk::DropDown {
    let dropdown = gtk::DropDown::new(Some(model.clone()), gtk::Expression::NONE);

    {
        let state = shared_state.lock().unwrap();
        let bracket = &state.division.bracket;
        let selected = bracket.get(col as usize)
            .and_then(|col| col.get(row as usize))
            .map(|cell| cell.map(|cell| cell as u32 + 1).unwrap_or(0))
            .unwrap_or(0);
        dropdown.set_selected(selected);
    }

    dropdown.connect_selected_notify(move |dropdown| {
        let selected_index = dropdown.selected();
        let mut state = shared_state.lock().unwrap();
        state.division.bracket.get_mut(col as usize)
            .and_then(|col| col.get_mut(row as usize))
            .map(|cell| *cell =
                if selected_index == 0 {
                    None
                } else {
                    Some((selected_index - 1) as usize)
                }
            );
    });

    dropdown
}
