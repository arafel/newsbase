use cursive::Cursive;
use cursive::views::{Dialog, Button, EditView, SelectView, DummyView, LinearLayout};
use cursive::traits::*;

// use sqlite;

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("OK", |s| {
            let name = s.call_on_name("name", |view: &mut EditView| {
                view.get_content()
            }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn main() {
    let mut siv = cursive::default();
    // let connection = sqlite::open("cache/headers/alt.sysadmin.recovery.db").unwrap();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Select a profile"));

    // connection
    //     .iterate("SELECT id, subject FROM headers WHERE sender LIKE '%mikea%'", |results| {
    //         for &(column, value) in results.iter() {
    //             println!("{} = {:?}", column, value);
    //         }
    //         true
    //     })
    //     .unwrap();

    // siv.add_layer(Dialog::around(TextView::new("Hello dialog!"))
    //                      .title("Cursive")
    //                      .button("Quit", |s| s.quit()));

    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
