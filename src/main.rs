use rusty_gui::Draw;
use rusty_gui::Screen;
use rusty_gui::Button;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box
    }
}

fn main() {
    let screen = Screen { components:
        vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            })
        ]
    };

    screen.run();
}
