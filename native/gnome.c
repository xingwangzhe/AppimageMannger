/* AppimageMannger: a real GTK4 / libadwaita frontend for the Rust RPC engine.
 * No AppImage is executed for inspection; every management operation is sent
 * as an argv JSON value to the engine, without invoking a shell. */
#include "i18n_fallback.h"
#include <adwaita.h>
#include <json-glib/json-glib.h>
#include <errno.h>
#include <string.h>

// English fallback is embedded; selected translations arrive from the engine.
static JsonObject *catalog;
static GHashTable *source_messages;
static void init_catalog(void) {
    if (catalog) return;
    JsonParser *parser=json_parser_new();
    if(json_parser_load_from_data(parser,APPIMAGE_EN_JSON,-1,NULL)) catalog=json_object_ref(json_node_get_object(json_parser_get_root(parser)));
    else catalog=json_object_new();
    g_object_unref(parser);
    source_messages=g_hash_table_new_full(g_str_hash,g_str_equal,g_free,g_free);
}
static const gchar *tr_text(const gchar *key) {
    init_catalog();
    const gchar *text=json_object_has_member(catalog,key)?json_object_get_string_member(catalog,key):key;
    g_hash_table_replace(source_messages,g_strdup(key),g_strdup(key));
    g_hash_table_replace(source_messages,g_strdup(text),g_strdup(key));
    return text;
}

static GdkTexture *copy_texture(GtkWidget *widget) {
    if (!GTK_IS_IMAGE(widget)) return NULL;
    GtkIconPaintable *icon = NULL;
    GdkPaintable *paintable = gtk_image_get_paintable(GTK_IMAGE(widget));
    if (paintable && GDK_IS_TEXTURE(paintable)) return g_object_ref(GDK_TEXTURE(paintable));
    GFile *file = NULL;
    if (paintable && GTK_IS_ICON_PAINTABLE(paintable)) file = gtk_icon_paintable_get_file(GTK_ICON_PAINTABLE(paintable));
    GtkIconTheme *theme = gtk_icon_theme_get_for_display(gtk_widget_get_display(widget));
    if (gtk_image_get_storage_type(GTK_IMAGE(widget)) == GTK_IMAGE_ICON_NAME) {
        icon = gtk_icon_theme_lookup_icon(theme, gtk_image_get_icon_name(GTK_IMAGE(widget)), NULL, 128, 1, GTK_TEXT_DIR_NONE, 0);
    } else if (gtk_image_get_storage_type(GTK_IMAGE(widget)) == GTK_IMAGE_GICON) {
        icon = gtk_icon_theme_lookup_by_gicon(theme, gtk_image_get_gicon(GTK_IMAGE(widget)), 128, 1, GTK_TEXT_DIR_NONE, 0);
    }
    if (icon) file = gtk_icon_paintable_get_file(icon);
    gchar *path = file ? g_file_get_path(file) : NULL;
    GError *error = NULL;
    GdkPixbuf *pixels = path ? gdk_pixbuf_new_from_file_at_scale(path, 1024, 1024, TRUE, &error) : NULL;
    GdkTexture *texture = pixels ? gdk_texture_new_for_pixbuf(pixels) : NULL;
    g_clear_object(&pixels); g_clear_object(&icon); g_clear_error(&error); g_free(path);
    return texture;
}
static void copy_content(GtkButton *button, gpointer data) {
    GtkWidget *popover = data;
    const gchar *text = g_object_get_data(G_OBJECT(button), "copy-text");
    GdkTexture *texture = g_object_get_data(G_OBJECT(button), "copy-texture");
    GdkClipboard *clipboard = gtk_widget_get_clipboard(GTK_WIDGET(button));
    if (text) gdk_clipboard_set_text(clipboard, text);
    else if (texture) gdk_clipboard_set_texture(clipboard, texture);
    gtk_popover_popdown(GTK_POPOVER(popover));
}
static void copy_popover_closed(GtkPopover *popover, gpointer data) {
    (void)data; gtk_widget_unparent(GTK_WIDGET(popover));
}
static void copy_menu(GtkGestureClick *gesture, int presses, double x, double y, gpointer data) {
    (void)presses; GtkWidget *widget = data;
    const gchar *text = GTK_IS_LABEL(widget) ? gtk_label_get_text(GTK_LABEL(widget)) : NULL;
    if (GTK_IS_BUTTON(widget)) text = gtk_button_get_label(GTK_BUTTON(widget));
    GdkTexture *texture = copy_texture(widget);
    if ((!text || !*text) && !texture) return;
    GtkWidget *popover = gtk_popover_new(), *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 4);
    gtk_widget_set_parent(popover, widget); gtk_popover_set_child(GTK_POPOVER(popover), box);
    if (text && *text) {
        GtkWidget *button = gtk_button_new_with_label(tr_text("复制文字"));
        g_object_set_data_full(G_OBJECT(button), "copy-text", g_strdup(text), g_free);
        g_signal_connect(button, "clicked", G_CALLBACK(copy_content), popover); gtk_box_append(GTK_BOX(box), button);
    }
    if (texture) {
        GtkWidget *button = gtk_button_new_with_label(tr_text("复制图片"));
        g_object_set_data_full(G_OBJECT(button), "copy-texture", texture, g_object_unref);
        g_signal_connect(button, "clicked", G_CALLBACK(copy_content), popover); gtk_box_append(GTK_BOX(box), button);
    }
    GdkRectangle rectangle = {(int)x, (int)y, 1, 1}; gtk_popover_set_pointing_to(GTK_POPOVER(popover), &rectangle);
    g_signal_connect(popover, "closed", G_CALLBACK(copy_popover_closed), NULL);
    gtk_gesture_set_state(GTK_GESTURE(gesture), GTK_EVENT_SEQUENCE_CLAIMED); gtk_popover_popup(GTK_POPOVER(popover));
}
static gboolean copy_widget_mapped(GSignalInvocationHint *hint, guint count, const GValue *values, gpointer data) {
    (void)hint; (void)count; (void)data;
    GtkWidget *widget = g_value_get_object(&values[0]);
    if (g_object_get_data(G_OBJECT(widget), "copy-enabled")) return TRUE;
    g_object_set_data(G_OBJECT(widget), "copy-enabled", GINT_TO_POINTER(1));
    if (GTK_IS_LABEL(widget)) {
        // Action captions have a context menu; dragging data labels selects text.
        if (!gtk_widget_get_ancestor(widget, GTK_TYPE_BUTTON)) gtk_label_set_selectable(GTK_LABEL(widget), TRUE);
    }
    if (GTK_IS_IMAGE(widget) || GTK_IS_BUTTON(widget) ||
        (GTK_IS_LABEL(widget) && gtk_widget_get_ancestor(widget, GTK_TYPE_BUTTON))) {
        GtkGesture *gesture = gtk_gesture_click_new(); gtk_gesture_single_set_button(GTK_GESTURE_SINGLE(gesture), GDK_BUTTON_SECONDARY);
        g_signal_connect(gesture, "pressed", G_CALLBACK(copy_menu), widget); gtk_widget_add_controller(widget, GTK_EVENT_CONTROLLER(gesture));
    }
    return TRUE;
}
static void retranslate_tree(GtkWidget *widget) {
    if(GTK_IS_LABEL(widget)) {
        const gchar *current=gtk_label_get_text(GTK_LABEL(widget));
        const gchar *stored=g_object_get_data(G_OBJECT(widget),"i18n-msgid");
        const gchar *key=stored?stored:g_hash_table_lookup(source_messages,current);
        if(key){gchar *copy=g_strdup(key);g_object_set_data_full(G_OBJECT(widget),"i18n-msgid",g_strdup(copy),g_free);gtk_label_set_text(GTK_LABEL(widget),tr_text(copy));g_free(copy);}
    }
    if(GTK_IS_ENTRY(widget)) {
        const gchar *placeholder=gtk_entry_get_placeholder_text(GTK_ENTRY(widget));
        const gchar *key=placeholder?g_hash_table_lookup(source_messages,placeholder):NULL;
        if(key){gchar *copy=g_strdup(key);gtk_entry_set_placeholder_text(GTK_ENTRY(widget),tr_text(copy));g_free(copy);}
    }
    for(GtkWidget *child=gtk_widget_get_first_child(widget);child;child=gtk_widget_get_next_sibling(child))retranslate_tree(child);
}
typedef struct State State;
typedef void (*Reply)(State *, JsonNode *, gpointer);
typedef struct {
    State *state;
    gchar *request;
    gchar *message;
    Reply reply;
    gpointer user_data;
    GSubprocess *process;
} Job;
struct State {
    AdwApplication *application;
    GtkWindow *window;
    GtkWidget *content, *list, *search, *count, *status, *spinner;
    GtkWidget *progress;
    GtkWidget *detail, *detail_scroll, *empty, *title, *version, *description, *location;
    GtkWidget *icon, *repository, *update_text, *install_button;
    GtkWindow *settings;
    GtkWidget *directory_entry, *header_title;
    GMenu *menu;
    JsonNode *config, *apps, *asset, *locales;
    gchar *selected, *engine, *application_version;
    GQueue *jobs;
    gboolean running, building, closing, translated;
    guint save_timer;
};

