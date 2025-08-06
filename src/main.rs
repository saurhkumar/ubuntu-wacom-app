use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box as GtkBox, Label, Orientation};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

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

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::new(app);
    window.set_title("Wacom Monitor Switcher");
    window.set_default_size(300, 200);
    window.set_border_width(10);

    // Try to load and set the tablet application icon
    let icon_path = Path::new("resources/wacom_tablet_icon.png");
    if icon_path.exists() {
        window.set_icon_from_file(icon_path).ok();
    } else {
        // Fallback to the original icon if the tablet icon doesn't exist
        let fallback_icon_path = Path::new("resources/icon.png");
        if fallback_icon_path.exists() {
            window.set_icon_from_file(fallback_icon_path).ok();
        }
    }

    // Create a vertical box
    let vbox = GtkBox::new(Orientation::Vertical, 10);
    window.add(&vbox);

    // Create a button
    let button = Button::with_label("Switch Wacom Monitor");
    vbox.pack_start(&button, true, true, 0);

    // Create a status label
    let label = Label::new(None);
    
    // Check if Wacom tablet is connected
    match is_wacom_connected() {
        true => label.set_markup("<span size='large'>Wacom tablet detected. Click the button to switch monitor mapping.</span>"),
        false => label.set_markup("<span size='large' color='red'>No Wacom tablet detected. Please connect your device.</span>")
    }
    
    vbox.pack_start(&label, true, true, 0);

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
