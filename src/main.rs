use gtk::*;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;

extern crate indexmap;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

mod acls;
mod dps_interface;
mod meter;
mod router;
mod serialization_helpers;
mod vlan_route;

mod faucet_document;

use crate::faucet_document::FaucetDocument;



fn main() {
    let application = gtk::Application::new(Some("com.example.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        
        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).unwrap();
        gio::resources_register(&res);

      

        // Load the CSS
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_resource("/application/css/style.css");
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let color_blind_provider = gtk::CssProvider::new();
        color_blind_provider.load_from_resource("/application/css/colorblindstyle.css");

        // Load the window UI
        let builder = Builder::new_from_resource("/application/button_form.glade");

        // Get a reference to the window
        let window: Window = builder.get_object("main_window").expect("Couldn't get window");
        window.set_application(Some(app));

        let scrolled_window: ScrolledWindow = builder.get_object("scrolled_window").expect("Couldn't get scrolledWindow");

        //let my_label: Label = builder.get_object("label1").expect("Couldn't get label1.");
        let description_label: Label = builder.get_object("descriptionLabel").expect("Couldn't get descriptionLabel.");
        description_label.set_vexpand(true);
        description_label.set_line_wrap(true);
        //let my_button: Button = builder.get_object("button1").expect("Couldn't get button1.");
        let left_pane: Fixed = builder.get_object("leftPane").expect("couldn't get leftPane.");
        let right_pane: Fixed = builder.get_object("rightPane").expect("couldn't get rightPane.");
        // my_button.connect_clicked(glib::clone!(@weak left_pane, @weak description_label, @weak right_pane => move |_|{       
            
        // }));

        let file_open_menu: MenuItem = builder.get_object("file_open_menu").expect("Couldn't get file_open_menu.");
        let view_colorblind_menu: MenuItem = builder.get_object("color_mode").expect("Couldn't get colour blind menu option");
                
        let top_box: Box = builder.get_object("top_box").expect("Couldn't get top box");

        // file_open_menu.connect_activate(glib::clone!(@weak fixed1 => move|_|{
        //     my_label.set_text("open selected");
        // }));
        let file_chooser: FileChooserDialog = builder.get_object("file_chooser").expect("couldn't get filechooser.");
        file_open_menu.connect_activate(glib::clone!(@weak file_chooser => move|_| {
            //my_label.set_text("open selected");
            // let mut file_chooser = FileChooserDialogBuilder::default();
            // file_chooser.build();
            
            //glib::MainContext::default().spawn_local(dialog(window));
            
            file_chooser.run();

        }));


        let file_chooser_cancel: Button = builder.get_object("file_chooser_cancel").expect("couldn't get filechooser_cancel.");
        file_chooser_cancel.connect_clicked(glib::clone!(@weak file_chooser => move |_| {
            //file_chooser.set_visible(false);
            file_chooser.hide();
            //my_label.set_text("cancel selected");
            
        }));

        let file_chooser_select: Button = builder.get_object("file_chooser_select").expect("couldn't get filechooser_select.");
        
        //fn go(file_chooser: glib::WeakRef<FileChooserDialog>, left_pane: glib::WeakRef<Pane>,description_label: glib::WeakRef<Label>, right_pane: glib::WeakRef<Pane>){
        fn go(file_chooser: &FileChooserDialog, left_pane:&Fixed,description_label: &Label, right_pane: &Fixed){
                //clear the screen
            let left_pane_children = left_pane.get_children();
            for child in left_pane_children{
                left_pane.remove(&child);
            }
            description_label.set_text("");


            let file = file_chooser.get_filename();

            let filepath = file.unwrap();
            //my_label.set_text(filepath.to_str().unwrap());
            file_chooser.hide();
            //my_label.set_text("cancel selected");

            let s = read_file(String::from(format!("{}", &filepath.to_str().unwrap())));
            let mut faucet_document: FaucetDocument = serde_yaml::from_str(&s).unwrap();
            let dones = serde_yaml::to_string(&faucet_document).unwrap();
            let mut file = std::fs::File::create("data.yaml").expect("create failed");
            file.write_all(dones.as_bytes()).expect("write failed");
            println!("data written to file");
            description_label.set_text(&format!("{:?}",faucet_document.vlans));
            // right_pane.show_all();


            let mut n: i32 = 0;
            let mut y = 50;
            let mut advance_y = false;
            
            
            //VLANS
            let mut resource = "/application/icons/VLAN.png";
            unsafe{
                if in_colorblind_mode {
                    resource = "/application/icons/VLAN_cb.png";
                }   
            }
            for vlan in faucet_document.vlans {                      
            //for (key, vlan) in faucet_document.vlans.iter_mut(){
                let new_icon = ImageBuilder::new()
                .resource(resource)
                //.tooltip_text("tooltip")
                .build();         

                let new_button = ButtonBuilder::new()
                    .opacity(0.0)
                    .width_request(120)
                    .height_request(100)
                    .build();
                
                
//Update here for new edit function
                new_button.connect_clicked(glib::clone!(@weak description_label, @weak right_pane => move |_|{      
                    //vlan.1.acl_in = String::from("this is a test");
                    //vlan.acl_in = String::from("this is a test");
                    //faucet_document.vlans.get_mut(key).unwrap().acl_in = String::from("this is a test");
                    description_label.set_text(&vlan.1.get_config_string());
                    //description_label.set_text(&vlan.get_config_string());
                    
                    right_pane.show_all();
                }));

                let x = n * 150 + 10;
                //let y = 100 + n * 100;

                left_pane.put(&new_icon,x,y);
                left_pane.put(&new_button,x,y);
                n = n + 1;
                advance_y = true;
            }

            if advance_y {
            y = y + 150;
            }

            //ACLS
            resource = "/application/icons/ACL.png";
            unsafe{
                if in_colorblind_mode {
                    resource = "/application/icons/ACL_cb.png";
                }   
            }
            let mut n: i32 = 0;
            advance_y = false;
            for acl in faucet_document.acls {                       
                let new_icon = ImageBuilder::new()
                    .resource(resource)
                    //.tooltip_text("tooltip")
                    .build();
                
                let new_button = ButtonBuilder::new()
                    .opacity(0.0)
                    .width_request(120)
                    .height_request(100)
                    .build();

//Update here for new edit function
                new_button.connect_clicked(glib::clone!(@weak description_label, @weak right_pane => move |_|{      
                    description_label.set_text(&acl.1.get_config_string());
                    right_pane.show_all();
                }));

                let x = n * 150 + 10;
                //let y = 100 + n * 100;
                left_pane.put(&new_icon,x,y);
                left_pane.put(&new_button,x,y);
                n = n + 1;
                advance_y = true;
            }

            if advance_y {
            y = y + 150;
            }
            //Routers
            resource = "/application/icons/Router.png";
            unsafe{
            if in_colorblind_mode {
                resource = "/application/icons/Route_cb.png";
            }   
            }
            let mut n: i32 = 0;
            advance_y = false;
            for router in faucet_document.routers {                       
                let new_icon = ImageBuilder::new()
                    .resource(resource)
                    //.tooltip_text("tooltip")
                    .build();
                
                let new_button = ButtonBuilder::new()
                .opacity(0.0)
                .width_request(120)
                .height_request(100)
                .build();
                
//Update here for new edit function
                new_button.connect_clicked(glib::clone!(@weak description_label, @weak right_pane => move |_|{      
                    description_label.set_text(&router.1.get_config_string());
                    right_pane.show_all();
                }));

                let x = n * 150 + 10;
                //let y = 100 + n * 100;
                left_pane.put(&new_icon,x,y);
                left_pane.put(&new_button,x,y);
                n = n + 1;
                advance_y = true;
                }
                if advance_y {
                y = y + 150;
                }
                //Switches (DPS)
                resource = "/application/icons/Switch.png";
                unsafe{
                    if in_colorblind_mode {
                        resource = "/application/icons/Switch_cb.png";
                    }   
                }
                let mut n: i32 = 0;
                advance_y = false;
                for switch in faucet_document.dps {                       
                let new_icon = ImageBuilder::new()
                    .resource(resource)
                    //.tooltip_text("tooltip")
                    .build();

                let new_button = ButtonBuilder::new()
                    .opacity(0.0)
                    .width_request(120)
                    .height_request(100)
                    .build();
                
//Update here for new edit function
                new_button.connect_clicked(glib::clone!(@weak description_label, @weak right_pane => move |_|{      
                    description_label.set_text(&switch.1.get_config_string());
                    right_pane.show_all();
                }));

                let x = n * 150 + 10;
                //let y = 100 + n * 100;
                left_pane.put(&new_icon,x,y);
                left_pane.put(&new_button,x,y);
                n = n + 1;
                advance_y = true;
            }

            if advance_y {
            y = y + 150;
            }
            //Meters
            resource = "/application/icons/Meter.png";
            unsafe{
                if in_colorblind_mode {
                    resource = "/application/icons/Meter_cb.png";
                }   
            }
            let mut n: i32 = 0;
            for meter in faucet_document.meters {                       
                let new_icon = ImageBuilder::new()
                    .resource(resource)
                    //.tooltip_text("tooltip")
                    .build();

                let new_button = ButtonBuilder::new()
                    .opacity(0.0)
                    .width_request(120)
                    .height_request(100)
                    .build();
                
//Update here for new edit function
                new_button.connect_clicked(glib::clone!(@weak description_label, @weak right_pane => move |_|{      
                    description_label.set_text(&meter.1.get_config_string());
                    right_pane.show_all();
                }));

                let x = n * 150 + 10;
                //let y = 100 + n * 100;
                left_pane.put(&new_icon,x,y);
                left_pane.put(&new_button,x,y);
                n = n + 1;
            }
            left_pane.show_all();       
        }
        
        file_chooser_select.connect_clicked(glib::clone!(@weak file_chooser,@weak left_pane, @weak description_label, @weak right_pane => move |_| {
            go(&file_chooser,&left_pane,&description_label,&right_pane);
        }));

        static mut in_colorblind_mode: bool = false;
        view_colorblind_menu.connect_activate(glib::clone!(@strong file_chooser_select, @strong color_blind_provider, @strong css_provider, @strong left_pane, @weak description_label=> move|_| {
            toggle_colorblind_mode();
            unsafe{
                if (in_colorblind_mode)
                {
                    StyleContext::remove_provider_for_screen(&gdk::Screen::get_default().expect("Error getting screen on css switch."),&css_provider);
                    StyleContext::add_provider_for_screen(
                        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                        &color_blind_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }
                else
                {
                    StyleContext::remove_provider_for_screen(&gdk::Screen::get_default().expect("Error getting screen on css switch."),&color_blind_provider);
                    StyleContext::add_provider_for_screen(
                        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                        &css_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }
            }
            

            let left_pane_children = left_pane.get_children();
            for child in left_pane_children{
                left_pane.remove(&child);
            }
            description_label.set_text("");
            
            
                go(&file_chooser,&left_pane,&description_label,&right_pane);
        }));

        fn toggle_colorblind_mode(){
            unsafe{
                in_colorblind_mode = !in_colorblind_mode;
            }
            
        }

        // Show the UI
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}



fn read_file(file_path: String) -> String {
    let read_file_result = fs::read_to_string(file_path);
    match read_file_result {
        Ok(x) => {
            return x;
        }
        Err(e) => {
            eprintln!("Could not read file. The error was \"{}\". Exiting.", e);
            std::process::exit(1);
        }
    }
}