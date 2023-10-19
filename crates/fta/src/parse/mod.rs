use std::cell::Cell;

use swc_common::comments::Comment;
use swc_common::sync::Lrc;
use swc_common::{comments::Comments, input::SourceFileInput};
use swc_common::{BytePos, SourceMap};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{error::Error, lexer::Lexer, Parser, Syntax, TsConfig};

mod tests;

pub fn parse_module(
    source: &str,
    use_tsx: bool,
    include_comments: bool,
) -> (Result<Module, Error>, usize) {
    let cm: Lrc<SourceMap> = Default::default();
    let comments = CountingComments::new();
    let code: String = source
        .lines()
        .filter(|line| !line.trim().is_empty()) // Remove lines that are empty or contain only whitespace
        .collect::<Vec<_>>()
        .join("\n");

    let fm = cm.new_source_file(
        swc_common::FileName::Custom("input.ts".to_string()),
        code.clone(),
    );

    let ts_config = TsConfig {
        tsx: use_tsx,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: true,
    };

    let lexer = Lexer::new(
        Syntax::Typescript(ts_config),
        EsVersion::Es2020,
        SourceFileInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);
    let parsed = parser.parse_module();

    println!(
        "Lines: {:?}, Comments: {:?}",
        code.lines().count(),
        comments.count()
    );

    let line_count = if include_comments == true {
        code.lines().count()
    } else {
        code.lines().count() - comments.count()
    };

    (parsed, line_count)
}

struct CountingComments {
    count: Cell<usize>,
}

impl Comments for CountingComments {
    fn add_leading(self: &CountingComments, _pos: BytePos, _comment: Comment) {
        let current_count = self.count.get();
        self.count
            .set(current_count + 1 + _comment.text.matches('\n').count());
    }

    fn add_leading_comments(self: &CountingComments, _pos: BytePos, _comments: Vec<Comment>) {
        let current_count = self.count.get();
        let comment_count: usize = _comments
            .iter()
            .map(|comment| comment.text.matches('\n').count())
            .sum();
        self.count.set(current_count + 1 + comment_count);
    }

    fn add_trailing(self: &CountingComments, _pos: BytePos, _comment: Comment) {
        //         let current_count = self.count.get();
        //         self.count.set(current_count + 1);
    }

    fn add_trailing_comments(self: &CountingComments, _pos: BytePos, _comments: Vec<Comment>) {
        //         let current_count = self.count.get();
        //         self.count.set(current_count + _comments.len());
    }

    fn has_leading(&self, _pos: BytePos) -> bool {
        false
    }

    fn has_trailing(&self, _pos: BytePos) -> bool {
        false
    }

    fn take_leading(self: &CountingComments, _pos: BytePos) -> Option<Vec<Comment>> {
        None
    }

    fn take_trailing(self: &CountingComments, _pos: BytePos) -> Option<Vec<Comment>> {
        None
    }

    fn move_leading(&self, _from: swc_common::BytePos, _to: swc_common::BytePos) {
        todo!()
    }

    fn get_leading(&self, _pos: swc_common::BytePos) -> Option<Vec<swc_common::comments::Comment>> {
        todo!()
    }

    fn move_trailing(&self, _from: swc_common::BytePos, _to: swc_common::BytePos) {
        todo!()
    }

    fn get_trailing(
        &self,
        _pos: swc_common::BytePos,
    ) -> Option<Vec<swc_common::comments::Comment>> {
        todo!()
    }

    fn add_pure_comment(&self, _pos: swc_common::BytePos) {
        todo!()
    }
}

impl CountingComments {
    fn new() -> Self {
        Self {
            count: Cell::new(0),
        }
    }

    fn count(&self) -> usize {
        self.count.get()
    }
}
