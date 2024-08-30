use uuid::Uuid;
use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct UuidExtension;

impl zed::Extension for UuidExtension {
    fn new() -> Self {
        UuidExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "uuidv4" => {
                let uuid = Uuid::new_v4().to_string();
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..uuid.len()).into(),
                        label: "UUID v4".to_string(),
                    }],
                    text: uuid,
                })
            }
            _ => Err("Unknown command".to_string()),
        }
    }
}

zed::register_extension!(UuidExtension);
