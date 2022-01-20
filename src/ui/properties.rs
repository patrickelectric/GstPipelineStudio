// properties.rs
//
// Copyright 2022 Stéphane Cerveau <scerveau@collabora.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-only
use crate::app::GPSApp;
use crate::gps as GPS;
use crate::logger;
use crate::{GPS_INFO, GPS_TRACE};
use gtk::glib;
use gtk::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::{Box, Button, CheckButton, ComboBoxText, Dialog, Entry, Label, Widget};

fn value_as_str(v: &glib::Value) -> Option<String> {
    match v.type_() {
        glib::Type::I8 => Some(str_some_value!(v, i8).to_string()),
        glib::Type::U8 => Some(str_some_value!(v, u8).to_string()),
        glib::Type::BOOL => Some(str_some_value!(v, bool).to_string()),
        glib::Type::I32 => Some(str_some_value!(v, i32).to_string()),
        glib::Type::U32 => Some(str_some_value!(v, u32).to_string()),
        glib::Type::I64 => Some(str_some_value!(v, i64).to_string()),
        glib::Type::U64 => Some(str_some_value!(v, u64).to_string()),
        glib::Type::F32 => Some(str_some_value!(v, f32).to_string()),
        glib::Type::F64 => Some(str_some_value!(v, f64).to_string()),
        glib::Type::STRING => str_opt_value!(v, String),
        _ => None,
    }
}

pub fn property_to_widget<F: Fn(String, String) + 'static>(
    app: &GPSApp,
    node_id: u32,
    element_name: &str,
    property_name: &str,
    param: &glib::ParamSpec,
    f: F,
) -> Option<Widget> {
    match param.type_() {
        _t if param.type_() == glib::ParamSpecBoolean::static_type() => {
            let check_button = CheckButton::new();
            check_button.set_widget_name(property_name);
            GPS_TRACE!("add CheckBox property : {}", check_button.widget_name());
            if let Some(value) = app.element_property(node_id, property_name) {
                check_button.set_active(value.parse::<bool>().unwrap_or(false));
            } else if (param.flags() & glib::ParamFlags::READABLE) == glib::ParamFlags::READABLE
                || (param.flags() & glib::ParamFlags::READWRITE) == glib::ParamFlags::READWRITE
            {
                if let Ok(value) = GPS::ElementInfo::element_property(element_name, param.name()) {
                    check_button.set_active(value.parse::<bool>().unwrap_or(false));
                }
            } else if let Some(value) = value_as_str(param.default_value()) {
                check_button.set_active(value.parse::<bool>().unwrap_or(false));
            }
            check_button.connect_toggled(glib::clone!(@weak check_button => move |c| {
                f(c.widget_name().to_string(), c.is_active().to_string() );
            }));
            Some(check_button.upcast::<Widget>())
        }
        t if [
            glib::ParamSpecInt::static_type(),
            glib::ParamSpecUInt::static_type(),
            glib::ParamSpecInt64::static_type(),
            glib::ParamSpecUInt64::static_type(),
            glib::ParamSpecString::static_type(),
        ]
        .contains(&t) =>
        {
            let entry = Entry::new();
            entry.set_widget_name(property_name);
            GPS_TRACE!("Add Edit property : {}", entry.widget_name());
            if let Some(value) = app.element_property(node_id, property_name) {
                entry.set_text(&value);
            } else if (param.flags() & glib::ParamFlags::READABLE) == glib::ParamFlags::READABLE
                || (param.flags() & glib::ParamFlags::READWRITE) == glib::ParamFlags::READWRITE
            {
                if let Ok(value) = GPS::ElementInfo::element_property(element_name, param.name()) {
                    entry.set_text(&value);
                }
            } else if let Some(value) = value_as_str(param.default_value()) {
                entry.set_text(&value);
            }

            entry.connect_changed(glib::clone!(@weak entry=> move |e| {
                f(e.widget_name().to_string(), e.text().to_string())
            }));
            Some(entry.upcast::<Widget>())
        }
        t if [
            glib::ParamSpecEnum::static_type(),
            glib::ParamSpecFlags::static_type(),
        ]
        .contains(&t) =>
        {
            let combo = ComboBoxText::new();
            combo.set_widget_name(property_name);
            GPS_TRACE!("add ComboBox property : {}", combo.widget_name());
            if t.is_a(glib::ParamSpecEnum::static_type()) {
                let param = param
                    .clone()
                    .downcast::<glib::ParamSpecEnum>()
                    .expect("Should be a ParamSpecEnum");
                let enums = param.enum_class();
                for value in enums.values() {
                    combo.append_text(&format!(
                        "{}:{}:{}",
                        value.value(),
                        value.nick(),
                        value.name()
                    ));
                }
            } else if t.is_a(glib::ParamSpecFlags::static_type()) {
                let param = param
                    .clone()
                    .downcast::<glib::ParamSpecFlags>()
                    .expect("Should be a ParamSpecEnum");
                let flags = param.flags_class();
                for value in flags.values() {
                    combo.append_text(&format!(
                        "{}:{}: {}",
                        value.value(),
                        value.nick(),
                        value.name()
                    ));
                }
            }
            if let Some(value) = app.element_property(node_id, property_name) {
                let value = value.split_once(':').unwrap_or(("0", ""));
                //Retrieve the first value (index) from the property
                combo.set_active(Some(value.0.parse::<u32>().unwrap_or(0)));
            } else if (param.flags() & glib::ParamFlags::READABLE) == glib::ParamFlags::READABLE
                || (param.flags() & glib::ParamFlags::READWRITE) == glib::ParamFlags::READWRITE
            {
                if let Ok(value) = GPS::ElementInfo::element_property(element_name, param.name()) {
                    combo.set_active(Some(value.parse::<u32>().unwrap_or(0)));
                }
            }

            combo.connect_changed(move |c| {
                if let Some(text) = c.active_text() {
                    let text = text.to_string();
                    let fields: Vec<&str> = text.split(':').collect();
                    f(
                        c.widget_name().to_string(),
                        format!("{}:{}", fields[0], fields[1]),
                    )
                }
            });
            Some(combo.upcast::<Widget>())
        }
        _ => {
            GPS_INFO!(
                "Property not supported : name={} type={}",
                property_name,
                param.type_()
            );
            None
        }
    }
}

