use mlua::prelude::*;
use clap::Parser;
use serde::{Serialize, Deserialize};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Lua file to execute
    #[arg(short, long)]
    file: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Timing {
    start: f64,
    end_time: f64,
    steps: Vec<Step>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct Step {
    name: String,
    #[serde(default)]
    start: f64,
    #[serde(default)]

    end_time: f64,
}

fn main() -> LuaResult<()> {
    let args = Args::parse();
    let lua = Lua::new();
    lua.globals().set("null", lua.null())?;
    lua.globals().set("array_mt", lua.array_metatable())?;
    let timing = Timing::default();
    let timing = lua.to_value(&timing)?;
    lua.globals().set("timing", timing)?;

    // Load and execute the Lua file
    let result = lua.load(&std::fs::read_to_string(args.file).expect("Could not read file"))
        .exec();

    println!("Result: {:#?}", result);
    let timing: LuaValue = lua.globals().get("timing")?;
    let timing: Timing = lua.from_value(timing)?;
    println!("Timing: {:#?}", timing);
    
    Ok(())
}
