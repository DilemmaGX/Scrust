use crate::ast::*;
use crate::sb3::{Block, Comment, Costume, Field, Input, Mutation, NormalBlock, Sound, Target};
use md5;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProcedureInfo {
    pub proccode: String,
    pub arg_ids: Vec<String>,
    pub arg_names: Vec<String>,
    pub warp: bool,
}

pub struct CompilerContext<'a> {
    pub blocks: HashMap<String, Block>,
    pub variables: HashMap<String, (String, Value)>,
    pub lists: HashMap<String, (String, Vec<Value>)>,
    pub broadcast_map: HashMap<String, String>,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub comments: HashMap<String, Comment>,
    pub pending_comment: Option<String>,
    pub global_variables: Option<&'a HashMap<String, (String, Value)>>,
    pub global_lists: Option<&'a HashMap<String, (String, Vec<Value>)>>,
    pub asset_instructions: Vec<(PathBuf, String)>,
    pub procedures: HashMap<String, ProcedureInfo>,
    pub current_proc_args: Option<HashMap<String, Type>>,
}

impl<'a> CompilerContext<'a> {
    pub fn new(
        global_variables: Option<&'a HashMap<String, (String, Value)>>,
        global_lists: Option<&'a HashMap<String, (String, Vec<Value>)>>,
    ) -> Self {
        Self {
            blocks: HashMap::new(),
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcast_map: HashMap::new(),
            costumes: Vec::new(),
            sounds: Vec::new(),
            comments: HashMap::new(),
            pending_comment: None,
            global_variables,
            global_lists,
            asset_instructions: Vec::new(),
            procedures: HashMap::new(),
            current_proc_args: None,
        }
    }

    pub fn add_block(&mut self, block: NormalBlock) -> String {
        let id = Uuid::new_v4().to_string();
        self.blocks.insert(id.clone(), Block::Normal(block));
        id
    }

    pub fn add_variable(&mut self, name: String, val: Value) -> String {
        let id = Uuid::new_v4().to_string();
        self.variables.insert(id.clone(), (name, val));
        id
    }

    pub fn add_list(&mut self, name: String, val: Vec<Value>) -> String {
        let id = Uuid::new_v4().to_string();
        self.lists.insert(id.clone(), (name, val));
        id
    }

    pub fn add_costume(&mut self, name: String, path: String, project_root: &Path) {
        let source_path = project_root.join(&path);
        let content = fs::read(&source_path)
            .unwrap_or_else(|_| panic!("Failed to read asset: {:?}", source_path));
        let ext = source_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("svg");
        let md5 = format!("{:x}", md5::compute(&content));
        let filename = format!("{}.{}", md5, ext);

        self.costumes.push(Costume {
            asset_id: md5.clone(),
            name,
            bitmap_resolution: Some(1),
            md5ext: filename.clone(),
            data_format: ext.to_string(),
            rotation_center_x: 0.0,
            rotation_center_y: 0.0,
        });

        self.asset_instructions.push((source_path, filename));
    }

    pub fn add_sound(&mut self, name: String, path: String, project_root: &Path) {
        let source_path = project_root.join(&path);
        let content = fs::read(&source_path)
            .unwrap_or_else(|_| panic!("Failed to read asset: {:?}", source_path));
        let ext = source_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("wav");
        let md5 = format!("{:x}", md5::compute(&content));
        let filename = format!("{}.{}", md5, ext);

        self.sounds.push(Sound {
            asset_id: md5.clone(),
            name,
            md5ext: filename.clone(),
            data_format: ext.to_string(),
            rate: Some(44100),
            sample_count: Some(0),
        });

        self.asset_instructions.push((source_path, filename));
    }

    pub fn add_menu_block(&mut self, opcode: &str, field_name: &str, value: String) -> String {
        let id = Uuid::new_v4().to_string();
        let mut fields = HashMap::new();
        fields.insert(
            field_name.to_string(),
            Field::Generic(vec![json!(value), Value::Null]),
        );

        let block = NormalBlock {
            opcode: opcode.to_string(),
            next: None,
            parent: None, // Will be linked by parent
            inputs: HashMap::new(),
            fields,
            shadow: true,
            top_level: false,
            x: None,
            y: None,
            mutation: None,
        };
        self.blocks.insert(id.clone(), Block::Normal(block));
        id
    }
}

