// treeview.rs
//
// Copyright 2022 Stéphane Cerveau <scerveau@collabora.com>
//
// This file is part of GstPipelineStudio
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::GPSApp;
use gtk::prelude::TreeViewExt;
use gtk::{CellRendererText, TreeView, TreeViewColumn};

pub fn add_column_to_treeview(app: &GPSApp, tree_name: &str, column_name: &str, column_n: i32) {
    let treeview: TreeView = app
        .builder
        .object(tree_name)
        .expect("Couldn't get tree_name");
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", column_n);
    column.set_title(column_name);
    treeview.append_column(&column);
}
