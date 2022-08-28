
pub const DATA_VARS: &[&str] = &["%DATA_LOGIC%", "%DATA_IMPORTS%", "%DATA_TYPE%"];

pub const DATA_REPLACERS: &[&str] = &[
    include_str!("./bases/data/DataLogic.in"),
    include_str!("./bases/data/DataImports.in"),
    include_str!("./bases/data/DataType.in"),
];

pub const CHILDREN_VARS: &[&str] = &["%CHILDREN_TYPE%", "%CHILDREN_JSX%", "%CHILDREN_EXISTS%"];

pub const CHILDREN_REPLACERS: &[&str] = &[
    include_str!("./bases/children/ChildrenType.in"),
    include_str!("./bases/children/ChildrenJSX.in"),
    include_str!("./bases/children/ChildrenExists.in"),
];
