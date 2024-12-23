trait UI {
    fn draw(&self);
}

struct Button;
struct Input;

impl UI for Button {
    fn draw(&self) {
        println!("drawing a button");
    }
}

impl UI for Input {
    fn draw(&self) {
        println!("drawing an input box");
    }
}

fn render(widgets: Vec<Box<dyn UI>>) {
    for w in widgets {
        w.draw();
    }
}

fn main() {
    let btn = Button;
    let input = Input;
    let widgets: Vec<Box<dyn UI>> = vec![Box::new(btn), Box::new(input)];
    render(widgets);
}
