pub struct ReportNode {
    pub children: Vec<ReportNode>,
    pub note: String,
    pub act: i64,
}

impl ReportNode {
    pub fn new(notes: &[&str], act: i64) -> ReportNode {
        assert!(notes.len() > 0, "Notes slice should not be zero");
        let note = notes[0].to_string();

        let mut children = vec![];
        if notes.len() > 1 {
            children.push(ReportNode::new(&notes[1..], act));
        }
        ReportNode {
            children,
            note,
            act,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn extend(&mut self, notes: &[&str], act: i64) {
        self.act += act;
        if notes.len() > 0 {
            for child in self.children.iter_mut() {
                if &child.note == notes[0] {
                    child.extend(&notes[1..], act);
                    return;
                }
            }
            self.children.push(ReportNode::new(notes, act));
        }
    }

    pub fn collapse(&mut self) {
        for child in self.children.iter_mut() {
            if !child.is_leaf() {
                child.collapse();
            }
        }
        if self.children.len() == 1 && self.act == self.children[0].act {
            self.note = format!("{} {}", self.note, self.children[0].note);
            self.children = ::std::mem::replace(&mut self.children[0].children, Vec::new());
        }
    }

    pub fn to_string(&self, display_in_hours: bool, root_items_only: bool) -> String {
        self.to_string_producer("", display_in_hours, root_items_only)
    }

    fn to_string_producer(&self, prefix: &str, display_in_hours: bool, root_items_only: bool) -> String {
        let mut string = if display_in_hours {
            format!("{}{}:{:02}  {}", prefix, self.act / 60, self.act % 60, self.note)
        } else {
            format!("{}{}  {}", prefix, self.act, self.note)
        };
        if !root_items_only {
            let prefix = format!("{}  ", prefix);
            for child in self.children.iter() {
                string = format!("{}\n{}", string, child.to_string_producer(&prefix, display_in_hours, root_items_only));
            }
        }
        string
    }
}