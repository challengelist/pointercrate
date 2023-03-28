use super::Page;
use crate::{model::user::User, permissions::Permissions, state::PointercrateState, ViewResult};
use actix_web::HttpResponse;
use actix_web_codegen::get;
use maud::{html, Markup, PreEscaped};

#[derive(Debug)]
struct Homepage {
    demonlist_team: Vec<User>,
    demonlist_subteam: Vec<User>,
    pointercrate_team: Vec<User>,
    pointercrate_subteam: Vec<User>,
}

#[get("/")]
pub async fn index(state: PointercrateState) -> ViewResult<HttpResponse> {
    let mut connection = state.connection().await?;

    let mut demonlist_team = User::by_permission(Permissions::ListAdministrator, &mut connection).await?;
    let demonlist_subteam = User::by_permission(Permissions::ListModerator, &mut connection).await?;
    let mut pointercrate_team = User::by_permission(Permissions::Administrator, &mut connection).await?;
    let pointercrate_subteam = User::by_permission(Permissions::Moderator, &mut connection).await?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        Homepage {
            demonlist_team,
            demonlist_subteam,
            pointercrate_team,
            pointercrate_subteam
        }
        .render()
        .0,
    ))
}

impl Page for Homepage {
    fn title(&self) -> String {
        "Challenge List - Homepage".to_owned()
    }

    fn description(&self) -> String {
        "The Challenge List is a coordinated list of the hardest challenges, maintained and ran by the game's most skilled challenge players."
            .to_owned()
    }

    fn scripts(&self) -> Vec<&str> {
        vec!["js/home.js", "js/modules/tab.mjs"]
    }

    fn stylesheets(&self) -> Vec<&str> {
        vec!["css/home.css"]
    }

