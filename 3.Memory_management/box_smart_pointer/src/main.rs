trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>,
}

fn main() {
    let button_a = Button {
        text: "button a".to_string(),
    };
    let button_b = Box::new(Button {
        text: "button b".to_string(),
    });

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