pub fn compile_target(
    program: &Program,
    is_stage: bool,
    global_variables: Option<&HashMap<String, (String, Value)>>,
    global_lists: Option<&HashMap<String, (String, Vec<Value>)>>,
    project_root: &Path,
) -> (Target, Vec<(PathBuf, String)>) {
    let mut ctx = CompilerContext::new(global_variables, global_lists);

    // Process Globals/Variables first
    for item in &program.items {
        if let Item::Variable(decl) = item {
            if !is_stage && decl.visibility == Visibility::Public {
                continue;
            }
            if let Type::List = decl.ty {
                let mut initial_values = Vec::new();
                if let Expr::List(exprs) = &decl.init {
                    for e in exprs {
                        match e {
                            Expr::Number(n) => initial_values.push(json!(n)),
                            Expr::String(s) => initial_values.push(json!(s)),
                            Expr::Bool(b) => initial_values.push(json!(b)),
                            _ => (),
                        }
                    }
                }
                ctx.add_list(decl.name.clone(), initial_values);
            } else {
                // TODO: evaluate init expr
                let val = json!(0); // Placeholder
                ctx.add_variable(decl.name.clone(), val);
            }
        } else if let Item::Costume(decl) = item {
            ctx.add_costume(decl.name.clone(), decl.path.clone(), project_root);
        } else if let Item::Sound(decl) = item {
            ctx.add_sound(decl.name.clone(), decl.path.clone(), project_root);
        } else if let Item::Procedure(proc) = item {
            let mut arg_ids = Vec::new();
            let mut arg_names = Vec::new();
            let mut proccode = proc.name.clone();

            for param in &proc.params {
                match param.ty {
                    Type::Boolean => proccode.push_str(" %b"),
                    Type::Number => proccode.push_str(" %n"),
                    _ => proccode.push_str(" %s"),
                }
                arg_ids.push(Uuid::new_v4().to_string());
                arg_names.push(param.name.clone());
            }

            ctx.procedures.insert(
                proc.name.clone(),
                ProcedureInfo {
                    proccode,
                    arg_ids,
                    arg_names,
                    warp: proc.is_warp,
                },
            );
        }
    }

    let mut last_stmt_id: Option<String> = None;

    // Process Functions, Procedures and Comments
    for item in &program.items {
        match item {
            Item::Comment(text) => {
                ctx.pending_comment = Some(text.clone());
            }
            Item::Stmt(stmt) => {
                let is_first = last_stmt_id.is_none();
                if let Some(id) = compile_stmt(stmt, last_stmt_id.clone(), &mut ctx) {
                    if is_first {
                        if let Some(Block::Normal(b)) = ctx.blocks.get_mut(&id) {
                            b.top_level = true;
                            b.x = Some(0.0);
                            b.y = Some(0.0);
                        }
                    }

                    if let Some(comment_text) = ctx.pending_comment.take() {
                        let comment_id = Uuid::new_v4().to_string();
                        ctx.comments.insert(
                            comment_id,
                            Comment {
                                block_id: Some(id.clone()),
                                x: 0.0,
                                y: 0.0,
                                width: 200.0,
                                height: 200.0,
                                minimized: false,
                                text: comment_text,
                            },
                        );
                    }
                    last_stmt_id = Some(id);
                }
            }
            Item::Function(func) => {
                last_stmt_id = None;
                if let Some(block_id) = compile_function(func, &mut ctx) {
                    if let Some(comment_text) = ctx.pending_comment.take() {
                        let comment_id = Uuid::new_v4().to_string();
                        ctx.comments.insert(
                            comment_id,
                            Comment {
                                block_id: Some(block_id),
                                x: 0.0,
                                y: 0.0,
                                width: 200.0,
                                height: 200.0,
                                minimized: false,
                                text: comment_text,
                            },
                        );
                    }
                }
            }
            Item::Procedure(proc) => {
                last_stmt_id = None;
                if let Some(block_id) = compile_procedure(proc, &mut ctx) {
                    if let Some(comment_text) = ctx.pending_comment.take() {
                        let comment_id = Uuid::new_v4().to_string();
                        ctx.comments.insert(
                            comment_id,
                            Comment {
                                block_id: Some(block_id),
                                x: 0.0,
                                y: 0.0,
                                width: 200.0,
                                height: 200.0,
                                minimized: false,
                                text: comment_text,
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }

    (
        Target {
            is_stage,
            name: if is_stage {
                "Stage".to_string()
            } else {
                "Sprite".to_string()
            },
            variables: ctx.variables,
            lists: ctx.lists,
            broadcasts: ctx.broadcast_map,
            blocks: ctx.blocks,
            comments: ctx.comments,
            current_costume: 0,
            costumes: ctx.costumes,
            sounds: ctx.sounds,
            volume: 100.0,
            layer_order: 0,
            tempo: if is_stage { Some(60) } else { None },
            video_transparency: if is_stage { Some(50) } else { None },
            video_state: if is_stage {
                Some("on".to_string())
            } else {
                None
            },
            text_to_speech_language: None,
            visible: if is_stage { None } else { Some(true) },
            x: if is_stage { None } else { Some(0.0) },
            y: if is_stage { None } else { Some(0.0) },
            size: if is_stage { None } else { Some(100.0) },
            direction: if is_stage { None } else { Some(90.0) },
            draggable: if is_stage { None } else { Some(false) },
            rotation_style: if is_stage {
                None
            } else {
                Some("all around".to_string())
            },
        },
        ctx.asset_instructions,
    )
}

fn compile_procedure(proc: &ProcedureDef, ctx: &mut CompilerContext) -> Option<String> {
    // Retrieve pre-calculated info
    let info = match ctx.procedures.get(&proc.name) {
        Some(i) => i.clone(),
        None => return None,
    };

    // Create Prototype Block (Shadow)
    let prototype_id = Uuid::new_v4().to_string();
    let mut inputs = HashMap::new();

    // Create argument reporters
    for (i, arg_name) in info.arg_names.iter().enumerate() {
        let arg_id = &info.arg_ids[i];

        // Check type for correct opcode
        let opcode = match proc.params[i].ty {
            Type::Boolean => "argument_reporter_boolean",
            _ => "argument_reporter_string_number",
        };

        let arg_block = NormalBlock {
            opcode: opcode.to_string(),
            next: None,
            parent: Some(prototype_id.clone()),
            inputs: HashMap::new(),
            fields: {
                let mut f = HashMap::new();
                f.insert(
                    "VALUE".to_string(),
                    Field::Generic(vec![json!(arg_name), Value::Null]),
                );
                f
            },
            shadow: true,
            top_level: false,
            x: None,
            y: None,
            mutation: None,
        };

        ctx.blocks.insert(arg_id.clone(), Block::Normal(arg_block));
        inputs.insert(
            arg_id.clone(),
            Input::Generic(vec![json!(1), json!(arg_id)]),
        );
    }

    let mutation = Mutation {
        tag_name: "mutation".to_string(),
        children: Some(vec![]),
        proccode: Some(info.proccode),
        argumentids: Some(serde_json::to_string(&info.arg_ids).unwrap()),
        argumentnames: Some(serde_json::to_string(&info.arg_names).unwrap()),
        argumentdefaults: Some(serde_json::to_string(&vec![""; info.arg_names.len()]).unwrap()),
        warp: Some(info.warp.to_string()),
    };

    let prototype_block = NormalBlock {
        opcode: "procedures_prototype".to_string(),
        next: None,
        parent: None,
        inputs,
        fields: HashMap::new(),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        mutation: Some(mutation),
    };
    ctx.blocks
        .insert(prototype_id.clone(), Block::Normal(prototype_block));

    // Create Definition Block (Hat)
    let definition_block = NormalBlock {
        opcode: "procedures_definition".to_string(),
        next: None,
        parent: None,
        inputs: {
            let mut i = HashMap::new();
            i.insert(
                "custom_block".to_string(),
                Input::Generic(vec![json!(1), json!(prototype_id)]),
            );
            i
        },
        fields: HashMap::new(),
        shadow: false,
        top_level: true,
        x: Some(0.0), // TODO: layout
        y: Some(0.0),
        mutation: None,
    };
    let def_id = ctx.add_block(definition_block);

    // Update parent of prototype
    if let Some(Block::Normal(b)) = ctx.blocks.get_mut(&prototype_id) {
        b.parent = Some(def_id.clone());
    }

    let mut prev_id = Some(def_id.clone());

    // Set current procedure args context
    let mut proc_args = HashMap::new();
    for param in &proc.params {
        proc_args.insert(param.name.clone(), param.ty.clone());
    }
    ctx.current_proc_args = Some(proc_args);

    // Compile body
    for stmt in &proc.body {
        prev_id = compile_stmt(stmt, prev_id, ctx);
    }

    // Clear context
    ctx.current_proc_args = None;

    Some(def_id)
}

fn compile_function(func: &Function, ctx: &mut CompilerContext) -> Option<String> {
    // Check for Hat attributes
    let hat_opcode = if let Some(attr) = func.attributes.first() {
        match attr.name.as_str() {
            "on_flag_clicked" => Some("event_whenflagclicked"),
            "on_key_pressed" => Some("event_whenkeypressed"),
            "on_clone_start" => Some("control_start_as_clone"),
            _ => None,
        }
    } else {
        None // Custom block definition (TODO)
    };

    if let Some(opcode) = hat_opcode {
        let mut prev_id;

        // Create Hat Block
        let mut hat_block = NormalBlock {
            opcode: opcode.to_string(),
            next: None,
            parent: None,
            inputs: HashMap::new(),
            fields: HashMap::new(),
            shadow: false,
            top_level: true,
            x: Some(0.0), // TODO: layout
            y: Some(0.0),
            mutation: None,
        };

        // Handle hat fields (e.g., key option)
        if opcode == "event_whenkeypressed" {
            if let Some(attr) = func.attributes.first() {
                if let Some(Expr::String(key)) = attr.args.first() {
                    hat_block.fields.insert(
                        "KEY_OPTION".to_string(),
                        Field::Generic(vec![json!(key), Value::Null]),
                    );
                }
            }
        }

        let hat_id = ctx.add_block(hat_block);
        prev_id = Some(hat_id.clone());

        // Compile body
        for stmt in &func.body {
            prev_id = compile_stmt(stmt, prev_id, ctx);
        }

        Some(hat_id)
    } else {
        None
    }
}

fn compile_stmt(
    stmt: &Stmt,
    parent_id: Option<String>,
    ctx: &mut CompilerContext,
) -> Option<String> {
    match stmt {
        Stmt::Expr(Expr::Call(name, args)) | Stmt::Expr(Expr::ProcCall(name, args)) => {
            let (opcode, inputs, fields, mutation) = map_call(name, args, ctx);

            // Identify shadow blocks before moving inputs
            let mut shadow_ids = Vec::new();
            for (_, input) in &inputs {
                // We know input is Input::Generic here based on map_call implementation
                // but for safety we keep the match, but we can suppress the warning or change logic
                // Since Input only has one variant, we can direct access or keep as is.
                // The warning says `if let` is irrefutable.
                let Input::Generic(vals) = input;
                if vals.len() >= 2 && vals[0] == json!(1) {
                    if let Some(shadow_id) = vals[1].as_str() {
                        shadow_ids.push(shadow_id.to_string());
                    }
                }
            }

            let block = NormalBlock {
                opcode,
                next: None,
                parent: parent_id.clone(),
                inputs,
                fields,
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation,
            };
            let id = ctx.add_block(block);

            // Fix up parent pointers for shadow blocks
            for shadow_id in shadow_ids {
                if let Some(Block::Normal(shadow_block)) = ctx.blocks.get_mut(&shadow_id) {
                    shadow_block.parent = Some(id.clone());
                }
            }

            // Link parent to this
            if let Some(pid) = parent_id {
                if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                    parent_block.next = Some(id.clone());
                }
            }

            Some(id)
        }
        Stmt::If(cond, then_block, else_block) => {
            let cond_input = compile_expr_input(cond, ctx);

            // Compile substacks
            let substack_id = compile_sequence(then_block, ctx);
            let substack2_id = if let Some(else_b) = else_block {
                compile_sequence(else_b, ctx)
            } else {
                None
            };

            let mut inputs = HashMap::new();
            inputs.insert("CONDITION".to_string(), cond_input);
            if let Some(sid) = substack_id {
                inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::Generic(vec![json!(2), json!(sid)]),
                );
            }
            if let Some(sid) = substack2_id {
                inputs.insert(
                    "SUBSTACK2".to_string(),
                    Input::Generic(vec![json!(2), json!(sid)]),
                );
            }

            let opcode = if else_block.is_some() {
                "control_if_else"
            } else {
                "control_if"
            };

            let block = NormalBlock {
                opcode: opcode.to_string(),
                next: None,
                parent: parent_id.clone(),
                inputs,
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation: None,
            };
            let id = ctx.add_block(block);

            if let Some(pid) = parent_id {
                if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                    parent_block.next = Some(id.clone());
                }
            }
            Some(id)
        }
        Stmt::Forever(body) => {
            let substack_id = compile_sequence(body, ctx);
            let mut inputs = HashMap::new();
            if let Some(sid) = substack_id {
                inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::Generic(vec![json!(2), json!(sid)]),
                );
            }
            let block = NormalBlock {
                opcode: "control_forever".to_string(),
                next: None,
                parent: parent_id.clone(),
                inputs,
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation: None,
            };
            let id = ctx.add_block(block);
            if let Some(pid) = parent_id {
                if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                    parent_block.next = Some(id.clone());
                }
            }
            Some(id)
        }
        Stmt::Repeat(count, body) => {
            let count_input = compile_expr_input(count, ctx);
            let substack_id = compile_sequence(body, ctx);
            let mut inputs = HashMap::new();
            inputs.insert("TIMES".to_string(), count_input);
            if let Some(sid) = substack_id {
                inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::Generic(vec![json!(2), json!(sid)]),
                );
            }
            let block = NormalBlock {
                opcode: "control_repeat".to_string(),
                next: None,
                parent: parent_id.clone(),
                inputs,
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation: None,
            };
            let id = ctx.add_block(block);
            if let Some(pid) = parent_id {
                if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                    parent_block.next = Some(id.clone());
                }
            }
            Some(id)
        }
        Stmt::Until(cond, body) => {
            let cond_input = compile_expr_input(cond, ctx);
            let substack_id = compile_sequence(body, ctx);
            let mut inputs = HashMap::new();
            inputs.insert("CONDITION".to_string(), cond_input);
            if let Some(sid) = substack_id {
                inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::Generic(vec![json!(2), json!(sid)]),
                );
            }
            let block = NormalBlock {
                opcode: "control_repeat_until".to_string(),
                next: None,
                parent: parent_id.clone(),
                inputs,
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation: None,
            };
            let id = ctx.add_block(block);
            if let Some(pid) = parent_id {
                if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                    parent_block.next = Some(id.clone());
                }
            }
            Some(id)
        }
        Stmt::Assign(name, val) => {
            // Find variable ID
            let var_id = ctx
                .variables
                .iter()
                .find(|(_, (vname, _))| vname == name)
                .map(|(id, _)| id.clone())
                .or_else(|| {
                    ctx.global_variables.and_then(|globals| {
                        globals
                            .iter()
                            .find(|(_, (vname, _))| vname == name)
                            .map(|(id, _)| id.clone())
                    })
                });

            if let Some(vid) = var_id {
                let val_input = compile_expr_input(val, ctx);
                let mut inputs = HashMap::new();
                inputs.insert("VALUE".to_string(), val_input);
                let mut fields = HashMap::new();
                fields.insert(
                    "VARIABLE".to_string(),
                    Field::Generic(vec![json!(name), json!(vid)]),
                );

                let block = NormalBlock {
                    opcode: "data_setvariableto".to_string(),
                    next: None,
                    parent: parent_id.clone(),
                    inputs,
                    fields,
                    shadow: false,
                    top_level: false,
                    x: None,
                    y: None,
                    mutation: None,
                };
                let id = ctx.add_block(block);
                if let Some(pid) = parent_id {
                    if let Some(Block::Normal(parent_block)) = ctx.blocks.get_mut(&pid) {
                        parent_block.next = Some(id.clone());
                    }
                }
                Some(id)
            } else {
                parent_id // Variable not found, ignore
            }
        }
        _ => parent_id, // Ignore others for MVP
    }
}

