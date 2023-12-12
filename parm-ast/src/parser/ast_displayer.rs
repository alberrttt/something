#[derive(Debug, Clone, PartialEq)]
pub struct DisplayNode {
    pub subtitle: String,
    pub name: String,
    pub children: Option<Vec<DisplayNode>>,
}

impl DisplayNode {
    pub fn new(name: impl Into<String>) -> Self {
        DisplayNode {
            subtitle: String::new(),
            name: name.into(),
            children: None,
        }
    }
    pub fn with_children(name: String, children: Vec<DisplayNode>) -> Self {
        DisplayNode {
            subtitle: String::new(),
            name,
            children: Some(children),
        }
    }
    pub fn child(mut self, child: DisplayNode) -> Self {
        if let Some(children) = &mut self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }
    pub fn display(&self, depth: usize) {
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str("  ");
        }

        println!("{}{}{}", indent, self.subtitle, self.name);

        if let Some(children) = &self.children {
            for child in children {
                child.display(depth + 1);
            }
        }
    }
}