pub fn display_plugin_properties(app: &GPSApp, element_name: &str, node_id: u32) {
    let dialog: Dialog = app
        .builder
        .object("dialog-plugin-properties")
        .expect("Couldn't get dialog-plugin-properties");

    dialog.set_title(Some(&format!("{} properties", element_name)));
    dialog.set_default_size(640, 480);
    dialog.set_modal(true);

    let properties_box: Box = app
        .builder
        .object("box-plugin-properties")
        .expect("Couldn't get box-plugin-properties");
    let update_properties: Rc<RefCell<HashMap<String, String>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let properties = GPS::ElementInfo::element_properties(element_name).unwrap();
    while let Some(child) = properties_box.first_child() {
        properties_box.remove(&child);
    }

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(100)
        .width_request(100)
        .build();

    let mut properties: Vec<(&String, &glib::ParamSpec)> = properties.iter().collect();
    properties.sort_by(|a, b| a.0.cmp(b.0));
    let mut i = 0;
    for (name, param) in properties {
        //Entry
        let widget = property_to_widget(
            app,
            node_id,
            element_name,
            name,
            param,
            glib::clone!(@strong update_properties => move |name, value| {
                GPS_TRACE!("property changed: {}:{}", name, value);
                update_properties.borrow_mut().insert(name, value);
            }),
        );
        if let Some(widget) = widget {
            let label = Label::new(Some(name));
            label.set_hexpand(true);
            label.set_halign(gtk::Align::Start);
            label.set_margin_start(4);
            grid.attach(&label, 0, i, 1, 1);
            grid.attach(&widget, 1, i, 1, 1);
            i += 1;
        }
    }
    properties_box.append(&grid);

    let properties_apply_btn: Button = app
        .builder
        .object("button-apply-plugin-properties")
        .expect("Couldn't get button-apply-plugin-properties");

    let app_weak = app.downgrade();
    properties_apply_btn.connect_clicked(
        glib::clone!(@strong update_properties, @weak dialog => move |_| {
            let app = upgrade_weak!(app_weak);
            app.update_element_properties(node_id, &update_properties.borrow());
            dialog.close();
        }),
    );

    dialog.show();
    for p in update_properties.borrow().values() {
        GPS_INFO!("updated properties {}", p);
    }
}