    fn body(&self) -> Markup {
        html! {
            div.information-banner.left {
                div.tab-display#information-tabs {
                    div.information {
                        div style = "display: flex; flex-flow: column;"{
                            h1 style="text-align: left; margin-top: 0px" {
                                "The Challenge List"
                            }
                            div.tab-content.tab-content-active data-tab-id ="1" {
                                "Welcome to the website of the Geometry Dash Challenge List! Here you'll find the list of the hardest challenges that Geometry Dash's playerbase has to offer."
                            }
                        }
                        /*div.tab-selection.flex.wrap style="padding: 20px 0px; text-align: center"{
                            div.tab.tab-active.hover.scale data-tab-id="1" style="padding: 10px" {
                                h3 {
                                    "Ranking"
                                }
                                i class = "fa fa-list-ol fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="2" style="padding: 10px" {
                                h3 {
                                    "Stats Viewer"
                                }
                                i class = "fa fa-globe fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="3" style="padding: 10px" {
                                h3 {
                                    "Records"
                                }
                                i class = "fa fa-trophy fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="4" style="padding: 10px" {
                                h3 {
                                    "Informative"
                                }
                                i class = "fa fa-info fa-2x" aria-hidden="true" {}
                            }
                        }*/
                    }
                }
            }
        
            /*div.center.information-banner.left {
                div.tab-display#changelog-tabs {
                    div style = "display: flex; flex-flow: column;" {
                        h2 style="text-align: left; margin-top: 0px" {
                            "Changelog"
                        }
                        div.tab-content.tab-content-active data-tab-id ="98" {
                            h3 style="text-align: left; font-size: 110%" {
                                "2020: Not a lot tbh!"
                            }
                            p {
                                "Keeping up with the tradition of updating this part of the website precisely once a year, I find myself not really able to report about a major update at all. While there were some quite significant changes made to pointercrate this year, they were mostly internal, or not available to the public. Which is why I am using this entry to just quickly summarize the few things that did change this year"
                            }
                            ul {
                                li {
                                    "GD Integration has been reworked and is now based upon a must more solid foundation. Our new GD connector, dash-rs, was developed in collaboration with " a.link href = "https://github.com/mgostIH" {"mgostIH"} "."
                                }
                                li {
                                    "Back in march, pointercrate's internals were rewritten to utilize the then-stabilized " code { "async/await" } " mechanics. At the same time, the entire database layer was rewritten. "
                                }
                                li {
                                    "Since september, the list mods use a web interface for managing the list, instead of a disord bot"
                                }
                            }
                            p {
                                "Some of these changes will bring improvements to the public facing parts of pointercrate soon hopefully. Mainly, there are plans to overhault the record submitters using some of the UI components that have been developed for the internal management interface. A completely list of planned featured can also be found on " a.link href = "https://trello.com/b/10tslTjl/pointercrate" { "this trello board" } "."
                            }
                        }
                        div.tab-content data-tab-id ="99" {
                            h3 style="text-align: left; font-size: 110%" {
                                "2019-03-02: Rustification!"
                            }
                            p {
                                "The entire website has been rewritten in Rust! Various minor bugs that were noticed while porting over from the old python backend were fixed and performance has greatly improved. Other than that, it's mostly an internal change, so the list of (visible) changes is rather short:"
                            }
                            ul {
                                li {
                                    "I have (yet again) redesigned the home page! Most notably, it has been merged it with the former about page, as both were very under-utilized."
                                }
                                li {
                                    "The Demonlist now has an overview page over"
                                    a href = "/challenges/" { " here. " }
                                    "which shows an actual list (revolutionary, I know) of all demons"
                                }
                                li {
                                    "The API has been majorly overhauled. " i{"Most"} " changes should be backward compatible, since I just added a ton of fields, but there are some other changes as well, so be sure to check the documentation again. Mainly, there is no " code { "state" } " parameter for demons anymore and " code{"player.beaten"} " has become the more general " code{"player.records"} "."
                                }
                                li {
                                    "There is now deeper integration with the GD servers, allowing me to display additions stats for each demon. The code to calculate level length and accurately count objects was contributed by cos8o, so big thanks to him!"
                                }
                            }
                            p {
                                "Now onto some more serious topics: As some of you might know, I took up a second undergrad course (mathematics) in october, meaning my university schedule became much more demanding, leaving me nearly no time to work on pointercrate. Development on discord bots related to pointercrate and the Demonlist has already been taken over by GunnerBones, and with pointercrate becoming open source, I'm hoping to find more people will to work on it. "
                            }
                        }
                        div.tab-content data-tab-id ="100" {
                            h3 style="text-align: left" {
                                "2018-04-04: Anniversary Update!"
                            }
                            p {
                                "Its been one year since I rented the pointercrate domain and started hosting the Demonlist! Today I'm happy to announce the official pointercrate API, which can be used to programmatically access the Demonlist. The documentation can be found"
                                a href = "/documentation/" { " here. " }
                                "Further minor changes include:"
                            }
                            ul {
                                li {
                                    "Internal rework of how list mods are authenticated. They now use the account system."
                                }
                                li {
                                    "The website now embeds nicely into discord!"
                                }
                                li {
                                    "We added a link to the official Demonlist discord server, which is an awesome place where I get help with spellchecking"
                                }
                                li {
                                    "There is now a public discord bot that integrates with the Demonlist! Find it in the discord server!"
                                }
                                li {
                                    "The API is actually just the first step in something way more awesome hopefully coming \"soon\"... :)"
                                }
                            }
                        }
                        div.tab-content data-tab-id ="101" {
                            h3 style="text-align: left" {
                                "2017-10-29: New design!"
                            }
                            p {
                                "Pointercrate's design has been completely overhauled, to fix various issues we had with the old version:"
                            }
                            ul {
                                li {
                                    "The old homepage was thrown together in 5 minutes and you all knew it"
                                }
                                li {
                                    "Scrollbars were working weirdly or not at all"
                                }
                                li {
                                    "On mobile you couldn't close the Demonlist after clicking that weird button in the bottom left corner"
                                }
                                li {
                                    "There was way too much blue"
                                }
                            }
                            p {
                                "Most of these issues arose because the old version was not designed with mobile in mind, and mobile support was 'hacked in' later. The new design uses a mobile-first approach and should be a lot more responsive."
                            }
                        }
                    }
                    aside.tab-selection style="padding: 20px 0px; text-align: center"{
                        h3.tab.tab-active data-tab-id="98" style="padding: 10px; text-align:left" { "2020" }
                        h3.tab data-tab-id="99" style="padding: 10px; text-align:left" { "2019-03-02" }
                        h3.tab data-tab-id="100" style="padding: 10px; text-align:left" { "2018-04-04" }
                        h3.tab data-tab-id="101" style="padding: 10px; text-align: left" { "2017-10-29" }
                    }
                }
            }*/
            div.center.information-banner.right {
                div style = "flex-flow: column" {
                    h1#contact {
                        "Contacts & Staff Members"
                    }
                    div.flex#about-inner {
                        div style = "flex-basis: 0; padding: 5px" {
                            h2 { "Challenge List Team: "}
                            p {
                                "The Challenge List is managed by a fairly sized team of players consisting of: "
                            }
                            div.flex.wrap style = "padding: 20px" {
                                @for member in &self.demonlist_team {
                                    h4 style="display: inline; margin: 5px" { (member.name()) }
                                }

                                @for member in &self.demonlist_subteam {
                                    h4 style="display: inline; margin: 5px; font-weight: normal" { (member.name()) }
                                }
                            }
                            /*p {
                                "Contact these people for any list related questions/issues"
                            }
                            i {
                                "Twitter: "
                                a href = "https://twitter.com/demonlistgd" {"demonlistgd"}
                            }
                            br ;
                            i {
                                "YouTube: "
                                a href = "https://www.youtube.com/channel/UCqI5feGZEqJRp6VcrP5gVyw" {"Demon List GD"}
                            }
                            br ;
                            i {
                                "Twitch: "
                                a href = "https://twitch.tv/demonlistgd/" {"DemonListGD"}
                            }
                            br ;
                            i {
                                "Discord: "
                                a href = "https://discord.gg/cZcBxQT" {"Demon List Public Server"}
                            }*/
                        }
                        div style = "flex-basis: 0; padding: 5px" {
                            h2 { "Development Team: "}
                            p {
                                "This instance of Pointercrate, and all related softwares and technologies to the GD Challenge List are managed by:"
                            }
                            div.flex.wrap style = "padding: 20px" {
                                @for member in &self.pointercrate_team {
                                    h4 style="display: inline; margin: 5px" { (member.name()) }
                                }

                                @for member in &self.pointercrate_subteam {
                                    h4 style="display: inline; margin: 5px; font-weight:normal" { (member.name()) }
                                }
                            }
                            /*p {
                                "Contact these people for suggestion for pointercrate itself, bug reports or programming related questions"
                            }
                            i {
                                "Twitter: "
                                a href = "https://twitter.com/stadust1971" {"stadust - pointercrate"}
                            }
                            br ;
                            i {
                                "Discord: "
                                a href = "https://discord.gg/sQewUEB" {"Pointercrate Central"}
                            }*/
                        }
                    }
                }
            }
        }
    }

    fn head(&self) -> Vec<Markup> {
        vec![html! {
            (PreEscaped(r#"
<style>
    .tab-active {
        color: #0881c6;
    }
</style>
<script type="application/ld+json">
  {
    "@context": "http://schema.org",
    "@type": "Organization",
    "name": "pointercrate",
    "description": "Pointercrate is the home of the official Demonlist, a ranking of the hardest rated demons maintained by some of the game's most skilled players",
    "url": "https://pointercrate.com/",
    "logo": "https://pointercrate.com/static2/images/pointercrate2.png",
    "sameAs": [
      "https://twitter.com/demonlistgd",
      "https://www.youtube.com/channel/UCqI5feGZEqJRp6VcrP5gVyw"
    ]
  }
</script>
            "#))
        }]
    }
}
