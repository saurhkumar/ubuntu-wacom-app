use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box as GtkBox, Label, Image, Orientation};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use glib::timeout_add_local;

const APP_ID: &str = "org.example.WacomMonitorSwitcher";

fn main() {
    // Create a new application
    let app = Application::new(
        Some(APP_ID),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn create_connection_icon(connected: bool) -> Image {
    let icon_name = if connected {
        "gtk-yes" // Green checkmark icon
    } else {
        "gtk-no"  // Red X icon
    };
    
    let image = Image::from_icon_name(Some(icon_name), gtk::IconSize::Dialog);
    image
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::new(app);
    window.set_title("Wacom Monitor Switcher");
    window.set_default_size(400, 250);
    window.set_border_width(10);

    // Set the professional Wacom tablet application icon
    window.set_title("Wacom Tablet Controller");
    
    // Use our professional high-quality icon
    let icon_path = Path::new("resources/wacom_pro_icon_v2.png");
    if icon_path.exists() {
        match window.set_icon_from_file(icon_path) {
            Ok(_) => println!("Successfully set icon from wacom_pro_icon_v2.png"),
            Err(e) => println!("Failed to set icon: {}", e)
        }
    } else {
        // Fallback icons if our professional icon doesn't exist
        let fallback_paths = [
            "resources/wacom_pro_icon.png",
            "resources/new_wacom_icon.png",
            "resources/wacom_tablet_icon.png",
            "resources/tablet_icon.png", 
            "resources/icon.png"
        ];
        
        for path in fallback_paths.iter() {
            let fallback_path = Path::new(path);
            if fallback_path.exists() {
                if window.set_icon_from_file(fallback_path).is_ok() {
                    println!("Using fallback icon: {}", path);
                    break;
                }
            }
        }
    }
    
    // Also try to set the icon name for theme integration
    window.set_icon_name(Some("input-tablet"));

    // Create a vertical box
    let vbox = GtkBox::new(Orientation::Vertical, 10);
    window.add(&vbox);
    
    // Create a connection status area (horizontal box)
    let status_box = GtkBox::new(Orientation::Horizontal, 5);
    vbox.pack_start(&status_box, false, false, 5);
    
    // Check initial connection status
    let is_connected = is_wacom_connected();
    
    // Create connection icon
    let status_icon = create_connection_icon(is_connected);
    status_box.pack_start(&status_icon, false, false, 5);
    
    // Create connection status label
    let status_label = Label::new(None);
    if is_connected {
        status_label.set_markup("<span size='large'>Wacom tablet connected</span>");
    } else {
        status_label.set_markup("<span size='large' color='red'>No Wacom tablet detected</span>");
    }
    status_box.pack_start(&status_label, true, true, 5);
    
    // Add a separator
    let separator = gtk::Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 5);
    
    // Create a button
    let button = Button::with_label("Switch Wacom Monitor");
    vbox.pack_start(&button, true, true, 0);
    
    // Create a status label for switch operations
    let label = Label::new(None);
    label.set_markup("<span size='large'>Click the button to switch monitor mapping</span>");
    vbox.pack_start(&label, true, true, 0);
    
    // Set up periodic connection checking
    let status_icon_ref = status_icon.clone();
    let status_label_ref = status_label.clone();
    let _label_ref = label.clone(); // Keep for potential future use
    let button_ref = button.clone();
    
    // Create a shared state for the connection status
    let was_connected = Rc::new(RefCell::new(is_connected));

    // Create a copy of shared state for the timeout function
    let was_connected_timeout = was_connected.clone();
    let status_icon_timeout = status_icon_ref.clone();
    let status_label_timeout = status_label_ref.clone();
    let button_timeout = button_ref.clone();
    
    // Set up periodic connection check (every 1 second)
    timeout_add_local(Duration::from_millis(1000), move || {
        // Check if tablet connection status changed
        let current_status = is_wacom_connected();
        let mut previous_status = was_connected_timeout.borrow_mut();
        
        // If connection status has changed, update the UI
        if current_status != *previous_status {
            // Update the icon
            // Using direct icon name setting instead of creating a new icon object
            status_icon_timeout.set_from_icon_name(Some(if current_status { "gtk-yes" } else { "gtk-no" }), gtk::IconSize::Dialog);
            
            // Update the status label
            if current_status {
                status_label_timeout.set_markup("<span size='large' color='green'>Wacom tablet connected</span>");
                button_timeout.set_sensitive(true);
            } else {
                status_label_timeout.set_markup("<span size='large' color='red'>No Wacom tablet detected</span>");
                button_timeout.set_sensitive(false);
            }
            
            // Update the stored status
            *previous_status = current_status;
        }
        
        // Return true to keep the timeout active
        glib::Continue(true)
    });
    
    // Connect button click event
    button.connect_clicked(move |_| {
        // First check if tablet is connected
        if !is_wacom_connected() {
            label.set_markup("<span size='large' color='red'>No Wacom tablet detected. Please connect your device.</span>");
            return;
        }
        
        // Execute the Wacom monitor switching command
        let output = switch_wacom_monitors();
        
        // Update the label with the result
        match output {
            Ok(_) => {
                label.set_markup("<span size='large' color='green'>Successfully switched Wacom monitor mapping!</span>");
            },
            Err(err) => {
                label.set_markup(&format!("<span size='large' color='red'>Error: {}</span>", err));
            }
        }
    });
    
    // Initially set button sensitivity based on connection status
    button.set_sensitive(is_connected);

    // Show all widgets
    window.show_all();
}

/// Function to switch Wacom monitors using xsetwacom
/// Check if a Wacom tablet is connected
fn is_wacom_connected() -> bool {
    // Try to get list of Wacom devices
    let devices_output = Command::new("xsetwacom")
        .arg("list")
        .output();
    
    // If command fails, return false
    if let Err(_) = devices_output {
        return false;
    }
    
    let output = devices_output.unwrap();
    if !output.status.success() {
        return false;
    }
    
    let devices_str = String::from_utf8_lossy(&output.stdout);
    
    // Return true if the output is not empty (meaning at least one device found)
    !devices_str.trim().is_empty()
}

fn switch_wacom_monitors() -> Result<(), String> {
    // Get list of Wacom devices
    let devices_output = Command::new("xsetwacom")
        .arg("list")
        .output()
        .map_err(|e| format!("Failed to execute xsetwacom: {}", e))?;
    
    if !devices_output.status.success() {
        return Err(format!("xsetwacom list command failed: {}", 
            String::from_utf8_lossy(&devices_output.stderr)));
    }
    
    let devices_str = String::from_utf8_lossy(&devices_output.stdout);
    
    // No devices found
    if devices_str.trim().is_empty() {
        return Err("No Wacom devices found".to_string());
    }
    
    // Process each device
    for line in devices_str.lines() {
        // Extract device ID from the line
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        
        let id_part = parts[1];
        let id_parts: Vec<&str> = id_part.split(' ').collect();
        if id_parts.len() < 2 {
            continue;
        }
        
        let device_id = id_parts[1];
        
        // Set the device to map to next output
        let status = Command::new("xsetwacom")
            .args(["set", device_id, "maptooutput", "next"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("Failed to set mapping for device {}: {}", device_id, e))?;
        
        if !status.success() {
            return Err(format!("Failed to set mapping for device {}", device_id));
        }
    }
    
    Ok(())
}
