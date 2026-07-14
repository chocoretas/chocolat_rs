use crate::command::*;
use serenity::all::{CreateMessage, CreateEmbed};

pub struct BugReport;

#[async_trait::async_trait]
impl Command for BugReport {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "bugreport",
            description: "Informa un error/bug que hayas encontrado",
            category: "Información",
        }
    }

    async fn execute(
        &self,
        ctx: &serenity::all::Context,
        msg: &serenity::all::Message,
        _args: Vec<String>,
    ) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .title("🐛 Reportar un Bug")
            .description("Si encontraste algún error en Chocolat, por favor infórmalo abriendo un 'Issue' en nuestro repositorio de GitHub o contactando a los creadores en el servidor de soporte.\n\n🔗 **[GitHub Issues](https://github.com/chocoretas/chocolat_rs/issues)**")
            
            .color(0xFF0000);

        msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(embed)
        ).await?;

        Ok(())
    }
}

inventory::submit! {
    CommandRegistration {
        command: &BugReport
    }
}
