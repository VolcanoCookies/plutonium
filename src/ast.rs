pub enum NodeType {
    Integer,
    Float,
    String,
    Boolean,
    Operator,
    Variable,
    Function,
    Program,
    Block,
}

pub trait Node {
    // Node type
    fn get_type(&self) -> NodeType;
}

pub struct IntegerNode {
    pub value: i32,
}

impl Node for IntegerNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Integer;
    }
}

pub struct FloatNode {
    pub value: f32,
}

impl Node for FloatNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Float;
    }
}

pub struct StringNode {
    pub value: String,
}

impl Node for StringNode {
    fn get_type(&self) -> NodeType {
        return NodeType::String;
    }
}

pub struct BooleanNode {
    pub value: bool,
}

impl Node for BooleanNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Boolean;
    }
}

pub struct OperatorNode<'a> {
    pub operator: String,
    pub left: &'a dyn Node,
    pub right: &'a dyn Node,
}

impl<'a> Node for OperatorNode<'a> {
    fn get_type(&self) -> NodeType {
        return NodeType::Operator;
    }
}

pub struct VariableNode {
    pub name: String,
}

impl Node for VariableNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Variable;
    }
}

pub struct FunctionNode<'a> {
    pub name: String,
    pub args: Vec<&'a dyn Node>,
}

impl<'a> Node for FunctionNode<'a> {
    fn get_type(&self) -> NodeType {
        return NodeType::Function;
    }
}

pub struct ProgramNode {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for ProgramNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Program;
    }
}

pub struct BlockNode {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for BlockNode {
    fn get_type(&self) -> NodeType {
        return NodeType::Block;
    }
}
