pub type DocRef<'a> = &'a Doc<'a>;

pub fn pretty_print(doc_ref: DocRef, max_width: u32) -> String {
    let mut printer = PrettyPrinter::new(doc_ref, max_width);
    printer.print()
}

#[derive(PartialEq, Debug)]
pub enum Doc<'a> {
    Newline,
    NewlineWithNoIndent,
    Text(String, u32), // Important: the given text should not contain line breaks
    Flat(DocRef<'a>),
    Softline,  // a space or a newline
    Maybeline, // nil or a newline
    IndentWithMark(u32, DocRef<'a>),
    IndentWithoutMark(u32, DocRef<'a>),
    Dedent(u32, DocRef<'a>),
    Concat(Vec<DocRef<'a>>),
    Choice(DocRef<'a>, DocRef<'a>),
}

struct PrettyPrinter<'a> {
    max_width: u32,
    col: u32,
    chunks: Vec<Chunk<'a>>,
}

pub struct PrettyConfig {
    pub indent_size: u32,
}

impl PrettyConfig {
    pub fn new(indent_size: u32) -> Self {
        if indent_size == 0 {
            panic!("indent_size must be greater than 0")
        } else {
            Self { indent_size }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chunk<'a> {
    doc_ref: DocRef<'a>,
    indent: u32,
    flat: bool,
    indented: bool,
}

impl<'a> Chunk<'a> {
    fn with_doc(self, doc_ref: DocRef<'a>) -> Self {
        Chunk {
            doc_ref,
            indent: self.indent,
            flat: self.flat,
            indented: self.indented,
        }
    }

    fn indent_with_mark(self, indent: u32, doc_ref: DocRef<'a>) -> Self {
        let new_indent = if self.indented {
            self.indent
        } else {
            self.indent + indent
        };
        Chunk {
            doc_ref,
            indent: new_indent,
            flat: self.flat,
            indented: true, // Set flag to true;
        }
    }

    fn indent_without_mark(self, indent: u32, doc_ref: DocRef<'a>) -> Self {
        let new_indent = if self.indented {
            self.indent
        } else {
            self.indent + indent
        };
        Chunk {
            doc_ref,
            indent: new_indent,
            flat: self.flat,
            indented: self.indented, // Keep the flag as it is
        }
    }

    fn dedent_and_unmark(self, indent: u32, doc_ref: DocRef<'a>) -> Self {
        let new_indent = if self.indented {
            self.indent.saturating_sub(indent)
        } else {
            self.indent
        };
        Chunk {
            doc_ref,
            indent: new_indent,
            flat: self.flat,
            indented: false, // Reset indented flag
        }
    }

    fn flat(self, doc_ref: DocRef<'a>) -> Self {
        Chunk {
            doc_ref,
            indent: self.indent,
            flat: true,
            indented: self.indented,
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    fn new(doc_ref: DocRef<'a>, max_width: u32) -> Self {
        let chunk = Chunk {
            doc_ref,
            indent: 0,
            flat: false,
            indented: false,
        };

        Self {
            max_width,
            col: 0,
            chunks: vec![chunk],
        }
    }

    fn print(&mut self) -> String {
        let mut result = String::new();

        while let Some(chunk) = self.chunks.pop() {
            match chunk.doc_ref {
                Doc::Newline => {
                    result.push('\n');
                    for _ in 0..chunk.indent {
                        result.push(' ');
                    }
                    self.col = chunk.indent;
                }
                Doc::Softline => {
                    if chunk.flat {
                        result.push(' ');
                        self.col += 1;
                    } else {
                        result.push('\n');
                        for _ in 0..chunk.indent {
                            result.push(' ');
                        }
                        self.col = chunk.indent;
                    }
                }
                Doc::Maybeline => {
                    if !chunk.flat {
                        result.push('\n');
                        for _ in 0..chunk.indent {
                            result.push(' ');
                        }
                        self.col = chunk.indent;
                    }
                }
                Doc::NewlineWithNoIndent => {
                    result.push('\n');
                    self.col = 0;
                }
                Doc::Text(text, width) => {
                    result.push_str(text);
                    self.col += width;
                }
                Doc::Flat(x) => self.chunks.push(chunk.flat(x)),
                Doc::IndentWithMark(i, x) => self.chunks.push(chunk.indent_with_mark(*i, x)),
                Doc::IndentWithoutMark(i, x) => {
                    self.chunks.push(chunk.indent_without_mark(*i, x))
                }
                Doc::Dedent(i, x) => self.chunks.push(chunk.dedent_and_unmark(*i, x)),
                Doc::Concat(seq) => {
                    for n in seq.iter().rev() {
                        self.chunks.push(chunk.with_doc(n));
                    }
                }
                Doc::Choice(x, y) => {
                    if chunk.flat || self.fits(chunk.with_doc(x)) {
                        self.chunks.push(chunk.with_doc(x));
                    } else {
                        self.chunks.push(chunk.with_doc(y));
                    }
                }
            }
        }
        result
    }

    fn fits(&self, chunk: Chunk<'a>) -> bool {
        let mut remaining_width = self.max_width.saturating_sub(self.col);
        let mut stack = vec![chunk];
        let mut chunks = &self.chunks as &[Chunk];

        loop {
            let chunk = if let Some(chunk) = stack.pop() {
                chunk
            } else if let Some((chunk, more_chunks)) = chunks.split_last() {
                chunks = more_chunks;
                *chunk
            } else {
                return true;
            };

            match chunk.doc_ref {
                Doc::Newline => return true,
                Doc::Softline => {
                    if chunk.flat {
                        if remaining_width >= 1 {
                            remaining_width -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return true;
                    }
                }
                Doc::Maybeline => {
                    if !chunk.flat {
                        return true;
                    }
                }
                Doc::NewlineWithNoIndent => return true,
                Doc::Text(_, text_width) => {
                    if *text_width <= remaining_width {
                        remaining_width -= text_width;
                    } else {
                        return false;
                    }
                }
                Doc::Flat(x) => stack.push(chunk.flat(x)),
                Doc::IndentWithMark(i, x) => stack.push(chunk.indent_with_mark(*i, x)),
                Doc::IndentWithoutMark(i, x) => stack.push(chunk.indent_with_mark(*i, x)),
                Doc::Dedent(i, x) => stack.push(chunk.dedent_and_unmark(*i, x)),
                Doc::Concat(seq) => {
                    for n in seq.iter().rev() {
                        stack.push(chunk.with_doc(n));
                    }
                }
                Doc::Choice(x, y) => {
                    if chunk.flat {
                        stack.push(chunk.with_doc(x));
                    } else {
                        // With assumption: for every choice `x | y`,
                        // the first line of `y` is no longer than the first line of `x`.
                        stack.push(chunk.with_doc(y));
                    }
                }
            }
        }
    }
}
