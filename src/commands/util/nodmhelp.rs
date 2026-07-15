use crate::command::{Command, CommandInfo, CommandRegistration};
use serenity::all::{Context, Message, CreateMessage, CreateEmbed, CreateEmbedAuthor, Colour};
use async_trait::async_trait;

pub struct NODMHELP;

#[async_trait]
impl Command for NODMHELP {
    fn info(&self) -> CommandInfo {
        CommandInfo {
            name: "nodmhelp",
            description: "Muestra la lista de comandos directamente en el canal",
            category: "Util",
        }
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: Vec<String>) -> serenity::Result<()> {
        let embed = CreateEmbed::new()
            .colour(Colour::from_rgb(0xAF, 0xBE, 0xFC))
            .author(CreateEmbedAuthor::new("Comandos de Chocolat"))
            .description("Holii~, me llamo Chocolat n.n, y esta es mi lista de comandos~. Si necesitas ayuda detallada con algún comando, [visita este enlace](https://chocolatbot.weebly.com/comandos.html) para ver todos los comandos detalladamente. ¡También únete a nuestro servidor de discord! (https://discord.gg/TKTGm69)")
            .field("Comandos Informativos", "`help` `botinfo` `about` `invite` `donate` `support` `suggestion` `bugreport` `vote` `infonodm` `nodmhelp`", false)
            .field("Comandos Útiles", "`ping` `avatar` `userinfo` `serverinfo` `servers` `channellist` `rolelist` `randomuser` `calc` `weather` `botinvite` `password` `translate` `pavatar` `shortlink` `hexcolor` `jumbo` `morse`", false)
            .field("Comandos Divertidos", "`say` `esay` `rannum` `8ball` `coinflip` `love` `ranime` `rate` `choose` `catrandom` `rps` `nekogirl` `machievement` `roll` `ascii` `bigtext` `note` `caguai` `reverse` `horoscope` `raw` `loli` `waifu` `husbando` `triggered` `trash` `trump` `meme` `achievement`", false)
            .field("Comandos de Juegos", "`osu` `fortnite` `pokedex`", false)
            .field("Comandos de Interacción", "`hug` `kiss` `kill` `pat` `poke` `slap` `cuddle` `highfive` `punch` `lick` `feed` `tickle` `bite` `handholding` `baka`", false)
            .field("Comandos de Reacción", "`cry` `happy` `confused` `blush` `dance` `like` `boom` `suicide` `sleep` `facepalm` `lewd` `dab` `angry` `banghead` `jpose` `nope` `pout` `shrug` `smug` `bored` `tsundere` `disgust`", false)
            .field("Comandos de Búsqueda", "`google` `youtube` `giphy` `anime` `manga` `gimages` `wiki` `lyrics`", false)
            .field("Comandos de Música", "ACTUALMENTE DESHABILITADOS", false)
            .field("Comandos NSFW", "`fuck` `r34` `rahentai` `hnekogirl` `konachan` `gmasturbate` `suck` `urban`", false)
            .field("Comandos ADMIN/MOD", "`ban` `kick` `unban` `mute` `unmute` `purge` `addrole` `remrole` `crole` `sdm` `nickname` `warn` `unwarn` `softban` `question` `tempmute` `hackban` `lastmessage`", false)
            .field("Comandos de Configuración", "`config` `setwelcome` `setleave` `setchannel` `setautorole` `setprefix` `setsuggestions` `setreports` `suggest` `approve` `deny` `maybe` `reportuser`", false);

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
        Ok(())
    }
}

inventory::submit! { CommandRegistration { command: &NODMHELP } }
