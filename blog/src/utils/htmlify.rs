/// Takes a string formatted in Markdown and returns it as sanitized HTML
pub fn transcribe(markdown: &str) -> String {
    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(markdown.into()));
    pandoc.set_output_format(
        pandoc::OutputFormat::Html5,
        vec![
        ]
    );
    pandoc.set_output(pandoc::OutputKind::Pipe);
    // String buffer output
    let res = pandoc.execute().unwrap();
    match res {
        pandoc::PandocOutput::ToBuffer(s) => s,
        _ => String::new()
    }
}

pub fn monthify(num: usize) -> Option<String> {
    let a = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    if num > a.len() {
        None
    } else {
        Some(a[num - 1].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_transcribes_md_as_html() {
        let input = "Hello world, [this is](http://www.google.com/) ~~a~~ *an* example.";
        let result = transcribe(input);
        let expected = "<p>Hello world, <a href=\"http://www.google.com/\">this is</a> <del>a</del> <em>an</em> example.</p>\n";

        assert_eq!(expected, &result);
    }

    #[test]
    fn it_gets_a_month() {
        let a = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        for n in 1..=12 {
            assert_eq!(Some(a[n-1].to_string()), monthify(n));
        }
    }

    #[test]
    fn it_doesnt_get_month_out_of_bounds() {
        assert_eq!(None, monthify(13));
    }
}
