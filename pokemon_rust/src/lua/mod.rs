mod events;
mod npc;
mod polymorphic_container;

use amethyst::{
    ecs::World,
    utils::application_root_dir,
};

use crate::{
    common::{AssetTracker, Direction},
    entities::character::CharacterId,
    map::{CoordinateSystem, GameScriptParameters},
};

use rlua::{Context, Error as LuaError, FromLua, Function, Lua, Result as LuaResult, Value};

use self::{
    events::{
        add_event,
        create_bgm_change_event,
        create_chained_event,
        create_cyclic_event,
        create_npc_move_event,
        create_npc_rotate_event,
        create_npc_rotate_towards_player_event,
        create_text_event,
        create_warp_event,
        dispatch_event,
        preload_bgm,
    },
    npc::{
        add_npc,
        change_npc_direction,
        create_npc,
        rotate_npc_towards_player,
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

impl<'lua> FromLua<'lua> for CharacterId {
    fn from_lua(lua_value: Value<'lua>, context: Context<'lua>) -> LuaResult<Self> {
        Ok(CharacterId(usize::from_lua(lua_value, context)?))
    }
}

impl<'lua> FromLua<'lua> for Direction {
    fn from_lua(lua_value: Value<'lua>, context: Context<'lua>) -> LuaResult<Self> {
        let lua_type_name = get_lua_type_name(&lua_value);
        let direction = match context.coerce_integer(lua_value)? {
            Some(0) => Direction::Up,
            Some(1) => Direction::Down,
            Some(2) => Direction::Left,
            Some(3) => Direction::Right,
            _ => return Err(LuaError::FromLuaConversionError {
                from: lua_type_name,
                to: "Direction",
                message: Some("expected a value in the range 1..=4".to_string()),
            }),
        };

        Ok(direction)
    }
}

/// Returns a string represention of a Lua type. This is a copy of
/// `Value::type_name()`, which for some reason is private...
fn get_lua_type_name(value: &Value) -> &'static str {
    match value {
        Value::Nil => "nil",
        Value::Boolean(_) => "boolean",
        Value::LightUserData(_) => "light userdata",
        Value::Integer(_) => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Table(_) => "table",
        Value::Function(_) => "function",
        Value::Thread(_) => "thread",
        Value::UserData(_) | Value::Error(_) => "userdata",
    }
}

struct ExecutionContext<'a> {
    asset_tracker: AssetTracker,
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
    parameters: &Option<GameScriptParameters>,
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
    parameters: &Option<GameScriptParameters>,
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
            Some(GameScriptParameters::SourceTile(coordinates)) =>
                function.call((coordinates.x(), coordinates.y()))?,
            Some(GameScriptParameters::TargetCharacter(character_id)) =>
                function.call(character_id.0)?,
            Some(GameScriptParameters::SourceMap(map_name)) =>
                function.call(map_name.clone())?,
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
            let $target_name = $scope.create_function_mut(
                |_, ($( $param_name ),*): ($( $param_type ),*)| {
                    Ok(
                        $original_name(&mut $context.borrow_mut(), $( $param_name ),*)
                    )
                }
            )?;

            $globals.set(stringify!($target_name), $target_name)?;
        )*
    }
}

fn run_with_native_functions<F, R>(world: &mut World, lua: &Lua, callback: F) -> Result<R, LuaScriptError>
where
    F: FnOnce(&Context) -> Result<R, LuaScriptError>,
{
    let asset_tracker = world.remove::<AssetTracker>().unwrap();

    let execution_context = RefCell::new(ExecutionContext {
        world,
        asset_tracker,
        lua_variables: PolymorphicContainer::default(),
    });

    let result = lua.context(|context| {
        context.scope(|scope| {
            let globals = context.globals();

            native_functions!(
                (globals, scope, execution_context)
                // Event functions
                rust_create_bgm_change_event: create_bgm_change_event(filename: String),
                rust_preload_bgm: preload_bgm(filename: String),
                rust_create_chained_event: create_chained_event(),
                rust_create_cyclic_event: create_cyclic_event(event_key: usize),
                rust_create_npc_move_event:
                    create_npc_move_event(character_id: CharacterId, num_tiles: usize),
                rust_create_npc_rotate_event:
                    create_npc_rotate_event(character_id: CharacterId, direction: Direction),
                rust_create_npc_rotate_towards_player_event:
                    create_npc_rotate_towards_player_event(character_id: CharacterId),
                rust_create_text_event: create_text_event(text: String),
                rust_create_warp_event: create_warp_event(map: String, x: u32, y: u32),
                rust_add_event: add_event(chain_key: usize, new_event: usize),
                rust_dispatch_event: dispatch_event(key: usize),
                // NPC functions
                rust_create_npc:
                    create_npc(map_id: String, x: u32, y: u32, kind: String, direction: Direction),
                rust_change_npc_direction: change_npc_direction(npc_key: usize, direction: Direction),
                rust_rotate_npc_towards_player: rotate_npc_towards_player(character_id: CharacterId),
                rust_add_npc: add_npc(npc_key: usize)
            );

            callback(&context)
        })
    });

    let asset_tracker = execution_context.into_inner().asset_tracker;
    world.insert(asset_tracker);

    result
}
