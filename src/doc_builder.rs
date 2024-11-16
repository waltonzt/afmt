use crate::{
    data_model::DocBuild,
    doc::{Doc, DocRef, PrettyConfig},
    enum_def::FormattedMember,
};
use typed_arena::Arena;

pub struct DocBuilder<'a> {
    arena: Arena<Doc<'a>>,
    config: PrettyConfig,
}

impl<'a> DocBuilder<'a> {
    pub fn new(config: PrettyConfig) -> Self {
        Self {
            arena: Arena::new(),
            config,
        }
    }

    pub fn nil(&'a self) -> DocRef<'a> {
        self.txt("")
    }

    pub fn group_list(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        let choice = self.intersperse_with_softline(&elems, &sep);
        self.add_indent_level(self.group(choice))
    }

    pub fn intersperse_with_softline(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
                parts.push(self.softline());
            }
            parts.push(elem);
        }

        self.concat(parts)
    }

    /// flatted all elements
    pub fn intersperse_single_line(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
            }
            parts.push(self.flat(elem));
        }

        self.concat(parts)
    }

    pub fn surrounded_choice(
        &'a self,
        elems: &[DocRef<'a>],
        single_sep: &str,
        multi_sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        let single_line = self.surrounded_single_line(elems, single_sep, open, close);
        let multi_line = self.surrounded_multi_line(elems, multi_sep, open, close);

        self.choice(single_line, multi_line)
    }

    fn surrounded_single_line(
        &'a self,
        elems: &[DocRef<'a>],
        single_sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let single_line = self.concat(vec![
            self.txt(open),
            self.intersperse_single_line(elems, single_sep),
            self.txt(close),
        ]);
        single_line
    }

    pub fn surrounded_multi_line(
        &'a self,
        elems: &[DocRef<'a>],
        multi_sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.nl()),
            self.add_indent_level(self.intersperse_with_softline(elems, multi_sep)),
            self.nl(),
            self.txt(close),
        ]);
        multi_line
    }

    pub fn build_docs<'b, T: DocBuild<'a>>(
        &'a self,
        items: impl IntoIterator<Item = &'b T>,
    ) -> Vec<DocRef<'a>>
    where
        T: DocBuild<'a> + 'b,
    {
        items.into_iter().map(|item| item.build(self)).collect()
    }

    pub fn sep_with_trailing_newlines<'b, M>(&'a self, members: &[FormattedMember<M>]) -> DocRef<'a>
    where
        M: DocBuild<'a>,
    {
        let mut member_docs = Vec::new();

        for (i, m) in members.iter().enumerate() {
            member_docs.push(m.member.build(self));

            if i < members.len() - 1 {
                if m.has_trailing_newlines {
                    member_docs.push(self.nl_with_no_indent());
                }
                member_docs.push(self.nl());
            }
        }
        self.concat(member_docs)
    }

    pub fn nl(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Newline)
    }

    pub fn softline(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Softline)
    }

    pub fn maybeline(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Maybeline)
    }

    pub fn nl_with_no_indent(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::NewlineWithNoIndent)
    }

    pub fn txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let width = s.len() as u32;
        self.arena.alloc(Doc::Text(s, width))
    }

    pub fn _txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let space_s = format!(" {}", s);
        self.txt(space_s)
    }

    pub fn txt_(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let s_space = format!("{} ", s);
        self.txt(s_space)
    }

    pub fn _txt_(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let space_s_space = format!(" {} ", s);
        self.txt(space_s_space)
    }

    pub fn flat(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        self.arena.alloc(Doc::Flat(doc_ref))
    }

    pub fn add_indent_level(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        let relative_indent = self.config.indent_size;
        self.arena.alloc(Doc::Indent(relative_indent, doc_ref))
    }

    pub fn concat(&'a self, doc_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let n_vec = doc_refs.into_iter().collect::<Vec<_>>();
        self.arena.alloc(Doc::Concat(n_vec))
    }

    pub fn choice(&'a self, first: DocRef<'a>, second: DocRef<'a>) -> DocRef<'a> {
        self.arena.alloc(Doc::Choice(first, second))
    }

    pub fn group(&'a self, doc: DocRef<'a>) -> DocRef<'a> {
        self.choice(self.flat(doc), doc)
    }
}
