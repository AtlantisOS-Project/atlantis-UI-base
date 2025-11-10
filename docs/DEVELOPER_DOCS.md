# Developer Documentation

All functions and modules are listed here.

## `design/create_label_icon.c`

- `GtkWidget *create_label_icon(const char *icon_name, const char *label_text)
{`
- `GtkWidget *create_label_icon_position(const char *icon_name, const char *label_text, GtkAlign alignment)
{`

## `design/home_page.c`

- `void show_home_page(GtkWidget *widget, gpointer stack) 
{`

## `design/create_special_button.c`

- `GtkWidget *create_button(const char *label, GCallback callback, gpointer data) 
{`
- `GtkWidget *create_button_icon(const char *icon_name, const char *label_text, GCallback callback, gpointer data) 
{`
- `GtkWidget *create_button_icon_no_callback(const char *icon_name, const char *label_text) 
{`
- `GtkWidget *create_button_two_icon(const char *first_icon, const char *second_icon, const char *label_text, GCallback callback, gpointer data) 
{`
- `GtkWidget *create_button_icon_position(const char *icon_name, const char *label_text, GCallback callback, gpointer data, GtkAlign alignment)
{`

## `design/create_entry.c`

- `GtkWidget* create_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out)
{`
- `GtkWidget* create_password_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out)
{`

## `design/switch_page.c`

- `void switch_page(GtkWidget *widget, gpointer user_data) 
{`

## `design/chooser/file_chooser.c`

- `static void file_dialog_response_callback(GtkFileDialog *dialog, GAsyncResult *res, gpointer user_data) 
{`
- `void show_file_chooser(GtkWidget *widget, gpointer data) 
{`

## `design/chooser/folder_chooser.c`

- `static void folder_dialog_response_callback(GtkFileDialog *dialog, GAsyncResult *res, gpointer user_data)
{`
- `void show_folder_chooser(GtkWidget *widget, gpointer data)
{`

## `design/dialogs/about_dialog.c`

- `void show_about_dialog(GtkWidget *parent)
{`

## `design/dialogs/dialogs_spinner.c`

- `static gchar *run_command_capture_output(const gchar *cmd)
{`
- `static gpointer run_command_thread(gpointer data)
{`
- `static gpointer run_command_thread_return(gpointer data)
{`
- `gboolean pulse_progress(GtkProgressBar *pbar)
{`
- `void show_spinner_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd)
{`
- `void show_progress_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd)
{`
- `void show_spinner_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data)
{`
- `void show_progress_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data)
{`

## `design/dialogs/dialog_entry.c`

- `static void on_dialog_response(AdwAlertDialog *dialog, const char *response, gpointer user_data)
{`
- `void show_entry_dialog(GtkWidget *parent, const char *title, const char *body, const char *ok_label, const char *cancel_label, const char *entry_label, const char *placeholder, EntryDialogCallback callback, gpointer user_data)
{`

## `design/dialogs/dialog_multi_progress.c`

- `static gboolean update_dialog_description(gpointer data)
{`
- `static gpointer run_command_thread(gpointer data)
{`
- `void show_progress_dialog_multi(GtkWidget *parent, const char *title, const char *body, GSList *commands)
{`

## `design/dialogs/dialog.c`

- `void show_alert_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label)
{`

## `design/dialogs/dialogs.c`

- `void show_info_dialog(GtkWidget *parent, const char *body)
{`
- `void show_info_button_dialog(GtkWidget *parent, const char *body, const char *button_label)
{`
- `void show_dialog_title(GtkWidget *parent, const char *title, const char *body)
{`
- `void show_error_dialog(GtkWidget *parent, const char *body)
{`
- `void show_error_button_dialog(GtkWidget *parent, const char *body, const char *button_label)
{`
- `void show_error_title_dialog(GtkWidget *parent, const char *title, const char *body)
{`
- `void show_error_title_button_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label)
{`

## `helper/file_filter.c`

- `GListStore* load_file_filters(const char *config_path)
{`

## `helper/free_wrapper.c`

- `void free_wrapper(void *p) 
{`

## `helper/get_config_value.c`

- `char *get_config_value(const char *filename, const char *key) 
{`

## `helper/log/write_log_text.c`

- `static void spawn_cb(VteTerminal *terminal, GPid pid, GError *error, gpointer user_data)
{`
- `void log_viewer(void)
{`
- `void kill_program(GtkButton *button, gpointer user_data)
{`
- `GtkWidget *create_custom_headerbar(gpointer stack) 
{`

## `helper/log/write_log.c`

- `int map_level_to_syslog(int level) 
{`
- `void set_logging_mode(int syslog_mode) 
{`
- `const char *get_log_path(void)
{`
- `void close_logging(void) 
{`
- `void log_message(const char *level, int syslog_level, const char *fmt, va_list args) 
{`

## `helper/filesystem/home_dir.c`

- `gchar *get_home(const gchar *path) 
{`
- `const char *get_home_directory() 
{`

## `helper/filesystem/delete_files.c`

- `void delete_files_in_dir(const char *path) 
{`
- `static bool is_mount_point(const char *path, const char *parent_path) 
{`
- `int remove_file(const char *filepath) 
{`
- `static bool is_critical_system_path(const char *path) 
{`
- `static bool is_standard_home_directory(const char *path, const char *home_dir) 
{`
- `static void delete_files_and_parents(const char *path, const char *stop_dir)
{`
- `void delete_files_with_parent(const char *path) 
{`

## `helper/filesystem/make_path.c`

- `void make_directory(const char *path) 
{`
- `int make_path(const char *path)
{`
- `int make_path_dirname(const char *filepath)
{`
- `void create_directory(const char *path) 
{`

## `helper/filesystem/directory_exists.c`

- `int directory_exists(const char *path) 
{`
- `int file_exists(const char *path) 
{`

## `helper/filesystem/delete_directory.c`

- `void delete_directory(const char *path)
{`

## `helper/commands/command_pkexec.c`

- `void command_pkexec(const gchar *command) 
{`
- `void command_pkexec_spinner(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{`
- `void command_pkexec_progressbar(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{`

## `helper/commands/open_url.c`

- `void open_url(const char *url) 
{`

## `helper/commands/open_terminal_desktop.c`

- `void open_terminal_by_desktop(const char *function_command) 
{`

## `helper/commands/execute_command.c`

- `void run_command(const char *command) 
{`
- `char *execute_command(const char *command) 
{`

## `language/bind_language.c`

- `void bind_language(const char *lang)
{`

## `language/language.c`

- `void set_language_dir(const char *dir) 
{`
- `const char *get_language_dir(void) 
{`
- `void init_language(void) 
{`

## `theme/load_adw_provider.c`

- `GtkCssProvider* create_css_provider() 
{`
- `void unload_css_provider(GtkCssProvider *provider) 
{`
- `void load_adw_provider(void) 
{`
- `void adw_theme_changed(AdwStyleManager *style_manager, GParamSpec *pspec, gpointer user_data) 
{`
- `void use_adw_provider(void) 
{`

## `theme/custom_css_adw.c`

- `const char *get_custom_adw_css(void) 
{`

