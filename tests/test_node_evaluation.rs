use core::logic::nodes::add_node::AddNode;
use core::logic::{InputMap, EvalContext};
use core::types::MetaValue;

#[test]
fn test_add_node_evaluation() {
    let node = AddNode;
    let mut inputs = InputMap::new();
    inputs.insert("a".into(), MetaValue::Scalar(2.0));
    inputs.insert("b".into(), MetaValue::Scalar(3.0));
    let ctx = EvalContext::new(Default::default(), Default::default());
    let result = node.evaluate(&inputs, &ctx).unwrap();
    assert_eq!(result.get("result"), Some(&MetaValue::Scalar(5.0)));
}
