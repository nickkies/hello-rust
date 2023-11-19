// 1. Each parameter that is a reference gets its own liftime paramter.
// 2. If there is exactly one input lifetime parameter, that liftime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters.
struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    let mut tweet = Tweet { content: "Ferris" };

    let old_content = tweet.replace_content("Ferrris");

    println!("{old_content}");
    println!("{}", tweet.content);
}

fn _take_and_return_content<'a, 'b>(content: &'a str, _content2: &'b str) -> &'a str {
    content
}
