use pandoc_ast::{Block, MutVisitor};
use std::io::{self, Read, Write};

struct DivVisitor;

impl MutVisitor for DivVisitor {
    fn visit_block(&mut self, block: &mut Block) {
        *block = convert_div_to_figure(block.clone());
    }
}

fn convert_div_to_figure(block: Block) -> Block {
    match block {
        Block::Div(attr, blocks) => {
            let (_id, classes, _kvs) = attr.clone();
            
            // // 1. 如果不是 figure，直接放行
            if !classes.contains(&"figure".to_string()) {
                return Block::Div(attr, blocks);
            }

            // 如果是空的 Div，直接转为空 Figure
            if blocks.is_empty() {
                return Block::Figure(attr, (None, Vec::new()), blocks);
            }

            if let Some((last, all_but_last)) = blocks.split_last() {
                // 模式匹配最后一个元素，看它是不是 Para
                if let Block::Para(caption_inlines) = last {
                    let plain = Block::Plain(caption_inlines.clone());
                    // 成功提取 Caption，返回完美的 Figure 结构
                    return Block::Figure(attr, (None, vec![plain]), all_but_last.to_vec());
                }
            }

            // 兜底逻辑：如果 Div 有内容，但最后一个不是 Para
            eprintln!("Warning: Found a .figure Div, but the last element is not a Paragraph. Skipping conversion.");

            Block::Div(attr, blocks)
        }
        _ => block,
    }
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let s = pandoc_ast::filter(s, |mut pandoc| {
        DivVisitor.walk_pandoc(&mut pandoc);
        pandoc
    });
    io::stdout().write(s.as_bytes())?;

    Ok(())
}