static void scan(State *s);
static void save_config(State *s, const gchar *message, Reply reply, gpointer data);
static void update_detail(State *s);
static void show_settings(State *s);
static void queue_job(State *s, JsonObject *request, const gchar *message, Reply reply, gpointer data);
static void start_next(State *s);

static const gchar *string_member(JsonObject *o, const gchar *key) {
    if (!o || !json_object_has_member(o, key)) return "";
    JsonNode *n = json_object_get_member(o, key);
    return JSON_NODE_HOLDS_VALUE(n) && json_node_get_value_type(n) == G_TYPE_STRING ? json_node_get_string(n) : "";
}
static gboolean bool_member(JsonObject *o, const gchar *key) {
    return o && json_object_has_member(o, key) && json_object_get_boolean_member(o, key);
}
static JsonObject *configuration(State *s) { return s->config ? json_node_get_object(s->config) : NULL; }
static JsonObject *object_member(JsonObject *o, const gchar *key) {
    if (!o || !json_object_has_member(o, key)) return NULL;
    JsonNode *n = json_object_get_member(o, key);
    return JSON_NODE_HOLDS_OBJECT(n) ? json_node_get_object(n) : NULL;
}
static JsonObject *ensure_object(JsonObject *o, const gchar *key) {
    JsonObject *v = object_member(o, key);
    if (!v) { v = json_object_new(); json_object_set_object_member(o, key, v); }
    return v;
}
static JsonObject *request_new(const gchar *action) {
    JsonObject *o = json_object_new(); json_object_set_string_member(o, "action", action); return o;
}
static JsonObject *path_request(State *s, const gchar *action) {
    JsonObject *o = request_new(action);
    json_object_set_string_member(o, "path", s->selected ? s->selected : ""); return o;
}
static void margins(GtkWidget *w, gint n) {
    gtk_widget_set_margin_top(w,n); gtk_widget_set_margin_bottom(w,n);
    gtk_widget_set_margin_start(w,n); gtk_widget_set_margin_end(w,n);
}
static GtkWidget *label_new(const gchar *text, const gchar *css) {
    GtkWidget *w = gtk_label_new(text); gtk_label_set_xalign(GTK_LABEL(w),0);
    gtk_label_set_wrap(GTK_LABEL(w), TRUE);
    if (css) gtk_widget_add_css_class(w,css);
    return w;
}
static GtkWidget *button_new(const gchar *text, const gchar *icon, GCallback callback, State *s) {
    GtkWidget *b = gtk_button_new(); GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL,6);
    if (icon) gtk_box_append(GTK_BOX(box), gtk_image_new_from_icon_name(icon));
    gtk_box_append(GTK_BOX(box), gtk_label_new(text)); gtk_button_set_child(GTK_BUTTON(b),box);
    if (callback) g_signal_connect(b,"clicked",callback,s);
    return b;
}
static void status(State *s, const gchar *message, gboolean error) {
    const gchar *translated=tr_text(message?message:"");
    gchar *display=g_strdup_printf("%s: %s",error?"Error":"Info",translated);
    gtk_label_set_text(GTK_LABEL(s->status),display);g_free(display);
    if (error) gtk_widget_add_css_class(s->status,"error");
    else gtk_widget_remove_css_class(s->status,"error");
}
static void apply_appearance(State *s) {
    gboolean dark = g_strcmp0(string_member(configuration(s),"appearance"),"dark") == 0;
    adw_style_manager_set_color_scheme(adw_style_manager_get_default(),dark ? ADW_COLOR_SCHEME_FORCE_DARK : ADW_COLOR_SCHEME_FORCE_LIGHT);
}
static void remember_geometry(State *s) {
    JsonObject *c=configuration(s); if (!c || !s->window) return;
    gboolean maximized=gtk_window_is_maximized(s->window);
    json_object_set_boolean_member(c,"maximized",maximized);
    int content_width=gtk_widget_get_width(s->content);
    if(content_width>0) json_object_set_double_member(c,"sidebar_fraction",(double)gtk_paned_get_position(GTK_PANED(s->content))/content_width);
    if (!maximized) {
        int w=gtk_widget_get_width(GTK_WIDGET(s->window)), h=gtk_widget_get_height(GTK_WIDGET(s->window));
        if (w>=880 && h>=580) { json_object_set_double_member(c,"window_width",w); json_object_set_double_member(c,"window_height",h); }
    }
}
static void job_free(Job *j) {
    g_clear_object(&j->process); g_free(j->request); g_free(j->message); g_free(j);
}
static void rpc_finished(GObject *source, GAsyncResult *result, gpointer data) {
    Job *j=data; State *s=j->state; gchar *out=NULL,*err=NULL; GError *error=NULL;
    gboolean success=g_subprocess_communicate_utf8_finish(G_SUBPROCESS(source),result,&out,&err,&error);
    if (!success) {status(s,error ? error->message : tr_text("操作进程通讯失败"),TRUE);s->closing=FALSE;}
    else {
        JsonParser *parser=json_parser_new();
        if (!out || !json_parser_load_from_data(parser,out,-1,&error)) {
            status(s,error ? error->message : tr_text("管理引擎没有返回有效响应"),TRUE);s->closing=FALSE;
        }
        else {
            JsonNode *root=json_parser_get_root(parser);
            JsonObject *o=JSON_NODE_HOLDS_OBJECT(root) ? json_node_get_object(root) : NULL;
            if (!o || !bool_member(o,"ok")) {
                const gchar *message=string_member(o,"error");
                status(s,*message ? message : (err && *err ? err : tr_text("操作失败")),TRUE);
                if (s->closing) { s->closing=FALSE; }
            } else {
                status(s,j->message,FALSE);
                if (j->reply) j->reply(s,json_object_get_member(o,"data"),j->user_data);
            }
        }
        g_object_unref(parser);
    }
    g_clear_error(&error); g_free(out); g_free(err); job_free(j);
    s->running=FALSE;
    if (g_queue_is_empty(s->jobs)) { gtk_spinner_stop(GTK_SPINNER(s->spinner)); gtk_widget_set_sensitive(s->content,TRUE);if(s->settings)gtk_widget_set_sensitive(GTK_WIDGET(s->settings),TRUE); }
    start_next(s);
}
static void start_next(State *s) {
    if (s->running || g_queue_is_empty(s->jobs)) return;
    Job *j=g_queue_pop_head(s->jobs); s->running=TRUE;
    gtk_spinner_start(GTK_SPINNER(s->spinner)); gtk_widget_set_sensitive(s->content,FALSE);if(s->settings)gtk_widget_set_sensitive(GTK_WIDGET(s->settings),FALSE);
    status(s,j->message,FALSE);
    GError *error=NULL;
    j->process=g_subprocess_new(G_SUBPROCESS_FLAGS_STDOUT_PIPE|G_SUBPROCESS_FLAGS_STDERR_PIPE,&error,s->engine,"--rpc",j->request,NULL);
    if (!j->process) {
        status(s,error ? error->message : tr_text("无法启动管理引擎"),TRUE); g_clear_error(&error); job_free(j); s->running=FALSE;
        gtk_spinner_stop(GTK_SPINNER(s->spinner)); gtk_widget_set_sensitive(s->content,TRUE);if(s->settings)gtk_widget_set_sensitive(GTK_WIDGET(s->settings),TRUE);s->closing=FALSE; start_next(s); return;
    }
    g_subprocess_communicate_utf8_async(j->process,NULL,NULL,rpc_finished,j);
}
static void queue_job(State *s, JsonObject *request, const gchar *message, Reply reply, gpointer data) {
    JsonNode *n=json_node_new(JSON_NODE_OBJECT); json_node_take_object(n,request);
    JsonGenerator *g=json_generator_new(); json_generator_set_root(g,n);
    Job *j=g_new0(Job,1); j->state=s; j->request=json_generator_to_data(g,NULL);
    j->message=g_strdup(message); j->reply=reply; j->user_data=data;
    json_node_free(n); g_object_unref(g); g_queue_push_tail(s->jobs,j); start_next(s);
}
static void save_config(State *s,const gchar *message,Reply reply,gpointer data) {
    if (!s->config) return;
    JsonObject *r=request_new("save_config"); json_object_set_member(r,"config",json_node_copy(s->config));
    queue_job(s,r,message,reply,data);
}
static gboolean save_later(gpointer data) {
    State *s=data; s->save_timer=0;
    if (!s->closing) save_config(s,tr_text("设置草稿已保存"),NULL,NULL);
    return G_SOURCE_REMOVE;
}
static void schedule_save(State *s) {
    if (s->save_timer) g_source_remove(s->save_timer);
    s->save_timer=g_timeout_add(500,save_later,s);
}
static JsonObject *selected_app(State *s) {
    if (!s->apps || !s->selected) return NULL;
    JsonArray *apps=json_node_get_array(s->apps);
    for (guint i=0;i<json_array_get_length(apps);i++) {
        JsonObject *o=json_array_get_object_element(apps,i);
        if (g_strcmp0(string_member(o,"path"),s->selected)==0) return o;
    }
    return NULL;
}
static void clear_update(State *s) {
    if (s->asset) { json_node_free(s->asset); s->asset=NULL; }
    gtk_widget_set_sensitive(s->install_button,FALSE);
    gtk_label_set_text(GTK_LABEL(s->update_text),tr_text("检查上游 GitHub 的最新稳定版本。"));
}
static void update_detail(State *s) {
    JsonObject *app=selected_app(s); s->building=TRUE;
    gtk_widget_set_visible(s->detail,app!=NULL);gtk_widget_set_visible(s->detail_scroll,app!=NULL); gtk_widget_set_visible(s->empty,app==NULL);
    clear_update(s);
    if (app) {
        gtk_label_set_text(GTK_LABEL(s->title),string_member(app,"name"));
        gchar *size=g_format_size(json_object_get_int_member(app,"size_bytes"));
        gchar *version=g_strdup_printf(tr_text("版本 %s  ·  %s  ·  %s"),string_member(app,"version"),size,string_member(app,"architecture"));
        gtk_label_set_text(GTK_LABEL(s->version),version); g_free(size); g_free(version);
        gtk_label_set_text(GTK_LABEL(s->description),string_member(app,"description"));
        gtk_label_set_text(GTK_LABEL(s->location),string_member(app,"path"));
        const gchar *icon=string_member(app,"icon_path");
        if (*icon && g_file_test(icon,G_FILE_TEST_IS_REGULAR)) gtk_image_set_from_file(GTK_IMAGE(s->icon),icon);
        else gtk_image_set_from_icon_name(GTK_IMAGE(s->icon),"application-x-executable");
        gchar *key=g_path_get_basename(s->selected);
        JsonObject *drafts=object_member(configuration(s),"repository_drafts");
        const gchar *repo=drafts && json_object_has_member(drafts,key) ? string_member(drafts,key) : string_member(app,"repository");
        gtk_editable_set_text(GTK_EDITABLE(s->repository),repo); g_free(key);
    }
    s->building=FALSE;
}
static gboolean row_filter(GtkListBoxRow *row,gpointer data) {
    State *s=data; const gchar *query=gtk_editable_get_text(GTK_EDITABLE(s->search));
    const gchar *name=g_object_get_data(G_OBJECT(row),"name");
    gchar *q=g_utf8_casefold(query,-1),*n=g_utf8_casefold(name ? name : "",-1);
    gboolean match=strstr(n,q)!=NULL; g_free(q);g_free(n);return match;
}
static void on_search(GtkSearchEntry *entry,gpointer data) { (void)entry; State *s=data;gtk_list_box_invalidate_filter(GTK_LIST_BOX(s->list)); }
static void on_selected(GtkListBox *list,GtkListBoxRow *row,gpointer data) {
    (void)list; State *s=data; if (s->building || !row) return;
    g_free(s->selected);s->selected=g_strdup(g_object_get_data(G_OBJECT(row),"path"));
    json_object_set_string_member(configuration(s),"last_selected_path",s->selected);
    update_detail(s); save_config(s,tr_text("应用选择已保存"),NULL,NULL);
}
static void scan_reply(State *s,JsonNode *data,gpointer user_data) {
    (void)user_data; if (!data || !JSON_NODE_HOLDS_OBJECT(data)) {status(s,tr_text("应用列表响应格式无效"),TRUE);return;}
    JsonObject *o=json_node_get_object(data); JsonNode *c=json_object_get_member(o,"config"),*apps=json_object_get_member(o,"apps");
    if (!c || !JSON_NODE_HOLDS_OBJECT(c) || !apps || !JSON_NODE_HOLDS_ARRAY(apps)) {status(s,tr_text("应用配置响应格式无效"),TRUE);return;}
    if (s->config) json_node_free(s->config);
    s->config=json_node_copy(c);
    if (s->apps) json_node_free(s->apps);
    s->apps=json_node_copy(apps);
    g_free(s->application_version);s->application_version=g_strdup(string_member(o,"version"));
    JsonNode *locales=json_object_get_member(o,"locales");
    if(locales && JSON_NODE_HOLDS_ARRAY(locales)){if(s->locales)json_node_free(s->locales);s->locales=json_node_copy(locales);}
    if(!s->translated){
        JsonObject *messages=object_member(o,"translations");
        if(messages){GList *keys=json_object_get_members(messages);for(GList *it=keys;it;it=it->next){const gchar *key=it->data;json_object_set_member(catalog,key,json_node_copy(json_object_get_member(messages,key)));}g_list_free(keys);}
        const gchar *locale=string_member(o,"resolved_language");
        gtk_widget_set_default_direction(g_str_has_prefix(locale,"ar")||g_str_has_prefix(locale,"he")||g_str_has_prefix(locale,"fa")||g_str_has_prefix(locale,"ur")?GTK_TEXT_DIR_RTL:GTK_TEXT_DIR_LTR);
        retranslate_tree(GTK_WIDGET(s->window));
        g_object_set(s->search,"placeholder-text",tr_text("搜索应用名称…"),NULL);
        gtk_window_set_title(s->window,tr_text("AppimageMannger 管理器"));g_set_application_name(tr_text("AppimageMannger 管理器"));adw_window_title_set_title(ADW_WINDOW_TITLE(s->header_title),tr_text("AppimageMannger 管理器"));
        adw_window_title_set_subtitle(ADW_WINDOW_TITLE(s->header_title),tr_text("AppImage 应用管理器"));
        adw_status_page_set_title(ADW_STATUS_PAGE(s->empty),tr_text("让应用各就其位"));
        adw_status_page_set_description(ADW_STATUS_PAGE(s->empty),tr_text("导入 AppImage，或在设置中选择已有应用目录。"));
        g_menu_remove_all(s->menu);g_menu_append(s->menu,tr_text("设置"),"app.settings");g_menu_append(s->menu,tr_text("打开应用目录"),"app.folder");g_menu_append(s->menu,tr_text("关于"),"app.about");g_menu_append(s->menu,tr_text("退出"),"app.quit");
        s->translated=TRUE;
    }
    apply_appearance(s);
    if (!s->selected) s->selected=g_strdup(string_member(configuration(s),"last_selected_path"));
    s->building=TRUE;
    GtkWidget *child;while ((child=gtk_widget_get_first_child(s->list))) gtk_list_box_remove(GTK_LIST_BOX(s->list),child);
    JsonArray *array=json_node_get_array(s->apps);GtkListBoxRow *chosen=NULL;
    for (guint i=0;i<json_array_get_length(array);i++) {
        JsonObject *app=json_array_get_object_element(array,i);
        GtkWidget *row=gtk_list_box_row_new(),*box=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,12);margins(box,10);
        const gchar *icon=string_member(app,"icon_path");GtkWidget *image;
        if (*icon && g_file_test(icon,G_FILE_TEST_IS_REGULAR)) image=gtk_image_new_from_file(icon);
        else image=gtk_image_new_from_icon_name("application-x-executable");
        gtk_image_set_pixel_size(GTK_IMAGE(image),36);gtk_box_append(GTK_BOX(box),image);
        GtkWidget *texts=gtk_box_new(GTK_ORIENTATION_VERTICAL,4);gtk_widget_set_hexpand(texts,TRUE);
        GtkWidget *name=label_new(string_member(app,"name"),"heading");gtk_label_set_ellipsize(GTK_LABEL(name),PANGO_ELLIPSIZE_END);gtk_label_set_wrap(GTK_LABEL(name),FALSE);
        gtk_box_append(GTK_BOX(texts),name);gtk_box_append(GTK_BOX(texts),label_new(string_member(app,"version"),"dim-label"));gtk_box_append(GTK_BOX(box),texts);
        gtk_list_box_row_set_child(GTK_LIST_BOX_ROW(row),box);
        g_object_set_data_full(G_OBJECT(row),"path",g_strdup(string_member(app,"path")),g_free);
        g_object_set_data_full(G_OBJECT(row),"name",g_strdup(string_member(app,"name")),g_free);
        gtk_list_box_append(GTK_LIST_BOX(s->list),row);
        if (g_strcmp0(string_member(app,"path"),s->selected)==0) chosen=GTK_LIST_BOX_ROW(row);
    }
    if (!chosen) chosen=gtk_list_box_get_row_at_index(GTK_LIST_BOX(s->list),0);
    if (chosen) {gtk_list_box_select_row(GTK_LIST_BOX(s->list),chosen);g_free(s->selected);s->selected=g_strdup(g_object_get_data(G_OBJECT(chosen),"path"));}
    else {g_clear_pointer(&s->selected,g_free);}
    gchar *count=g_strdup_printf(tr_text("已安装的应用  ·  %u"),json_array_get_length(array));gtk_label_set_text(GTK_LABEL(s->count),count);g_free(count);
    if (json_object_has_member(configuration(s),"window_width")) {
        int w=(int)json_object_get_double_member(configuration(s),"window_width");int h=(int)json_object_get_double_member(configuration(s),"window_height");
        if (!gtk_widget_get_realized(GTK_WIDGET(s->window)) || !g_object_get_data(G_OBJECT(s->window),"size-restored")) {
            gtk_window_set_default_size(s->window,MAX(w,880),MAX(h,580));
            double fraction=json_object_has_member(configuration(s),"sidebar_fraction")?json_object_get_double_member(configuration(s),"sidebar_fraction"):0.27;
            gtk_paned_set_position(GTK_PANED(s->content),(int)(MAX(w,880)*fraction));
            if (bool_member(configuration(s),"maximized")) gtk_window_maximize(s->window);
            g_object_set_data(G_OBJECT(s->window),"size-restored",GINT_TO_POINTER(1));
        }
    }
    s->building=FALSE;update_detail(s);status(s,tr_text("准备就绪"),FALSE);gtk_window_present(s->window);
}
static void scan_after_save(State *s,JsonNode *data,gpointer user_data) {(void)data;(void)user_data;scan(s);}
static void scan(State *s) {
    if(s->save_timer){g_source_remove(s->save_timer);s->save_timer=0;save_config(s,tr_text("正在保存设置草稿…"),scan_after_save,NULL);return;}
    queue_job(s,request_new("scan"),tr_text("正在扫描 AppImage 应用…"),scan_reply,NULL);
}
static void refreshed(State *s,JsonNode *data,gpointer user_data) { (void)data;(void)user_data;scan(s); }
static void on_refresh(GtkButton *b,gpointer data) { (void)b;scan(data); }
static void on_launch(GtkButton *b,gpointer data) {(void)b;State *s=data;if(s->selected)queue_job(s,path_request(s,"launch"),tr_text("正在启动应用…"),NULL,NULL);}
static void on_folder(GtkButton *b,gpointer data) {(void)b;State *s=data;queue_job(s,request_new("open_directory"),tr_text("正在打开应用目录…"),NULL,NULL);}
static void check_reply(State *s,JsonNode *data,gpointer user_data) {
    (void)user_data;if (!data || !JSON_NODE_HOLDS_OBJECT(data)) return;
    JsonObject *o=json_node_get_object(data);clear_update(s);
    JsonNode *asset=json_object_get_member(o,"asset");
    if (asset && JSON_NODE_HOLDS_OBJECT(asset)) {s->asset=json_node_copy(asset);gtk_widget_set_sensitive(s->install_button,TRUE);}
    gchar *text=g_strdup_printf(tr_text("%s\n当前 %s  →  最新 %s\n\n%s"),tr_text(string_member(o,"status")),string_member(o,"current_version"),string_member(o,"latest_version"),string_member(o,"notes"));
    gtk_label_set_text(GTK_LABEL(s->update_text),text);status(s,string_member(o,"status"),FALSE);g_free(text);
}
static void on_check(GtkButton *b,gpointer data) {(void)b;State *s=data;if(s->selected)queue_job(s,path_request(s,"check_update"),tr_text("正在检查最新稳定版…"),check_reply,NULL);}
static void on_install(GtkButton *b,gpointer data) {
    (void)b;State *s=data;if(!s->selected||!s->asset)return;
    JsonObject *r=path_request(s,"install_update");json_object_set_member(r,"asset",json_node_copy(s->asset));
    queue_job(s,r,tr_text("正在下载并安装更新，完成后保留旧版备份…"),refreshed,NULL);
}
static void source_changed(GtkEditable *entry,gpointer data) {
    State *s=data;if(s->building||!s->selected||!s->config)return;
    gchar *key=g_path_get_basename(s->selected);JsonObject *drafts=ensure_object(configuration(s),"repository_drafts");
    const gchar *text=gtk_editable_get_text(entry);JsonObject *app=selected_app(s);
    if(g_strcmp0(text,string_member(app,"repository"))==0)json_object_remove_member(drafts,key);
    else json_object_set_string_member(drafts,key,text);
    g_free(key);schedule_save(s);
}
static gchar *normalize_repository(const gchar *value) {
    gchar *r=g_strdup(value);g_strstrip(r);
    if(g_str_has_prefix(r,"https://github.com/")){gchar *v=g_strdup(r+19);g_free(r);r=v;}
    while(g_str_has_suffix(r,"/"))r[strlen(r)-1]=0;
    if(g_str_has_suffix(r,".git"))r[strlen(r)-4]=0;
    if(!*r)return r;
    gchar **parts=g_strsplit(r,"/",-1);gboolean valid=g_strv_length(parts)==2;
    for(guint i=0;valid&&parts[i];i++){
        if(!*parts[i]||g_str_equal(parts[i],".")||g_str_equal(parts[i],".."))valid=FALSE;
        for(const gchar *p=parts[i];valid&&*p;p++)if(!g_ascii_isalnum(*p)&&!strchr("-_.",*p))valid=FALSE;
    }
    g_strfreev(parts);if(!valid){g_free(r);return NULL;}return r;
}
static void on_save_source(GtkButton *b,gpointer data) {
    (void)b;State *s=data;if(!s->selected||!s->config)return;
    gchar *repo=normalize_repository(gtk_editable_get_text(GTK_EDITABLE(s->repository)));
    if(!repo){status(s,tr_text("请填写 owner/repository 或 GitHub 仓库 HTTPS 地址"),TRUE);return;}
    gchar *key=g_path_get_basename(s->selected);JsonObject *repos=ensure_object(configuration(s),"repositories");
    if(*repo)json_object_set_string_member(repos,key,repo);else json_object_remove_member(repos,key);
    json_object_remove_member(ensure_object(configuration(s),"repository_drafts"),key);g_free(key);g_free(repo);
    save_config(s,tr_text("更新源已保存"),refreshed,NULL);
}
static void uninstall_response(GtkDialog *dialog,gint response,gpointer data) {
    State *s=data;gtk_window_destroy(GTK_WINDOW(dialog));
    if(response==GTK_RESPONSE_ACCEPT && s->selected)queue_job(s,path_request(s,"uninstall"),tr_text("正在将应用和启动入口移至回收站…"),refreshed,NULL);
}
static void on_integrate(GtkButton *b,gpointer data) {(void)b;State *s=data;if(s->selected)queue_job(s,path_request(s,"integrate"),tr_text("正在更新应用菜单入口…"),refreshed,NULL);}
static void on_uninstall(GtkButton *b,gpointer data) {
    (void)b;State *s=data;JsonObject *app=selected_app(s);if(!app)return;
    GtkWidget *dialog=gtk_message_dialog_new(s->window,GTK_DIALOG_MODAL,GTK_MESSAGE_WARNING,GTK_BUTTONS_NONE,tr_text("确认卸载 %s？"),string_member(app,"name"));
    gtk_message_dialog_format_secondary_text(GTK_MESSAGE_DIALOG(dialog),"%s",tr_text("应用文件与菜单入口会移至回收站，个人配置和数据会保留。"));
    gtk_dialog_add_buttons(GTK_DIALOG(dialog),tr_text("取消"),GTK_RESPONSE_CANCEL,tr_text("移至回收站"),GTK_RESPONSE_ACCEPT,NULL);
    gtk_dialog_set_default_response(GTK_DIALOG(dialog),GTK_RESPONSE_CANCEL);g_signal_connect(dialog,"response",G_CALLBACK(uninstall_response),s);gtk_window_present(GTK_WINDOW(dialog));
}
static void import_response(GtkNativeDialog *dialog,gint response,gpointer data) {
    State *s=data;if(response==GTK_RESPONSE_ACCEPT){
        GFile *file=gtk_file_chooser_get_file(GTK_FILE_CHOOSER(dialog));gchar *path=file ? g_file_get_path(file):NULL;
        if(path){JsonObject *r=request_new("import");json_object_set_string_member(r,"path",path);queue_job(s,r,tr_text("正在导入 AppImage…"),refreshed,NULL);}
        g_free(path);g_clear_object(&file);
    }g_object_unref(dialog);
}
static void on_import(GtkButton *b,gpointer data) {
    (void)b;State *s=data;GtkFileChooserNative *dialog=gtk_file_chooser_native_new(tr_text("导入 AppImage"),s->window,GTK_FILE_CHOOSER_ACTION_OPEN,tr_text("导入"),tr_text("取消"));
    GtkFileFilter *filter=gtk_file_filter_new();gtk_file_filter_set_name(filter,tr_text("AppImage 应用"));gtk_file_filter_add_pattern(filter,"*.AppImage");gtk_file_filter_add_pattern(filter,"*.appimage");gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(dialog),filter);
    GtkFileFilter *all=gtk_file_filter_new();gtk_file_filter_set_name(all,tr_text("所有文件"));gtk_file_filter_add_pattern(all,"*");gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(dialog),all);
    g_signal_connect(dialog,"response",G_CALLBACK(import_response),s);gtk_native_dialog_show(GTK_NATIVE_DIALOG(dialog));
}
static void restart_reply(State *s,JsonNode *data,gpointer user_data) {
    (void)data;gboolean restart=GPOINTER_TO_INT(user_data);apply_appearance(s);
    if(restart){GError *error=NULL;GSubprocess *p=g_subprocess_new(G_SUBPROCESS_FLAGS_NONE,&error,s->engine,NULL);
        if(!p){status(s,error?error->message:tr_text("无法切换界面"),TRUE);g_clear_error(&error);return;}g_object_unref(p);g_application_quit(G_APPLICATION(s->application));}
}
static void theme_changed(GObject *dropdown,GParamSpec *param,gpointer data) {
    (void)param;State *s=data;if(s->building||!s->config)return;
    json_object_set_string_member(configuration(s),"appearance",gtk_drop_down_get_selected(GTK_DROP_DOWN(dropdown))==1?"dark":"light");
    save_config(s,tr_text("外观设置已保存"),restart_reply,GINT_TO_POINTER(FALSE));
}
static void backend_changed(GObject *dropdown,GParamSpec *param,gpointer data) {
    (void)param;State *s=data;if(s->building||!s->config)return;
    const gchar *styles[]={"gpui","qt","gnome"};guint selected=gtk_drop_down_get_selected(GTK_DROP_DOWN(dropdown));if(selected>2)return;
    if(g_strcmp0(string_member(configuration(s),"window_style"),styles[selected])==0)return;
    remember_geometry(s);json_object_set_string_member(configuration(s),"window_style",styles[selected]);
    save_config(s,tr_text("正在保存并切换界面…"),restart_reply,GINT_TO_POINTER(TRUE));
}
static void language_changed(GObject *dropdown,GParamSpec *param,gpointer data) {
    (void)param;State *s=data;if(s->building||!s->config)return;
    guint selected=gtk_drop_down_get_selected(GTK_DROP_DOWN(dropdown));const gchar *code="system";
    if(selected>0&&s->locales){JsonArray *locales=json_node_get_array(s->locales);if(selected-1>=json_array_get_length(locales))return;code=string_member(json_array_get_object_element(locales,selected-1),"code");}
    const gchar *current=string_member(configuration(s),"language");if(!*current)current="system";
    if(g_strcmp0(code,current)==0)return;
    remember_geometry(s);json_object_set_string_member(configuration(s),"language",code);
    save_config(s,tr_text("选择语言后会保存设置并重新打开应用。"),restart_reply,GINT_TO_POINTER(TRUE));
}
static void directory_changed(GtkEditable *entry,gpointer data) {
    State *s=data;if(s->building||!s->config)return;
    const gchar *text=gtk_editable_get_text(entry);
    if(g_strcmp0(text,string_member(configuration(s),"applications_dir"))==0)json_object_set_null_member(configuration(s),"directory_draft");
    else json_object_set_string_member(configuration(s),"directory_draft",text);
    schedule_save(s);
}
static void directory_response(GtkNativeDialog *dialog,gint response,gpointer data) {
    State *s=data;if(response==GTK_RESPONSE_ACCEPT&&s->directory_entry){GFile *file=gtk_file_chooser_get_file(GTK_FILE_CHOOSER(dialog));gchar *path=file?g_file_get_path(file):NULL;if(path)gtk_editable_set_text(GTK_EDITABLE(s->directory_entry),path);g_free(path);g_clear_object(&file);}g_object_unref(dialog);
}
static void choose_directory(GtkButton *button,gpointer data) {
    (void)button;State *s=data;GtkFileChooserNative *dialog=gtk_file_chooser_native_new(tr_text("选择应用目录"),s->settings?s->settings:s->window,GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER,tr_text("选择"),tr_text("取消"));
    g_signal_connect(dialog,"response",G_CALLBACK(directory_response),s);gtk_native_dialog_show(GTK_NATIVE_DIALOG(dialog));
}
static void on_save_directory(GtkButton *button,gpointer data) {
    (void)button;State *s=data;if(!s->directory_entry||!s->config)return;
    const gchar *text=gtk_editable_get_text(GTK_EDITABLE(s->directory_entry));gchar *path;
    if(g_str_has_prefix(text,"~/"))path=g_build_filename(g_get_home_dir(),text+2,NULL);else path=g_strdup(text);
    if(!g_path_is_absolute(path)){status(s,tr_text("应用目录必须是绝对路径"),TRUE);g_free(path);return;}
    for(const gchar *p=path;*p;p++)if(g_ascii_iscntrl(*p)){status(s,tr_text("目录不能包含控制字符"),TRUE);g_free(path);return;}
    if(g_mkdir_with_parents(path,0755)!=0){status(s,g_strerror(errno),TRUE);g_free(path);return;}
    gchar *canonical=g_canonicalize_filename(path,NULL);json_object_set_string_member(configuration(s),"applications_dir",canonical);json_object_set_null_member(configuration(s),"directory_draft");g_free(path);g_free(canonical);
    save_config(s,tr_text("应用目录已保存"),refreshed,NULL);
}
static void settings_response(GtkDialog *dialog,gint response,gpointer data) {(void)response;State *s=data;s->settings=NULL;s->directory_entry=NULL;gtk_window_destroy(GTK_WINDOW(dialog));}
static void show_settings(State *s) {
    if(!s->config)return;
    if(s->settings){gtk_window_present(s->settings);return;}
    if(s->running)return;
    GtkWidget *dialog=gtk_dialog_new_with_buttons(tr_text("设置"),s->window,GTK_DIALOG_MODAL,tr_text("关闭"),GTK_RESPONSE_CLOSE,NULL);s->settings=GTK_WINDOW(dialog);gtk_window_set_default_size(s->settings,620,440);
    GtkWidget *box=gtk_box_new(GTK_ORIENTATION_VERTICAL,16);margins(box,24);gtk_box_append(GTK_BOX(gtk_dialog_get_content_area(GTK_DIALOG(dialog))),box);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("应用目录"),"heading"));
    GtkWidget *pathrow=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,8);s->directory_entry=gtk_entry_new();gtk_widget_set_hexpand(s->directory_entry,TRUE);
    JsonObject *c=configuration(s);const gchar *draft=string_member(c,"directory_draft");gtk_editable_set_text(GTK_EDITABLE(s->directory_entry),*draft?draft:string_member(c,"applications_dir"));
    gtk_box_append(GTK_BOX(pathrow),s->directory_entry);gtk_box_append(GTK_BOX(pathrow),button_new(tr_text("浏览…"),"folder-open-symbolic",G_CALLBACK(choose_directory),s));gtk_box_append(GTK_BOX(box),pathrow);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("更改目录后需要保存；已有应用不会被移动。输入草稿会自动保留。"),"dim-label"));
    GtkWidget *save=button_new(tr_text("保存并扫描"),NULL,G_CALLBACK(on_save_directory),s);gtk_widget_add_css_class(save,"suggested-action");gtk_widget_set_halign(save,GTK_ALIGN_START);gtk_box_append(GTK_BOX(box),save);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("外观"),"heading"));
    const gchar *themes[]={tr_text("浅色"),tr_text("深色"),NULL};GtkWidget *theme=gtk_drop_down_new_from_strings(themes);gtk_drop_down_set_selected(GTK_DROP_DOWN(theme),g_strcmp0(string_member(c,"appearance"),"dark")==0?1:0);gtk_box_append(GTK_BOX(box),theme);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("界面后端"),"heading"));
    const gchar *backends[]={tr_text("GPUI · Ely 原生界面"),tr_text("Qt · 原生 Qt Widgets 界面"),tr_text("GNOME · GTK4 / libadwaita 界面"),NULL};GtkWidget *backend=gtk_drop_down_new_from_strings(backends);const gchar *style=string_member(c,"window_style");gtk_drop_down_set_selected(GTK_DROP_DOWN(backend),g_strcmp0(style,"qt")==0?1:g_strcmp0(style,"gnome")==0?2:0);gtk_box_append(GTK_BOX(box),backend);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("主题立即保存；切换界面后端会保存设置并重新打开应用。"),"dim-label"));
    gtk_box_append(GTK_BOX(box),label_new(tr_text("语言"),"heading"));
    GtkStringList *language_names=gtk_string_list_new(NULL);gtk_string_list_append(language_names,tr_text("跟随系统"));
    guint language_index=0;const gchar *chosen_language=string_member(c,"language");
    if(s->locales){JsonArray *locales=json_node_get_array(s->locales);for(guint i=0;i<json_array_get_length(locales);i++){JsonObject *locale=json_array_get_object_element(locales,i);gtk_string_list_append(language_names,string_member(locale,"name"));if(g_strcmp0(chosen_language,string_member(locale,"code"))==0)language_index=i+1;}}
    GtkWidget *language=gtk_drop_down_new(G_LIST_MODEL(language_names),NULL);gtk_drop_down_set_selected(GTK_DROP_DOWN(language),language_index);gtk_box_append(GTK_BOX(box),language);
    gtk_box_append(GTK_BOX(box),label_new(tr_text("选择语言后会保存设置并重新打开应用。"),"dim-label"));
    g_signal_connect(language,"notify::selected",G_CALLBACK(language_changed),s);
    g_signal_connect(s->directory_entry,"changed",G_CALLBACK(directory_changed),s);g_signal_connect(theme,"notify::selected",G_CALLBACK(theme_changed),s);g_signal_connect(backend,"notify::selected",G_CALLBACK(backend_changed),s);
    g_signal_connect(dialog,"response",G_CALLBACK(settings_response),s);gtk_window_present(s->settings);
}
static void on_settings(GtkButton *b,gpointer data){(void)b;show_settings(data);}
static void on_about(GtkButton *b,gpointer data) {
    (void)b;State *s=data;const gchar *developers[]={"xingwangzhe <xingwangzhe@outlook.com>",NULL};
    GtkWidget *about=adw_about_window_new();adw_about_window_set_application_name(ADW_ABOUT_WINDOW(about),tr_text("AppimageMannger 管理器"));adw_about_window_set_application_icon(ADW_ABOUT_WINDOW(about),"appimage-mannger");
    adw_about_window_set_developer_name(ADW_ABOUT_WINDOW(about),"xingwangzhe");adw_about_window_set_developers(ADW_ABOUT_WINDOW(about),developers);
    adw_about_window_set_version(ADW_ABOUT_WINDOW(about),s->application_version?s->application_version:"");adw_about_window_set_license_type(ADW_ABOUT_WINDOW(about),GTK_LICENSE_AGPL_3_0_ONLY);adw_about_window_set_website(ADW_ABOUT_WINDOW(about),"https://github.com/xingwangzhe/AppimageMannger");
    adw_about_window_add_link(ADW_ABOUT_WINDOW(about), "GitHub · github.com/xingwangzhe", "https://github.com/xingwangzhe");
    gchar *email = g_strdup_printf("%s · xingwangzhe@outlook.com", tr_text("邮箱"));
    gchar *website = g_strdup_printf("%s · xingwangzhe.fun", tr_text("个人网站"));
    adw_about_window_add_link(ADW_ABOUT_WINDOW(about), email, "mailto:xingwangzhe@outlook.com");
    adw_about_window_add_link(ADW_ABOUT_WINDOW(about), website, "https://xingwangzhe.fun");
    g_free(email); g_free(website);
    adw_about_window_set_comments(ADW_ABOUT_WINDOW(about),tr_text("使用真实 GTK4 / libadwaita 原生界面与共享 Rust 管理引擎。"));gtk_window_set_transient_for(GTK_WINDOW(about),s->window);gtk_window_present(GTK_WINDOW(about));
}
static void quit_reply(State *s,JsonNode *data,gpointer user_data){(void)data;(void)user_data;g_application_quit(G_APPLICATION(s->application));}
static gboolean close_window(GtkWindow *window,gpointer data){(void)window;State *s=data;if(s->closing)return TRUE;if(!s->config)return FALSE;s->closing=TRUE;if(s->save_timer){g_source_remove(s->save_timer);s->save_timer=0;}remember_geometry(s);save_config(s,tr_text("正在保存窗口状态…"),quit_reply,NULL);return TRUE;}
static void pane_changed(GObject *object,GParamSpec *spec,gpointer data){(void)object;(void)spec;State *s=data;if(!s->config||s->building||s->closing)return;remember_geometry(s);schedule_save(s);}
static gboolean show_download_progress(gpointer data) {
    State *s=data;JsonObject *app=selected_app(s);gchar *bytes=NULL;gsize length=0;
    const gchar *path=app?string_member(app,"update_progress_path"):"";
    if(!s->running||!*path||!g_file_get_contents(path,&bytes,&length,NULL)){gtk_widget_set_visible(s->progress,FALSE);return G_SOURCE_CONTINUE;}
    JsonParser *parser=json_parser_new();
    if(length<=4096&&json_parser_load_from_data(parser,bytes,(gssize)length,NULL)) {
        JsonNode *root=json_parser_get_root(parser);
        if(JSON_NODE_HOLDS_OBJECT(root)) {
            JsonObject *progress=json_node_get_object(root);
            double downloaded=json_object_get_double_member(progress,"downloaded");
            JsonNode *total_node=json_object_get_member(progress,"total");
            double total=total_node&&!JSON_NODE_HOLDS_NULL(total_node)?json_node_get_double(total_node):0;
            gchar *amount=g_format_size((guint64)downloaded),*text=NULL;
            if(total>0){gchar *size=g_format_size((guint64)total);double fraction=CLAMP(downloaded/total,0,1);gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(s->progress),fraction);text=g_strdup_printf("%.1f%% · %s / %s",fraction*100,amount,size);g_free(size);}
            else {gtk_progress_bar_pulse(GTK_PROGRESS_BAR(s->progress));text=g_strdup(amount);}
            gtk_progress_bar_set_text(GTK_PROGRESS_BAR(s->progress),text);gtk_widget_set_visible(s->progress,TRUE);g_free(amount);g_free(text);
        }
    }
    g_object_unref(parser);g_free(bytes);return G_SOURCE_CONTINUE;
}
static void on_action(GSimpleAction *action,GVariant *parameter,gpointer data){(void)parameter;State *s=data;const gchar *name=g_action_get_name(G_ACTION(action));if(g_str_equal(name,"settings"))show_settings(s);else if(g_str_equal(name,"about"))on_about(NULL,s);else if(g_str_equal(name,"refresh"))scan(s);else if(g_str_equal(name,"import"))on_import(NULL,s);else if(g_str_equal(name,"folder"))on_folder(NULL,s);else if(g_str_equal(name,"quit"))gtk_window_close(s->window);}
static GtkWidget *section(GtkWidget *parent,const gchar *heading) {
    GtkWidget *frame=gtk_box_new(GTK_ORIENTATION_VERTICAL,12);margins(frame,20);GtkWidget *wrapper=gtk_frame_new(NULL);gtk_frame_set_child(GTK_FRAME(wrapper),frame);gtk_box_append(GTK_BOX(frame),label_new(heading,"heading"));gtk_box_append(GTK_BOX(parent),wrapper);return frame;
}
static void activate(GApplication *application,gpointer data) {
    static gboolean copy_hook_installed = FALSE;
    if (!copy_hook_installed) {
        gpointer widget_class = g_type_class_ref(GTK_TYPE_WIDGET);
        g_signal_add_emission_hook(g_signal_lookup("map", GTK_TYPE_WIDGET), 0, copy_widget_mapped, NULL, NULL);
        g_type_class_unref(widget_class);
        copy_hook_installed = TRUE;
    }
    const gchar *brand_icon=g_getenv("APPIMAGE_MAN_NGER_ICON");
    if(brand_icon){gchar *folder=g_path_get_dirname(brand_icon);gtk_icon_theme_add_search_path(gtk_icon_theme_get_for_display(gdk_display_get_default()),folder);g_free(folder);}
    State *s=data;if(s->window){gtk_window_present(s->window);return;}
    s->application=ADW_APPLICATION(application);s->window=GTK_WINDOW(adw_application_window_new(GTK_APPLICATION(application)));gtk_window_set_title(s->window,tr_text("AppimageMannger 管理器"));gtk_window_set_icon_name(s->window,"appimage-mannger");gtk_window_set_default_size(s->window,1160,800);gtk_widget_set_size_request(GTK_WIDGET(s->window),880,580);
    g_signal_connect(s->window,"close-request",G_CALLBACK(close_window),s);
    GtkWidget *root=gtk_box_new(GTK_ORIENTATION_VERTICAL,0);adw_application_window_set_content(ADW_APPLICATION_WINDOW(s->window),root);
    GtkWidget *header=adw_header_bar_new();adw_header_bar_set_decoration_layout(ADW_HEADER_BAR(header),":minimize,maximize,close");s->header_title=adw_window_title_new(tr_text("AppimageMannger 管理器"),tr_text("AppImage 应用管理器"));adw_header_bar_set_title_widget(ADW_HEADER_BAR(header),s->header_title);gtk_box_append(GTK_BOX(root),header);
    GMenu *menu=g_menu_new();s->menu=menu;g_menu_append(menu,tr_text("设置"),"app.settings");g_menu_append(menu,tr_text("打开应用目录"),"app.folder");g_menu_append(menu,tr_text("关于"),"app.about");g_menu_append(menu,tr_text("退出"),"app.quit");GtkWidget *menubutton=gtk_menu_button_new();gtk_menu_button_set_icon_name(GTK_MENU_BUTTON(menubutton),"open-menu-symbolic");gtk_menu_button_set_menu_model(GTK_MENU_BUTTON(menubutton),G_MENU_MODEL(menu));adw_header_bar_pack_start(ADW_HEADER_BAR(header),menubutton);/* State retains menu for retranslation after scan. */
    GtkWidget *refresh=button_new(tr_text("刷新"),"view-refresh-symbolic",G_CALLBACK(on_refresh),s);adw_header_bar_pack_end(ADW_HEADER_BAR(header),refresh);GtkWidget *import=button_new(tr_text("导入"),"list-add-symbolic",G_CALLBACK(on_import),s);gtk_widget_add_css_class(import,"suggested-action");adw_header_bar_pack_end(ADW_HEADER_BAR(header),import);
    s->content=gtk_paned_new(GTK_ORIENTATION_HORIZONTAL);gtk_paned_set_wide_handle(GTK_PANED(s->content),TRUE);gtk_paned_set_shrink_start_child(GTK_PANED(s->content),FALSE);gtk_paned_set_shrink_end_child(GTK_PANED(s->content),FALSE);gtk_widget_set_vexpand(s->content,TRUE);gtk_paned_set_position(GTK_PANED(s->content),290);gtk_box_append(GTK_BOX(root),s->content);g_signal_connect(s->content,"notify::position",G_CALLBACK(pane_changed),s);
    GtkWidget *sidebar=gtk_box_new(GTK_ORIENTATION_VERTICAL,12);gtk_widget_set_size_request(sidebar,260,-1);margins(sidebar,16);s->count=label_new(tr_text("已安装的应用"),"heading");gtk_box_append(GTK_BOX(sidebar),s->count);s->search=gtk_search_entry_new();g_object_set(s->search,"placeholder-text",tr_text("搜索应用名称…"),NULL);gtk_box_append(GTK_BOX(sidebar),s->search);
    s->list=gtk_list_box_new();gtk_list_box_set_selection_mode(GTK_LIST_BOX(s->list),GTK_SELECTION_SINGLE);gtk_widget_add_css_class(s->list,"navigation-sidebar");gtk_list_box_set_filter_func(GTK_LIST_BOX(s->list),row_filter,s,NULL);g_signal_connect(s->list,"row-selected",G_CALLBACK(on_selected),s);g_signal_connect(s->search,"search-changed",G_CALLBACK(on_search),s);
    GtkWidget *list_scroll=gtk_scrolled_window_new();gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(list_scroll),GTK_POLICY_NEVER,GTK_POLICY_AUTOMATIC);gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(list_scroll),s->list);gtk_widget_set_vexpand(list_scroll,TRUE);gtk_box_append(GTK_BOX(sidebar),list_scroll);gtk_box_append(GTK_BOX(sidebar),button_new(tr_text("设置与导入"),"emblem-system-symbolic",G_CALLBACK(on_settings),s));gtk_paned_set_start_child(GTK_PANED(s->content),sidebar);gtk_paned_set_resize_start_child(GTK_PANED(s->content),FALSE);
    GtkWidget *right=gtk_box_new(GTK_ORIENTATION_VERTICAL,0);s->empty=adw_status_page_new();adw_status_page_set_icon_name(ADW_STATUS_PAGE(s->empty),"application-x-executable-symbolic");adw_status_page_set_title(ADW_STATUS_PAGE(s->empty),tr_text("让应用各就其位"));adw_status_page_set_description(ADW_STATUS_PAGE(s->empty),tr_text("导入 AppImage，或在设置中选择已有应用目录。"));gtk_widget_set_vexpand(s->empty,TRUE);gtk_box_append(GTK_BOX(right),s->empty);
    s->detail=gtk_box_new(GTK_ORIENTATION_VERTICAL,20);margins(s->detail,28);gtk_widget_set_visible(s->detail,FALSE);
    GtkWidget *intro=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,18);s->icon=gtk_image_new_from_icon_name("application-x-executable");gtk_image_set_pixel_size(GTK_IMAGE(s->icon),72);gtk_box_append(GTK_BOX(intro),s->icon);GtkWidget *titles=gtk_box_new(GTK_ORIENTATION_VERTICAL,6);gtk_widget_set_hexpand(titles,TRUE);s->title=label_new("","title-1");s->version=label_new("","dim-label");gtk_box_append(GTK_BOX(titles),s->title);gtk_box_append(GTK_BOX(titles),s->version);gtk_box_append(GTK_BOX(intro),titles);GtkWidget *launch=button_new(tr_text("启动应用"),"media-playback-start-symbolic",G_CALLBACK(on_launch),s);gtk_widget_add_css_class(launch,"suggested-action");gtk_widget_set_valign(launch,GTK_ALIGN_CENTER);gtk_box_append(GTK_BOX(intro),launch);gtk_box_append(GTK_BOX(s->detail),intro);
    GtkWidget *information=section(s->detail,tr_text("应用信息"));s->description=label_new("",NULL);s->location=label_new("",NULL);gtk_label_set_selectable(GTK_LABEL(s->location),TRUE);gtk_box_append(GTK_BOX(information),s->description);gtk_box_append(GTK_BOX(information),label_new(tr_text("安装位置"),"dim-label"));gtk_box_append(GTK_BOX(information),s->location);GtkWidget *folder=button_new(tr_text("打开所在目录"),"folder-open-symbolic",G_CALLBACK(on_folder),s);gtk_widget_set_halign(folder,GTK_ALIGN_START);gtk_box_append(GTK_BOX(information),folder);
    GtkWidget *updates=section(s->detail,tr_text("版本与更新"));gtk_box_append(GTK_BOX(updates),label_new(tr_text("GitHub 更新仓库"),"dim-label"));GtkWidget *repo_row=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,8);s->repository=gtk_entry_new();gtk_entry_set_placeholder_text(GTK_ENTRY(s->repository),"owner/repository");gtk_widget_set_hexpand(s->repository,TRUE);g_signal_connect(s->repository,"changed",G_CALLBACK(source_changed),s);gtk_box_append(GTK_BOX(repo_row),s->repository);gtk_box_append(GTK_BOX(repo_row),button_new(tr_text("保存更新源"),NULL,G_CALLBACK(on_save_source),s));gtk_box_append(GTK_BOX(updates),repo_row);
    GtkWidget *update_buttons=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,8);gtk_box_append(GTK_BOX(update_buttons),button_new(tr_text("检查更新"),"view-refresh-symbolic",G_CALLBACK(on_check),s));s->install_button=button_new(tr_text("安装更新"),"folder-download-symbolic",G_CALLBACK(on_install),s);gtk_widget_add_css_class(s->install_button,"suggested-action");gtk_widget_set_sensitive(s->install_button,FALSE);gtk_box_append(GTK_BOX(update_buttons),s->install_button);gtk_box_append(GTK_BOX(updates),update_buttons);s->update_text=label_new(tr_text("检查上游 GitHub 的最新稳定版本。"),"dim-label");gtk_label_set_selectable(GTK_LABEL(s->update_text),TRUE);gtk_box_append(GTK_BOX(updates),s->update_text);
    GtkWidget *management=section(s->detail,tr_text("应用管理"));gtk_box_append(GTK_BOX(management),label_new(tr_text("卸载会移至回收站，保留个人配置和数据。"),"dim-label"));gtk_box_append(GTK_BOX(management),button_new(tr_text("修复菜单入口"),"application-menu-symbolic",G_CALLBACK(on_integrate),s));GtkWidget *remove=button_new(tr_text("卸载应用…"),"user-trash-symbolic",G_CALLBACK(on_uninstall),s);gtk_widget_add_css_class(remove,"destructive-action");gtk_widget_set_halign(remove,GTK_ALIGN_START);gtk_box_append(GTK_BOX(management),remove);
    GtkWidget *detail_scroll=gtk_scrolled_window_new();s->detail_scroll=detail_scroll;gtk_widget_set_visible(detail_scroll,FALSE);gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(detail_scroll),GTK_POLICY_NEVER,GTK_POLICY_AUTOMATIC);gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(detail_scroll),s->detail);gtk_widget_set_vexpand(detail_scroll,TRUE);gtk_box_append(GTK_BOX(right),detail_scroll);gtk_paned_set_end_child(GTK_PANED(s->content),right);
    GtkWidget *footer=gtk_box_new(GTK_ORIENTATION_HORIZONTAL,8);margins(footer,10);s->spinner=gtk_spinner_new();gtk_box_append(GTK_BOX(footer),s->spinner);s->status=label_new(tr_text("准备就绪"),NULL);gtk_widget_set_hexpand(s->status,TRUE);gtk_label_set_wrap(GTK_LABEL(s->status),FALSE);gtk_label_set_ellipsize(GTK_LABEL(s->status),PANGO_ELLIPSIZE_END);gtk_box_append(GTK_BOX(footer),s->status);s->progress=gtk_progress_bar_new();gtk_progress_bar_set_show_text(GTK_PROGRESS_BAR(s->progress),TRUE);gtk_widget_set_size_request(s->progress,260,-1);gtk_widget_set_visible(s->progress,FALSE);gtk_box_append(GTK_BOX(footer),s->progress);gtk_box_append(GTK_BOX(root),footer);
    const gchar *actions[]={"settings","about","refresh","import","folder","quit",NULL};for(guint i=0;actions[i];i++){GSimpleAction *a=g_simple_action_new(actions[i],NULL);g_signal_connect(a,"activate",G_CALLBACK(on_action),s);g_action_map_add_action(G_ACTION_MAP(application),G_ACTION(a));g_object_unref(a);}
    const gchar *refresh_keys[]={"<Primary>r",NULL},*import_keys[]={"<Primary>o",NULL},*settings_keys[]={"<Primary>comma",NULL},*quit_keys[]={"<Primary>q",NULL};gtk_application_set_accels_for_action(GTK_APPLICATION(application),"app.refresh",refresh_keys);gtk_application_set_accels_for_action(GTK_APPLICATION(application),"app.import",import_keys);gtk_application_set_accels_for_action(GTK_APPLICATION(application),"app.settings",settings_keys);gtk_application_set_accels_for_action(GTK_APPLICATION(application),"app.quit",quit_keys);
    /* Embedded English covers startup and transport errors; scan applies the selected locale. */
    g_timeout_add(350,show_download_progress,s);gtk_window_present(s->window);scan(s);
}
int main(int argc,char **argv) {
    init_catalog();
    g_set_prgname("appimage-mannger");g_set_application_name("AppimageMannger");
    State *s=g_new0(State,1);s->jobs=g_queue_new();const gchar *engine=g_getenv("APPIMAGE_MAN_NGER_ENGINE");s->engine=g_strdup(engine&&*engine?engine:"/usr/bin/appimage-mannger");
    if(!g_path_is_absolute(s->engine)){g_printerr("APPIMAGE_MAN_NGER_ENGINE must be an absolute path\n");return 2;}
    AdwApplication *app=adw_application_new("io.github.xingwangzhe.AppimageMannger",G_APPLICATION_NON_UNIQUE);g_signal_connect(app,"activate",G_CALLBACK(activate),s);
    int result=g_application_run(G_APPLICATION(app),argc,argv);g_object_unref(app);
    /* Pending asynchronous jobs own State until process exit. */
    return result;
}
