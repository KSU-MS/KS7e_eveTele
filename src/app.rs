use gtk::glib::clone;
use gtk::{prelude::*, Box, Button, HeaderBar, Label, Stack, StackSwitcher};
use gtk::{Application, ApplicationWindow, DropDown};

use crate::serial_script;

pub struct App {
    pub window: ApplicationWindow,
    pub container: gtk::Grid,
    pub header: HeaderBar,
    // pub stack_switch: StackSwitcher,
    // pub stack: Stack,
}

impl App {
    pub fn new(app: &Application) {
        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .build();

        // Create a box to throw widgets in
        let container = gtk::Grid::builder()
            .row_spacing(4)
            .column_spacing(4)
            .build();

        // Make DropDown for serial ports
        let wack = serial_script::list_ports();
        let v: Vec<_> = wack.iter().map(|s| s.as_str()).collect();
        let port_drop = DropDown::from_strings(v.as_slice());
        port_drop.set_margin_top(1);
        port_drop.set_margin_bottom(1);

        // Make Button to test the serial port
        let port_test = Button::builder()
            .margin_start(1)
            .margin_end(12)
            .margin_top(1)
            .margin_bottom(1)
            .label("Read selected port")
            .build();

        // Make label for HeaderBar
        let title = Label::new(Some("eveTelemtry or something idk"));

        // Make a header bar to throw the drop down and test into
        let header = HeaderBar::new();
        header.pack_start(&port_test);
        header.pack_end(&port_drop);
        header.set_title_widget(Some(&title));

        // Test the selected port on click
        port_test.connect_clicked(clone!(@strong port_test => move |_| {
        port_test.set_label("Testing, check terminal");
        }));

        // Present window
        window.set_titlebar(Some(&header));
        window.set_child(Some(&container));
        window.present();
    }
}
