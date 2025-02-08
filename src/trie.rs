#[allow(unused)]
#[derive(Debug)]
pub struct Tree {
    root: Node,
}

#[allow(unused)]
impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Node::empty(),
        }
    }
    pub fn insert(&mut self, content: String, sensitive_type: i32) {
        self.root.insert(content.chars().collect(), sensitive_type);
    }
    pub fn filter_word(&self, content: String) -> (String, Vec<FilterRecord>, bool) {
        let raw_content = content.chars().collect::<Vec<char>>();
        let mut replaced_content: Vec<char> = content.chars().collect();
        let mut total_records = vec![];
        let mut filter_flag = false;

        let raw_content1 = &raw_content;
        let tmp_total_records = &mut total_records;
        for (i, _c) in raw_content1.into_iter().enumerate() {
            let mut records = FilterRecords {
                records: vec![],
                match_content: String::new(),
                match_char_num: 0,
            };
            self.root
                .filter_word(raw_content[i..].to_vec(), &mut records);
            if records.records.len() > 0 {
                filter_flag = true;
                for record in &records.records {
                    let max = i + record.match_char_num as usize;
                    dbg!(_c, i, record.match_char_num);
                    for j in i..max {
                        replaced_content[j] = '*';
                    }
                }

                tmp_total_records.append(&mut records.records);
            }
        }
        (
            replaced_content.iter().collect(),
            total_records,
            filter_flag,
        )
    }
}

#[allow(unused)]
pub struct FilterRecord {
    pub match_word: String,
    pub match_char_num: u32,
    pub sensitive_type: i32,
}

struct FilterRecords {
    records: Vec<FilterRecord>,
    match_content: String,
    match_char_num: u32,
}

#[derive(Debug)]
struct Node {
    c: char,
    sensitive_type: i32, // 敏感词类型，例如宗教信仰、政治、涉黄等
    is_full_word: bool,
    children: Vec<Node>,
}

impl Node {
    fn empty() -> Node {
        Node {
            c: '\u{0000}',
            sensitive_type: 0,
            is_full_word: false,
            children: vec![],
        }
    }
    fn new(c: char, sensitive_type: i32) -> Self {
        Self {
            c,
            sensitive_type,
            is_full_word: false,
            children: vec![],
        }
    }
    fn insert(&mut self, content: Vec<char>, sensitive_type: i32) {
        if content.len() == 0 {
            self.sensitive_type = sensitive_type;
            self.is_full_word = true;
            return;
        }

        let cur_char = content[0];

        let mut find = false;
        for child in self.children.iter_mut() {
            if child.c == cur_char {
                child.insert(content[1..].to_vec(), sensitive_type);
                find = true;
                break;
            }
        }
        if !find {
            let mut child = Node::new(cur_char, sensitive_type);
            child.insert(content[1..].to_vec(), sensitive_type);
            self.children.push(child);
        }
    }
    fn filter_word(&self, content: Vec<char>, records: &mut FilterRecords) {
        if self.is_full_word {
            // 找到当前字符就已经组成违禁词了
            let mut record = FilterRecord {
                match_word: records.match_content.clone(),
                sensitive_type: self.sensitive_type,
                match_char_num: records.match_char_num,
            };
            record.sensitive_type = self.sensitive_type;
            records.records.push(record);
        }
        if content.len() == 0 {
            // 过滤内容被找完了还没找到完整的词，返回没找到
            return;
        }

        let cur_char = content[0];

        for child in self.children.iter() {
            if child.c == cur_char {
                records.match_content += child.c.to_string().as_str();
                records.match_char_num += 1;
                return child.filter_word(content[1..].to_vec(), records);
            }
        }
    }
}