fn compile_sequence(stmts: &Vec<Stmt>, ctx: &mut CompilerContext) -> Option<String> {
    if stmts.is_empty() {
        return None;
    }
    let mut prev_id = None;
    let mut first_id = None;

    for stmt in stmts {
        let new_id = compile_stmt(stmt, prev_id.clone(), ctx);
        if first_id.is_none() && new_id.is_some() {
            first_id = new_id.clone();
        }
        if new_id.is_some() {
            prev_id = new_id;
        }
    }
    first_id
}

fn find_list_arg(expr: &Expr, ctx: &CompilerContext) -> Option<(String, String)> {
    let name = match expr {
        Expr::String(s) => s,
        Expr::Variable(s) => s,
        _ => return None,
    };

    if let Some((id, (real_name, _))) = ctx.lists.iter().find(|(_, (n, _))| n == name) {
        return Some((real_name.clone(), id.clone()));
    }
    if let Some(globals) = ctx.global_lists {
        if let Some((id, (real_name, _))) = globals.iter().find(|(_, (n, _))| n == name) {
            return Some((real_name.clone(), id.clone()));
        }
    }
    None
}

fn map_call(
    name: &str,
    args: &Vec<Expr>,
    ctx: &mut CompilerContext,
) -> (
    String,
    HashMap<String, Input>,
    HashMap<String, Field>,
    Option<Mutation>,
) {
    let mut inputs = HashMap::new();
    let mut fields = HashMap::new();
    let mut mutation = None;

    let opcode = match name {
        "add_to_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("ITEM".to_string(), compile_expr_input(&args[1], ctx));
            "data_addtolist"
        }
        "delete_of_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("INDEX".to_string(), compile_expr_input(&args[1], ctx));
            "data_deleteoflist"
        }
        "delete_all_of_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            "data_deletealloflist"
        }
        "insert_at_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("INDEX".to_string(), compile_expr_input(&args[1], ctx));
            inputs.insert("ITEM".to_string(), compile_expr_input(&args[2], ctx));
            "data_insertatlist"
        }
        "replace_item_of_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("INDEX".to_string(), compile_expr_input(&args[1], ctx));
            inputs.insert("ITEM".to_string(), compile_expr_input(&args[2], ctx));
            "data_replaceitemoflist"
        }
        "item_of_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("INDEX".to_string(), compile_expr_input(&args[1], ctx));
            "data_itemoflist"
        }
        "length_of_list" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            "data_lengthoflist"
        }
        "list_contains" => {
            if let Some((list_name, list_id)) = find_list_arg(&args[0], ctx) {
                fields.insert(
                    "LIST".to_string(),
                    Field::Generic(vec![json!(list_name), json!(list_id)]),
                );
            }
            inputs.insert("ITEM".to_string(), compile_expr_input(&args[1], ctx));
            "data_listcontainsitem"
        }
        "move_steps" => {
            inputs.insert("STEPS".to_string(), compile_expr_input(&args[0], ctx));
            "motion_movesteps"
        }
        "turn_right" => {
            inputs.insert("DEGREES".to_string(), compile_expr_input(&args[0], ctx));
            "motion_turnright"
        }
        "turn_left" => {
            inputs.insert("DEGREES".to_string(), compile_expr_input(&args[0], ctx));
            "motion_turnleft"
        }
        "go_to" => {
            inputs.insert("X".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("Y".to_string(), compile_expr_input(&args[1], ctx));
            "motion_gotoxy"
        }
        "glide" => {
            inputs.insert("SECS".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("X".to_string(), compile_expr_input(&args[1], ctx));
            inputs.insert("Y".to_string(), compile_expr_input(&args[2], ctx));
            "motion_glidesecstoxy"
        }
        "point_in_direction" => {
            inputs.insert("DIRECTION".to_string(), compile_expr_input(&args[0], ctx));
            "motion_pointindirection"
        }
        "change_x_by" => {
            inputs.insert("DX".to_string(), compile_expr_input(&args[0], ctx));
            "motion_changexby"
        }
        "set_x_to" => {
            inputs.insert("X".to_string(), compile_expr_input(&args[0], ctx));
            "motion_setx"
        }
        "change_y_by" => {
            inputs.insert("DY".to_string(), compile_expr_input(&args[0], ctx));
            "motion_changeyby"
        }
        "set_y_to" => {
            inputs.insert("Y".to_string(), compile_expr_input(&args[0], ctx));
            "motion_sety"
        }
        "if_on_edge_bounce" => "motion_ifonedgebounce",
        "set_rotation_style" => {
             if let Expr::String(style) = &args[0] {
                fields.insert(
                    "STYLE".to_string(),
                    Field::Generic(vec![json!(style), Value::Null]),
                );
            }
            "motion_setrotationstyle"
        }
        "say" => {
            inputs.insert("MESSAGE".to_string(), compile_expr_input(&args[0], ctx));
            "looks_say"
        }
        "say_for" => {
            inputs.insert("MESSAGE".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("SECS".to_string(), compile_expr_input(&args[1], ctx));
            "looks_sayforsecs"
        }
        "think" => {
            inputs.insert("MESSAGE".to_string(), compile_expr_input(&args[0], ctx));
            "looks_think"
        }
        "think_for" => {
            inputs.insert("MESSAGE".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("SECS".to_string(), compile_expr_input(&args[1], ctx));
            "looks_thinkforsecs"
        }
        "switch_costume_to" => {
            inputs.insert("COSTUME".to_string(), compile_expr_input(&args[0], ctx));
            "looks_switchcostumeto"
        }
        "next_costume" => "looks_nextcostume",
        "switch_backdrop_to" => {
            inputs.insert("BACKDROP".to_string(), compile_expr_input(&args[0], ctx));
            "looks_switchbackdropto"
        }
        "next_backdrop" => "looks_nextbackdrop",
        "change_size_by" => {
            inputs.insert("CHANGE".to_string(), compile_expr_input(&args[0], ctx));
            "looks_changesizeby"
        }
        "set_size_to" => {
            inputs.insert("SIZE".to_string(), compile_expr_input(&args[0], ctx));
            "looks_setsizeto"
        }
        "change_effect_by" => {
            // args[0] is effect name, args[1] is value
             if let Expr::String(effect) = &args[0] {
                fields.insert(
                    "EFFECT".to_string(),
                    Field::Generic(vec![json!(effect), Value::Null]),
                );
            }
            inputs.insert("CHANGE".to_string(), compile_expr_input(&args[1], ctx));
            "looks_changeeffectby"
        }
        "set_effect_to" => {
            // args[0] is effect name, args[1] is value
             if let Expr::String(effect) = &args[0] {
                fields.insert(
                    "EFFECT".to_string(),
                    Field::Generic(vec![json!(effect), Value::Null]),
                );
            }
            inputs.insert("VALUE".to_string(), compile_expr_input(&args[1], ctx));
            "looks_seteffectto"
        }
        "clear_graphic_effects" => "looks_cleargraphiceffects",
        "show" => "looks_show",
        "hide" => "looks_hide",
        "go_to_front_layer" => {
            fields.insert("FRONT_BACK".to_string(), Field::Generic(vec![json!("front"), Value::Null]));
            "looks_gotofrontback"
        }
        "go_back_layer" => {
            fields.insert("FRONT_BACK".to_string(), Field::Generic(vec![json!("back"), Value::Null]));
            "looks_gotofrontback"
        }
        "go_forward_layers" => {
            fields.insert("FORWARD_BACKWARD".to_string(), Field::Generic(vec![json!("forward"), Value::Null]));
            inputs.insert("NUM".to_string(), compile_expr_input(&args[0], ctx));
            "looks_goforwardbackwardlayers"
        }
        "go_backward_layers" => {
            fields.insert("FORWARD_BACKWARD".to_string(), Field::Generic(vec![json!("backward"), Value::Null]));
            inputs.insert("NUM".to_string(), compile_expr_input(&args[0], ctx));
            "looks_goforwardbackwardlayers"
        }
        "size" => "looks_size",
        "costume_number" => {
            fields.insert("NUMBER_NAME".to_string(), Field::Generic(vec![json!("number"), Value::Null]));
            "looks_costumenumbername"
        }
        "costume_name" => {
            fields.insert("NUMBER_NAME".to_string(), Field::Generic(vec![json!("name"), Value::Null]));
            "looks_costumenumbername"
        }
        "backdrop_number" => {
            fields.insert("NUMBER_NAME".to_string(), Field::Generic(vec![json!("number"), Value::Null]));
            "looks_backdropnumbername"
        }
        "backdrop_name" => {
            fields.insert("NUMBER_NAME".to_string(), Field::Generic(vec![json!("name"), Value::Null]));
            "looks_backdropnumbername"
        }
        "start_sound" => {
            inputs.insert("SOUND_MENU".to_string(), compile_expr_input(&args[0], ctx));
            "sound_play"
        }
        "play_sound_until_done" => {
            inputs.insert("SOUND_MENU".to_string(), compile_expr_input(&args[0], ctx));
            "sound_playuntildone"
        }
        "stop_all_sounds" => "sound_stopallsounds",
        "change_volume_by" => {
            inputs.insert("VOLUME".to_string(), compile_expr_input(&args[0], ctx));
            "sound_changevolumeby"
        }
        "set_volume_to" => {
            inputs.insert("VOLUME".to_string(), compile_expr_input(&args[0], ctx));
            "sound_setvolumeto"
        }
        "volume" => "sound_volume",
        "broadcast" => {
            inputs.insert("BROADCAST_INPUT".to_string(), compile_expr_input(&args[0], ctx));
            "event_broadcast"
        }
        "broadcast_and_wait" => {
            inputs.insert("BROADCAST_INPUT".to_string(), compile_expr_input(&args[0], ctx));
            "event_broadcastandwait"
        }
        "create_clone_of" => {
            if let Expr::String(val) = &args[0] {
                let menu_val = if val == "myself" { "_myself_" } else { val };
                let menu_id = ctx.add_menu_block(
                    "control_create_clone_of_menu",
                    "CLONE_OPTION",
                    menu_val.to_string(),
                );
                inputs.insert(
                    "CLONE_OPTION".to_string(),
                    Input::Generic(vec![json!(1), json!(menu_id)]),
                );
            } else {
                inputs.insert(
                    "CLONE_OPTION".to_string(),
                    compile_expr_input(&args[0], ctx),
                );
            }
            "control_create_clone_of"
        }
        "delete_this_clone" => "control_delete_this_clone",
        "wait" => {
            inputs.insert("DURATION".to_string(), compile_expr_input(&args[0], ctx));
            "control_wait"
        }
        "wait_until" => {
             inputs.insert("CONDITION".to_string(), compile_expr_input(&args[0], ctx));
             "control_wait_until"
        }
        "stop" => {
             if let Expr::String(opt) = &args[0] {
                fields.insert(
                    "STOP_OPTION".to_string(),
                    Field::Generic(vec![json!(opt), Value::Null]),
                );
            }
            "control_stop"
        }
        "touching" => {
            inputs.insert(
                "TOUCHINGOBJECTMENU".to_string(),
                compile_expr_input(&args[0], ctx),
            );
            "sensing_touchingobject"
        }
        "touching_color" => {
            inputs.insert("COLOR".to_string(), compile_expr_input(&args[0], ctx));
            "sensing_touchingcolor"
        }
        "color_touching_color" => {
            inputs.insert("COLOR".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("COLOR2".to_string(), compile_expr_input(&args[1], ctx));
            "sensing_coloristouchingcolor"
        }
        "distance_to" => {
             inputs.insert("DISTANCETOMENU".to_string(), compile_expr_input(&args[0], ctx));
             "sensing_distanceto"
        }
        "ask_and_wait" => {
             inputs.insert("QUESTION".to_string(), compile_expr_input(&args[0], ctx));
             "sensing_askandwait"
        }
        "answer" => "sensing_answer",
        "key_pressed" => {
            inputs.insert("KEY_OPTION".to_string(), compile_expr_input(&args[0], ctx));
            "sensing_keypressed"
        }
        "mouse_down" => "sensing_mousedown",
        "mouse_x" => "sensing_mousex",
        "mouse_y" => "sensing_mousey",
        "set_drag_mode" => {
             if let Expr::String(mode) = &args[0] {
                fields.insert(
                    "DRAG_MODE".to_string(),
                    Field::Generic(vec![json!(mode), Value::Null]),
                );
            }
            "sensing_setdragmode"
        }
        "loudness" => "sensing_loudness",
        "timer" => "sensing_timer",
        "reset_timer" => "sensing_resettimer",
        "of" => {
             // property, object
             if let Expr::String(prop) = &args[0] {
                fields.insert(
                    "PROPERTY".to_string(),
                    Field::Generic(vec![json!(prop), Value::Null]),
                );
             }
             inputs.insert("OBJECT".to_string(), compile_expr_input(&args[1], ctx));
             "sensing_of"
        }
        "current_year" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("YEAR"), Value::Null]));
            "sensing_current"
        }
        "current_month" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("MONTH"), Value::Null]));
            "sensing_current"
        }
        "current_date" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("DATE"), Value::Null]));
            "sensing_current"
        }
        "current_day_of_week" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("DAYOFWEEK"), Value::Null]));
            "sensing_current"
        }
        "current_hour" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("HOUR"), Value::Null]));
            "sensing_current"
        }
        "current_minute" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("MINUTE"), Value::Null]));
            "sensing_current"
        }
        "current_second" => {
            fields.insert("CURRENTMENU".to_string(), Field::Generic(vec![json!("SECOND"), Value::Null]));
            "sensing_current"
        }
        "days_since_2000" => "sensing_dayssince2000",
        "username" => "sensing_username",
        "random" => {
            inputs.insert("FROM".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("TO".to_string(), compile_expr_input(&args[1], ctx));
            "operator_random"
        }
        "join" => {
            inputs.insert("STRING1".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("STRING2".to_string(), compile_expr_input(&args[1], ctx));
            "operator_join"
        }
        "letter_of" => {
            inputs.insert("STRING".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("LETTER".to_string(), compile_expr_input(&args[1], ctx));
            "operator_letter_of"
        }
        "length_of" => {
            inputs.insert("STRING".to_string(), compile_expr_input(&args[0], ctx));
            "operator_length"
        }
        "contains" => {
            inputs.insert("STRING1".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("STRING2".to_string(), compile_expr_input(&args[1], ctx));
            "operator_contains"
        }
        "mod" => {
            inputs.insert("NUM1".to_string(), compile_expr_input(&args[0], ctx));
            inputs.insert("NUM2".to_string(), compile_expr_input(&args[1], ctx));
            "operator_mod"
        }
        "round" => {
            inputs.insert("NUM".to_string(), compile_expr_input(&args[0], ctx));
            "operator_round"
        }
        "abs" | "floor" | "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "ln" | "log" | "e^" | "10^" => {
             fields.insert(
                "OPERATOR".to_string(),
                Field::Generic(vec![json!(name), Value::Null]),
            );
            inputs.insert("NUM".to_string(), compile_expr_input(&args[0], ctx));
            "operator_mathop"
        }
        "ceil" => {
             fields.insert(
                "OPERATOR".to_string(),
                Field::Generic(vec![json!("ceiling"), Value::Null]),
            );
            inputs.insert("NUM".to_string(), compile_expr_input(&args[0], ctx));
            "operator_mathop"
        }
        "y_position" => "motion_yposition",
        _ => {
            if let Some(info) = ctx.procedures.get(name).cloned() {
                for (i, arg) in args.iter().enumerate() {
                    if i < info.arg_ids.len() {
                        inputs.insert(info.arg_ids[i].clone(), compile_expr_input(arg, ctx));
                    }
                }

                mutation = Some(Mutation {
                    tag_name: "mutation".to_string(),
                    children: Some(vec![]),
                    proccode: Some(info.proccode),
                    argumentids: Some(serde_json::to_string(&info.arg_ids).unwrap()),
                    argumentnames: None,
                    argumentdefaults: None,
                    warp: Some(info.warp.to_string()),
                });

                "procedures_call"
            } else {
                "control_wait" // Default fallback
            }
        }
    };

    (opcode.to_string(), inputs, fields, mutation)
}

fn compile_expr_input(expr: &Expr, ctx: &mut CompilerContext) -> Input {
    match expr {
        Expr::Number(n) => Input::Generic(vec![json!(1), json!([4, n])]), // 4 is Number primitive
        Expr::String(s) => {
            // Handle special menu inputs if needed, but for now generic string
            Input::Generic(vec![json!(1), json!([10, s])]) // 10 is String primitive
        }
        Expr::Bool(b) => Input::Generic(vec![json!(1), json!([4, if *b { 1 } else { 0 }])]), // Scratch bools are tricky
        Expr::Variable(name) => {
            // Find variable ID
            let var_id = ctx
                .variables
                .iter()
                .find(|(_, (vname, _))| vname == name)
                .map(|(id, _)| id.clone())
                .or_else(|| {
                    ctx.global_variables.and_then(|globals| {
                        globals
                            .iter()
                            .find(|(_, (vname, _))| vname == name)
                            .map(|(id, _)| id.clone())
                    })
                });

            if let Some(vid) = var_id {
                // [12, Name, ID] - 12 is Variable primitive
                Input::Generic(vec![json!(3), json!([12, name, vid]), json!([10, ""])])
            } else {
                // Check if it is a procedure argument
                if let Some(proc_args) = &ctx.current_proc_args {
                    if let Some(ty) = proc_args.get(name) {
                        let opcode = match ty {
                            Type::Boolean => "argument_reporter_boolean",
                            _ => "argument_reporter_string_number",
                        };

                        let mut fields = HashMap::new();
                        fields.insert(
                            "VALUE".to_string(),
                            Field::Generic(vec![json!(name), Value::Null]),
                        );

                        let block = NormalBlock {
                            opcode: opcode.to_string(),
                            next: None,
                            parent: None,
                            inputs: HashMap::new(),
                            fields,
                            shadow: false,
                            top_level: false,
                            x: None,
                            y: None,
                            mutation: None,
                        };
                        let id = ctx.add_block(block);
                        return Input::Generic(vec![json!(3), json!(id), json!([10, ""])]);
                    }
                }

                Input::Generic(vec![json!(1), json!([10, ""])]) // Default empty string
            }
        }
        Expr::Call(name, args) | Expr::ProcCall(name, args) => {
            // Compile reporter block
            let (opcode, inputs, fields, mutation) = map_call(name, args, ctx);
            let block = NormalBlock {
                opcode,
                next: None,
                parent: None,
                inputs,
                fields,
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation,
            };
            let id = ctx.add_block(block);
            Input::Generic(vec![json!(3), json!(id), json!([10, ""])]) // 3 is Block input
        }
        Expr::BinOp(left, op, right) => {
            let lhs = compile_expr_input(left, ctx);
            let rhs = compile_expr_input(right, ctx);
            let mut inputs = HashMap::new();

            let (opcode, negate) = match op {
                Op::Add => ("operator_add", false),
                Op::Sub => ("operator_subtract", false),
                Op::Mul => ("operator_multiply", false),
                Op::Div => ("operator_divide", false),
                Op::Mod => ("operator_mod", false),
                Op::Lt => ("operator_lt", false),
                Op::Gt => ("operator_gt", false),
                Op::Eq => ("operator_equals", false),
                Op::And => ("operator_and", false),
                Op::Or => ("operator_or", false),
                Op::Ne => ("operator_equals", true),
                Op::Ge => ("operator_lt", true), // >= is not <
                Op::Le => ("operator_gt", true), // <= is not >
            };

            if opcode == "operator_add"
                || opcode == "operator_subtract"
                || opcode == "operator_multiply"
                || opcode == "operator_divide"
                || opcode == "operator_mod"
            {
                inputs.insert("NUM1".to_string(), lhs);
                inputs.insert("NUM2".to_string(), rhs);
            } else {
                inputs.insert("OPERAND1".to_string(), lhs);
                inputs.insert("OPERAND2".to_string(), rhs);
            }

            let block = NormalBlock {
                opcode: opcode.to_string(),
                next: None,
                parent: None,
                inputs,
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                x: None,
                y: None,
                mutation: None,
            };
            let mut id = ctx.add_block(block);

            if negate {
                let mut inputs = HashMap::new();
                inputs.insert(
                    "OPERAND".to_string(),
                    Input::Generic(vec![json!(3), json!(id), json!([10, ""])]),
                );
                let block = NormalBlock {
                    opcode: "operator_not".to_string(),
                    next: None,
                    parent: None,
                    inputs,
                    fields: HashMap::new(),
                    shadow: false,
                    top_level: false,
                    x: None,
                    y: None,
                    mutation: None,
                };
                id = ctx.add_block(block);
            }

            Input::Generic(vec![json!(3), json!(id), json!([10, ""])])
        }
        Expr::List(_) => Input::Generic(vec![json!(1), json!([10, ""])]), // Lists not supported as inputs
    }
}
