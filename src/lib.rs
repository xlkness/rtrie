pub mod trie;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::trie::Tree;

    #[allow(unreachable_code)]
    #[tokio::test]
    async fn test_trie() {
        dbg!("entry test trie..");
        println!("entry test trie");
        let mut tree = Tree::new();
        println!("new trie ok.");
        tree.insert("杀戮".parse().unwrap(), 1);
        tree.insert("杀戮游戏".parse().unwrap(), 1);
        tree.insert("bwi".parse().unwrap(), 2);
        tree.insert("bwin平台".parse().unwrap(), 3);


        let arc_tree = Arc::new(tree);
        {
            // 模拟多协程并发敏感词过滤
            let f1 = async {
                let (new_string, _, ok) = arc_tree.filter_word("杀".parse().unwrap());
                dbg!(&new_string);
                assert_eq!(ok, false);
            };

            let f2 = async {
                let (new_string, results, ok) = arc_tree.filter_word("杀戮1".parse().unwrap());
                dbg!(&new_string);
                assert_eq!(ok, true);
                assert_eq!(new_string, "**1");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].sensitive_type, 1);
                assert_eq!(results[0].match_word, "杀戮");
            };

            let f3 = async {
                let (new_string, results, ok) = arc_tree.filter_word("杀戮游戏戏".parse().unwrap());
                dbg!(&new_string);
                assert_eq!(ok, true);
                assert_eq!(new_string, "****戏");
                assert_eq!(results.len(), 2);
                assert_eq!(results[0].sensitive_type, 1);
                assert_eq!(results[0].match_word, "杀戮");
                assert_eq!(results[1].sensitive_type, 1);
                assert_eq!(results[1].match_word, "杀戮游戏");
            };

            _ = tokio::join!(f1, f2, f3);
        }

        let (new_string, results, ok) = arc_tree.filter_word("bwi".parse().unwrap());
        dbg!(&new_string);
        assert_eq!(ok, true);
        assert_eq!(new_string, "***");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].sensitive_type, 2);
        assert_eq!(results[0].match_word, "bwi");

        let (new_string, results, ok) = arc_tree.filter_word("bwin".parse().unwrap());
        dbg!(&new_string);
        assert_eq!(ok, true);
        assert_eq!(new_string, "***n");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].sensitive_type, 2);
        assert_eq!(results[0].match_word, "bwi");

        let (new_string, results, ok) = arc_tree.filter_word("bwin平台1".parse().unwrap());
        dbg!(&new_string);
        assert_eq!(ok, true);
        assert_eq!(new_string, "******1");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].sensitive_type, 2);
        assert_eq!(results[0].match_word, "bwi");
        assert_eq!(results[1].sensitive_type, 3);
        assert_eq!(results[1].match_word, "bwin平台");

        let (new_string, results, ok) = arc_tree.filter_word("bwin杀戮平台".parse().unwrap());
        dbg!(&new_string);
        assert_eq!(ok, true);
        assert_eq!(new_string, "***n**平台");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].sensitive_type, 2);
        assert_eq!(results[0].match_word, "bwi");
        assert_eq!(results[1].sensitive_type, 1);
        assert_eq!(results[1].match_word, "杀戮");
    }
}
