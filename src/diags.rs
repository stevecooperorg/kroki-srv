use crate::util::{BytesExt, StringExt};

// https://kroki.io/examples.html#blockdiag
pub struct BlockDiagram {
    blocks: Vec<Block>,
    edges: Vec<(usize, usize)>,
}

struct Block {
    label: String,
    color: Option<String>,
}

impl Block {
    fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            color: None,
        }
    }
}

impl Default for BlockDiagram {
    fn default() -> Self {
        Self {
            blocks: vec![],
            edges: vec![],
        }
    }
}

impl BlockDiagram {
    pub fn to_diagram_string(&self) -> String {
        let mut result = String::new();
        result.push_str("blockdiag { ");
        for block in &self.blocks {
            result.push_str(&format!("{}", block.label.quote()));
            if let Some(color) = block.color.as_ref() {
                result.push_str(&format!(r#" [color = "{}"]"#, color));
            }
            result.push_str("; ");
        }
        for (src, tgt) in &self.edges {
            if let (Some(src), Some(tgt)) = (self.blocks.get(*src), self.blocks.get(*tgt)) {
                result.push_str(&format!("{} -> {}; ", src.label.quote(), tgt.label.quote()))
            }
        }
        result.push_str("}");
        result
    }

    pub fn to_kroki(&self) -> String {
        self.to_diagram_string().deflate().base64()
    }

    fn block_index(&self, label: &str) -> Option<usize> {
        self.blocks.iter().position(|block| block.label == label)
    }

    pub fn with_block<S: Into<String>>(self, label: S) -> Self {
        let mut this = self;
        this.ensure_block(label);
        this
    }

    pub fn with_colored_block<S1: Into<String>, S2: Into<String>>(
        self,
        label: S1,
        color: S2,
    ) -> Self {
        let mut this = self;
        let idx = this.ensure_block(label);
        this.set_color(idx, color);
        this
    }

    fn set_color<S1: Into<String>>(&mut self, idx: usize, color: S1) {
        if let Some(block) = self.blocks.get_mut(idx) {
            block.color = Some(color.into());
        }
    }

    fn ensure_block<S: Into<String>>(&mut self, label: S) -> usize {
        let label = label.into();

        match self.block_index(&label).as_ref() {
            Some(idx) => *idx,
            None => {
                self.blocks.push(Block::new(label));
                self.blocks.len() - 1
            }
        }
    }

    pub fn with_edge<S1: Into<String>, S2: Into<String>>(self, src: S1, tgt: S2) -> Self {
        let mut this = self;
        let sidx = this.ensure_block(src);
        let tidx = this.ensure_block(tgt);
        this.edges.push((sidx, tidx));
        this
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_empty_diagram() {
        let diag = BlockDiagram::default();
        let str = diag.to_diagram_string();
        assert_eq!(r#"blockdiag { }"#, &str);

        // just shouldn't fail;
        let _ = diag.to_kroki();
    }

    #[test]
    fn it_generates_diagram() {
        let diag = BlockDiagram::default()
            .with_colored_block("foo", "red")
            .with_block("bar")
            .with_edge("foo", "bar");
        let str = diag.to_diagram_string();
        assert_eq!(
            r#"blockdiag { "foo" [color = "red"]; "bar"; "foo" -> "bar"; }"#,
            &str
        );

        // this is what the kroki website generates
        let b64 = diag.to_kroki();
        assert_eq!(
            "eJxLyslPzk7JTExXqFZQSsvPV1KITs7PyS9SsFVQKkpNUYq1VlBKSixSsobK6trB-LUANFYRig==",
            &b64
        );
    }
}
