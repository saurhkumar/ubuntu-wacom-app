#include <gtk/gtk.h>

static void on_button_clicked(GtkWidget *button, GtkWidget *label) {
    // Toggle the visibility of the label
    gboolean visible = gtk_widget_get_visible(label);
    gtk_widget_set_visible(label, !visible);
}

static void activate(GtkApplication *app, gpointer user_data) {
    // Create the main window
    GtkWidget *window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Hello World App");
    gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
    gtk_container_set_border_width(GTK_CONTAINER(window), 10);

    // Try to set an icon
    GError *error = NULL;
    GdkPixbuf *icon = gdk_pixbuf_new_from_file("resources/icon.png", &error);
    if (icon != NULL) {
        gtk_window_set_icon(GTK_WINDOW(window), icon);
        g_object_unref(icon);
    } else {
        g_warning("Could not load icon: %s", error->message);
        g_error_free(error);
    }

    // Create a vertical box
    GtkWidget *vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    gtk_container_add(GTK_CONTAINER(window), vbox);

    // Create a button
    GtkWidget *button = gtk_button_new_with_label("Click Me!");
    gtk_box_pack_start(GTK_BOX(vbox), button, TRUE, TRUE, 0);

    // Create a label (initially hidden)
    GtkWidget *label = gtk_label_new(NULL);
    gtk_label_set_markup(GTK_LABEL(label), "<span size='large' weight='bold'>Hello, World!</span>");
    gtk_widget_set_visible(label, FALSE);
    gtk_box_pack_start(GTK_BOX(vbox), label, TRUE, TRUE, 0);

    // Connect the button click event
    g_signal_connect(button, "clicked", G_CALLBACK(on_button_clicked), label);

    // Show all widgets
    gtk_widget_show_all(window);
}

int main(int argc, char *argv[]) {
    // Create a new application
    GtkApplication *app = gtk_application_new("org.example.HelloWorld", G_APPLICATION_FLAGS_NONE);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    
    // Run the application
    int status = g_application_run(G_APPLICATION(app), argc, argv);
    
    // Clean up
    g_object_unref(app);
    
    return status;
}
