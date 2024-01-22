use either::Either;

pub struct File(pub Vec<Statements>);

impl File {
    pub fn new() -> Self {
        File(Vec::new())
    }
}

pub struct Statements(pub Vec<Statement>);


pub enum Statement {
    CompoundStatement(CompoundStatement),
    SimpleStatements(SimpleStatements),
}



pub enum StatmentNewline {
    CompoundStatement(CompoundStatement),
    SimpleStatements(SimpleStatements),
}

pub struct SimpleStatements {
    pub statements: Vec<SimpleStatement>,
}


pub enum SimpleStatement {
    Assignment(Assignment),
    TypeAlias(TypeAlias),
    StarExpr(StarExpr),
    Return(Return),
    Import(Import),
    Raise(Raise),
    Pass,
    Delete(Delete),
    Yield(Yield),
    Assert(Assert),
    Break,
    Continue,
    Global(Global),
    Nonlocal(Nonlocal),
}


pub enum CompoundStatement {
    FunctionDef(FunctionDef),
    If(If),
    ClassDef(ClassDef),
    With(With),
    For(For),
    Try(Try),
    While(While),
    Match(Match),
}

pub enum Assignment {
    Simple(String, Expression, Option<AnnotatedRhs>),
    Targeted(SingleTarget, Expression, Option<AnnotatedRhs>),
    StarTargeted(Vec<StarTarget>, Either<YieldExpr,StarExpressions>, String),
    Augassign(SingleTarget, Augassign, Either<YieldExpr,StarExpressions>),
}

pub enum Augassign {
    Add,
    Sub,
    Mul,
    MatMul,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

pub struct Return {
    pub expr: Option<StarExpressions>,
}

pub struct Raise {
    pub expr: Option<Expression>,
    pub from: Option<Expression>,
}


pub struct Global {
    pub names: Vec<String>,
}

pub struct Nonlocal {
    pub names: Vec<String>,
}

pub struct Delete {
    pub targets: Vec<DeleteTargets>,
}


pub struct YieldStatment {
    pub expr: Option<YieldExpression>,
}

pub struct Assert {
    pub test: Expression,
    pub msg: Option<Expression>,
}


pub enum Import {
    Import(ImportName),
    ImportFrom(ImportFrom),
}

pub struct ImportName {
    pub names: DottedAsNames,
}


pub enum ImportFrom {
    Name(DottedName, ImportFromTargets),
    Dot(ImportFromTargets),
}

pub enum ImportFromTargets {
    Star,
    ImportFromAsNames(ImportFromAsNames),
}


pub struct ImportFromAsNames {
    names: Vec<ImportFromAsName>
}

pub struct ImportFromAsName {
    name: String,
    as_name: Option<String>
}

pub struct DottedAsNames {
    names: Vec<DottedAsName>
}

pub struct DottedAsName {
    name: DottedName,
    as_name: Option<String>,
}

pub enum DottedName {
    DottedName(Box<DottedName>, String),
    Name(String),
}

pub enum Block {
    Statements(Statements),
    SimpleStatements(SimpleStatements)
}

pub struct Decorators {
    decorators: Vec<NamedExpression>
}

pub struct ClassDef {
    decorators: Option<Decorators>,
    raw: ClassDefRaw,
}

pub struct ClassDefRaw {
    name: String,
    type_param: Option<TypeParams>,
    arguments: Option<Arguments>,
    block: Box<Block>,
}

pub struct FunctionDef {
    decorators: Option<Decorators>,
    raw: FunctionDefRaw,
}


pub struct FunctionDefRaw {
    is_async: bool,
    name: String,
    type_params: Option<TypeParams>,
    params: Option<Params>,
    return_type: Expression,
    func_type_comment: Option<String>,
    block: Box<Block>
}

pub struct Params(pub Vec<Parameters>);


pub enum Parameters {
    SlashNoDefault(SlashNoDefault, Vec<ParamNoDefault>,Vec<ParamWithDefault>, Option<StarEtc>),
    SlashWithDefault(SlashWithDefault, Vec<ParamWithDefault>, Option<StarEtc>),
    ParamNoDefault(Vec<ParamNoDefault>, Vec<ParamWithDefault>, Option<StarEtc>),
    ParamWithDefault(Vec<ParamWithDefault>,Option<StarEtc>),
    StarEtc(StarEtc),
}

pub struct SlashNoDefault {
    param_no_default: Vec<ParamNoDefault>,
}

pub struct SlashWithDefault {
    param_no_default: Vec<ParamNoDefault>,
    param_with_default: Vec<ParamWithDefault>,
}

pub enum StarEtc {
    ParamNoDefault(ParamNoDefault,Vec<ParamMaybeDefault>, Option<KeyWords>),
    ParamNoDefaultStarAnnotation(ParamNoDefaultStarAnnotation, Vec<ParamMaybeDefault>, Option<KeyWords>),
    ParamMaybeDefault(Vec<ParamMaybeDefault>, Option<KeyWords>),
    KeyWords(KeyWords),
}

pub struct ParamNoDefault {
    pub param: Param,
    pub type_comment: String,
}

pub struct ParamNoDefaultStarAnnotation {
    pub param_star_annotation: ParamStarAnnotation,
    pub type_comment: String,
}

pub struct ParamWithDefault {
    pub param: Param,
    pub type_comment: String,
}

pub struct ParamMaybeDefault {
    pub param: Param,
    pub default: Option<DefaultAssign>,
    pub type_comment: String,
}

pub struct Param {
    name: String,
    annotation: Option<Annotation>,
}

pub struct ParamStarAnnotation {
    pub name: String,
    pub star_annotation: StarAnnotation,
}

pub struct Annotation {
    pub expression: Expression,
}

pub struct StarAnnotation {
    pub expression: StarExpression,
}

pub enum DefaultAssign {
    Default(Expression),
    Invalid(InvalidDefault),
}


pub struct If {
    pub named_expression: NamedExpression,
    pub block: Box<Block>,
    pub rest: Option<Either<Elif,Else>>
}

pub struct Elif {
    pub named_expression: NamedExpression,
    pub block: Box<Block>,
    pub rest: Either<Box<Elif>,Option<Else>>
}

pub struct Else {
    pub block: Box<Block>
}



















































