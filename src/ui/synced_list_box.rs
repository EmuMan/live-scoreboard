use std::sync::{Arc, Mutex};

use gtk::prelude::*;

use crate::ui::entry_window::{EntryWindowField, open_entry_window};

#[derive(Clone)]
pub struct SyncedListBox<T> {
    pub window: gtk::ApplicationWindow,
    pub list_box: gtk::ListBox,
    pub shared_state: crate::SharedState,
    pub make_row: Arc<Mutex<Box<dyn Fn(&T) -> gtk::ListBoxRow>>>,
    pub get_data: Arc<Mutex<Box<dyn Fn(&crate::AppState) -> Option<&Vec<T>>>>>,
    pub get_mut_data: Arc<Mutex<Box<dyn Fn(&mut crate::AppState) -> Option<&mut Vec<T>>>>>,
    pub to_entry_window: Arc<Mutex<Box<dyn Fn(Option<&T>) -> Vec<EntryWindowField>>>>,
    pub from_entry_window: Arc<Mutex<Box<dyn Fn(&std::collections::HashMap<String, Option<String>>, Option<&T>) -> T>>>,
}

pub trait ConnectableList {
    fn connect_add_button(&self, button: &gtk::Button);
    fn connect_remove_button(
        &self,
        button: &gtk::Button,
        correct_for_move: Option<Box<dyn Fn(&mut crate::AppState, usize, Option<usize>)>>
    );
    fn connect_edit_button(&self, button: &gtk::Button);
    fn connect_move_button(
        &self,
        button: &gtk::Button,
        amount: i32,
        correct_for_move: Option<Box<dyn Fn(&mut crate::AppState, usize, Option<usize>)>>
    );
}

impl<T: Clone + 'static> SyncedListBox<T> {
    pub fn new(
        window: gtk::ApplicationWindow,
        list_box: gtk::ListBox,
        shared_state: crate::SharedState,
        make_row: Box<dyn Fn(&T) -> gtk::ListBoxRow>,
        get_data: Box<dyn Fn(&crate::AppState) -> Option<&Vec<T>>>,
        get_mut_data: Box<dyn Fn(&mut crate::AppState) -> Option<&mut Vec<T>>>,
        to_entry_window: Box<dyn Fn(Option<&T>) -> Vec<EntryWindowField>>,
        from_entry_window: Box<dyn Fn(&std::collections::HashMap<String, Option<String>>, Option<&T>) -> T>,
    ) -> Self {
        Self {
            window,
            list_box,
            shared_state,
            make_row: Arc::new(Mutex::new(make_row)),
            get_data: Arc::new(Mutex::new(get_data)),
            get_mut_data: Arc::new(Mutex::new(get_mut_data)),
            to_entry_window: Arc::new(Mutex::new(to_entry_window)),
            from_entry_window: Arc::new(Mutex::new(from_entry_window)),
        }
    }

    pub fn new_shared(
        window: gtk::ApplicationWindow,
        list_box: gtk::ListBox,
        shared_state: crate::SharedState,
        make_row: Box<dyn Fn(&T) -> gtk::ListBoxRow>,
        get_data: Box<dyn Fn(&crate::AppState) -> Option<&Vec<T>>>,
        get_mut_data: Box<dyn Fn(&mut crate::AppState) -> Option<&mut Vec<T>>>,
        to_entry_window: Box<dyn Fn(Option<&T>) -> Vec<EntryWindowField>>,
        from_entry_window: Box<dyn Fn(&std::collections::HashMap<String, Option<String>>, Option<&T>) -> T>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new(
            window,
            list_box,
            shared_state,
            make_row,
            get_data,
            get_mut_data,
            to_entry_window,
            from_entry_window,
        )))
    }

    pub fn remove_all(&self) {
        self.list_box.remove_all();
    }

    pub fn populate(&self) {
        self.list_box.remove_all();
        let state = self.shared_state.lock().unwrap();
        if let Some(data) = (self.get_data.lock().unwrap())(&state) {
            for item in data {
                self.list_box.append(&(self.make_row.lock().unwrap())(item));
            }
        }
    }

    pub fn append_new(&self) {
        let entry_window_fields = (self.to_entry_window.lock().unwrap())(None);

        let from_entry_window = self.from_entry_window.clone();
        let list_box = self.list_box.clone();
        let shared_state = self.shared_state.clone();
        let get_mut_data = self.get_mut_data.clone();
        let make_row = self.make_row.clone();

        open_entry_window(
            &self.window,
            shared_state.clone(),
            "Add Item",
            entry_window_fields,
            Box::new(move |fields| {
                let new_item = (from_entry_window.lock().unwrap())(&fields, None);
                let mut state = shared_state.lock().unwrap();
                if let Some(mut_data) = (get_mut_data.lock().unwrap())(&mut state) {
                    list_box.append(&(make_row.lock().unwrap())(&new_item));
                    mut_data.push(new_item);
                }
            }),
        );
    }

    pub fn remove_selected(&self) -> Option<usize> {
        if let Some(selected_row) = self.list_box.selected_row() {
            let row_index = selected_row.index() as usize;
            let mut state = self.shared_state.lock().unwrap();
            if let Some(mut_data) = (self.get_mut_data.lock().unwrap())(&mut state) {
                mut_data.remove(row_index);
                std::mem::drop(state);
                self.list_box.remove(&selected_row);
                return Some(row_index);
            }
            return None;
        }
        None
    }

    pub fn edit_selected(&self) {
        if let Some(selected_row) = self.list_box.selected_row() {
            let row_index = selected_row.index() as usize;
            let old_data = {
                let state = self.shared_state.lock().unwrap();
                let data = (self.get_data.lock().unwrap())(&state);
                data.map(|data| data[row_index].clone())
            };
            let entry_window_fields = (self.to_entry_window.lock().unwrap())(old_data.as_ref());
            
            let from_entry_window = self.from_entry_window.clone();
            let list_box = self.list_box.clone();
            let shared_state = self.shared_state.clone();
            let get_mut_data = self.get_mut_data.clone();
            let make_row = self.make_row.clone();

            open_entry_window(
                &self.window,
                shared_state.clone(),
                "Edit Item",
                entry_window_fields,
                Box::new(move |fields| {
                    let new_item = {
                        let new_item = (from_entry_window.lock().unwrap())(&fields, old_data.as_ref());
                        let mut state = shared_state.lock().unwrap();
                        let mut_data = (get_mut_data.lock().unwrap())(&mut state);
                        mut_data.map(|mut_data| {
                            mut_data[row_index] = new_item.clone();
                            new_item
                        })
                    };
                    if let Some(new_item) = new_item {
                        let new_row = (make_row.lock().unwrap())(&new_item);
                        list_box_move(&list_box, &selected_row, 0, Some(&new_row));
                    }
                }),
            );
        }
    }

    pub fn move_selected(&self, amount: i32) -> Option<usize> {
        if let Some(selected_row) = self.list_box.selected_row() {
            let row_index = selected_row.index() as usize;
            let data_length = {
                let state = self.shared_state.lock().unwrap();
                let data = (self.get_data.lock().unwrap())(&state);
                data.map(|data| data.len()).unwrap_or(0)
            };
            // cap the amount to the bounds of the list
            let amount = if amount > 0 {
                amount.min(data_length as i32 - row_index as i32 - 1)
            } else {
                amount.max(-(row_index as i32))
            };
            if amount != 0 {
                let new_index = (row_index as i32 + amount) as usize;
                // incredibly cursed block of code
                // (dual block to separate the lock from the list_box_move call)
                if {
                    let mut state = self.shared_state.lock().unwrap();
                    let mut_data = (self.get_mut_data.lock().unwrap())(&mut state);
                    mut_data.map(|mut_data| mut_data.swap(row_index, new_index)).is_some()
                } {
                    list_box_move(&self.list_box, &selected_row, amount, None);
                    return Some(row_index);
                }
            }
        }
        None
    }
}

