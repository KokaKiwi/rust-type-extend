use data::Extender;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};

pub fn expand(ctx: &mut ExtCtxt, _span: Span, tts: &[TokenTree]) -> Box<MacResult> {
    let mut parser = ctx.new_parser_from_tts(tts);

    let mut items = vec![];
    while let Some(item) = panictry!(parser.parse_item()) {
        match Extender::new(item.unwrap()) {
            Ok(extender) => {
                extender.extend(&mut items);
            }
            Err((span, e)) => {
                ctx.span_err(span, &e);
            }
        }
    }

    MacEager::items(items.into_iter().collect())
}
