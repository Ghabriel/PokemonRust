mod events;
mod npc;
mod polymorphic_container;

use amethyst::{
    ecs::World,
    utils::application_root_dir,
};

use crate::entities::map::{CoordinateSystem, LuaGameScriptParameters};

use rlua::{Context, Error as LuaError, Function, Lua};

use self::{
    events::{
        create_chained_event,
        create_text_event,
        create_warp_event,
        add_event,
        dispatch_event,
    },
    npc::{
        create_npc,
        change_npc_direction,
        add_npc,
    },
    polymorphic_container::PolymorphicContainer,
};

use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    fs::read_to_string,
    io::Error as IoError,
    ops::{Deref, DerefMut},
};

struct ExecutionContext<'a> {
    lua_variables: PolymorphicContainer,
    world: &'a mut World,
}

impl Deref for ExecutionContext<'_> {
    type Target = PolymorphicContainer;

    fn deref(&self) -> &PolymorphicContainer {
        &self.lua_variables
    }
}

impl DerefMut for ExecutionContext<'_> {
    fn deref_mut(&mut self) -> &mut PolymorphicContainer {
        &mut self.lua_variables
    }
}

thread_local! {
    static LUA: Lua = Lua::new();
}

#[derive(Debug)]
pub enum LuaScriptError {
    Io(IoError),
    Lua(LuaError),
}

impl From<IoError> for LuaScriptError {
    fn from(error: IoError) -> LuaScriptError {
        LuaScriptError::Io(error)
    }
}

impl From<LuaError> for LuaScriptError {
    fn from(error: LuaError) -> LuaScriptError {
        LuaScriptError::Lua(error)
    }
}

impl Display for LuaScriptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LuaScriptError::Io(error) => write!(f, "{}", error),
            LuaScriptError::Lua(error) => write!(f, "{}", error),
        }
    }
}

pub fn run_lua_script(
    world: &mut World,
    file: &str,
    function: &str,
    parameters: &Option<LuaGameScriptParameters>,
) -> Result<(), LuaScriptError> {
    LUA.with(|lua| {
        run_script(world, &lua, &file, &function, &parameters)
    })
}

fn run_script(
    world: &mut World,
    lua: &Lua,
    file: &str,
    function: &str,
    parameters: &Option<LuaGameScriptParameters>,
) -> Result<(), LuaScriptError> {
    run_with_native_functions(world, lua, |context| {
        let path = application_root_dir()
            .unwrap()
            .join(&file);

        let content = read_to_string(&path)?;

        context.load(&content).exec()?;

        let function: Function = context.globals().get(function)?;

        match parameters {
            None => function.call(())?,
            Some(LuaGameScriptParameters::SourceTile(coordinates)) =>
                function.call((coordinates.x(), coordinates.y()))?,
            Some(LuaGameScriptParameters::TargetNpc(npc_id)) =>
                function.call(*npc_id)?,
        }

        Ok(())
    })
}

macro_rules! native_functions {
    (
        ($globals:ident, $scope:ident, $context:ident)
        $($target_name:ident: $original_name:ident($($param_name:ident: $param_type:ty),*)),*
    ) => {
        $(
            #[allow(unused_parens)]
            let $target_name = $scope.create_function_mut(|_, ($( $param_name ),*): ($( $param_type ),*)| {
                Ok(
                    $original_name(&mut $context.borrow_mut(), $( $param_name ),*)
                )
            })?;

            $globals.set(stringify!($target_name), $target_name)?;
        )*
    }
}

fn run_with_native_functions<F, R>(world: &mut World, lua: &Lua, callback: F) -> Result<R, LuaScriptError>
where
    F: FnOnce(&Context) -> Result<R, LuaScriptError>,
{
    let execution_context = RefCell::new(ExecutionContext {
        world,
        lua_variables: PolymorphicContainer::default(),
    });

    lua.context(|context| {
        context.scope(|scope| {
            let globals = context.globals();

            native_functions!(
                (globals, scope, execution_context)
                // Event functions
                rust_create_chained_event: create_chained_event(),
                rust_create_text_event: create_text_event(text: String),
                rust_create_warp_event: create_warp_event(map: String, x: u32, y: u32),
                rust_add_event: add_event(chain_key: usize, new_event: usize),
                rust_dispatch_event: dispatch_event(key: usize),
                // NPC functions
                rust_create_npc: create_npc(map_id: String, x: u32, y: u32, kind: String, direction: u8),
                rust_change_npc_direction: change_npc_direction(npc_key: usize, direction: u8),
                rust_add_npc: add_npc(npc_key: usize)
            );

            callback(&context)
        })
    })
}