impl<T: Clone + 'static> ConnectableList for Arc<Mutex<SyncedListBox<T>>> {
    fn connect_add_button(&self, button: &gtk::Button) {
        let self_cloned = self.clone();
        button.connect_clicked(move |_| self_cloned.lock().unwrap().append_new());
    }

    fn connect_remove_button(
        &self,
        button: &gtk::Button,
        correct_for_move: Option<Box<dyn Fn(&mut crate::AppState, usize, Option<usize>)>>
    ) {
        let self_cloned = self.clone();
        button.connect_clicked(move |_| {
            let removed_index = self_cloned.lock().unwrap().remove_selected();
            if let (Some(correct_for_move), Some(removed_index)) =
                (&correct_for_move, removed_index)
            {
                let locked_self = self_cloned.lock().unwrap();
                let mut state = locked_self.shared_state.lock().unwrap();
                correct_for_move(&mut state, removed_index, None);
            }
        });
    }

    fn connect_edit_button(&self, button: &gtk::Button) {
        let self_cloned = self.clone();
        button.connect_clicked(move |_| self_cloned.lock().unwrap().edit_selected());
    }

    fn connect_move_button(
        &self,
        button: &gtk::Button,
        amount: i32,
        correct_move: Option<Box<dyn Fn(&mut crate::AppState, usize, Option<usize>)>>
    ) {
        let self_cloned = self.clone();
        button.connect_clicked(move |_| {
            let initial_index = self_cloned.lock().unwrap().move_selected(amount);
            if let (Some(correct_move), Some(initial_index)) =
                (&correct_move, initial_index)
            {
                let locked_self = self_cloned.lock().unwrap();
                let mut state = locked_self.shared_state.lock().unwrap();
                let new_index = (initial_index as i32 + amount) as usize;
                correct_move(&mut state, initial_index, Some(new_index));
            }
        });
    }
}

// Does NOT check bounds
fn list_box_move(
    list_box: &gtk::ListBox,
    row: &gtk::ListBoxRow,
    direction: i32,
    replace_with: Option<&gtk::ListBoxRow>
) {
    let row_index = row.index() as i32;
    let new_index = row_index + direction;
    let new_row = replace_with.unwrap_or_else(|| row);
    list_box.remove(row);
    list_box.insert(new_row, new_index);
    list_box.select_row(None as Option<&gtk::ListBoxRow>);
    list_box.select_row(Some(new_row));
}
