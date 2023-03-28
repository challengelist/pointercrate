pub use self::{
    demon_page::page,
    overview::{index, overview_demons, OverviewDemon},
};
use crate::{
    config,
    model::{demonlist::demon::Demon, nationality::Nationality},
};
use maud::{html, Markup, PreEscaped, Render};

mod demon_page;
mod overview;

struct ListSection {
    name: &'static str,
    description: &'static str,
    id: &'static str,
    numbered: bool,
}

static MAIN_SECTION: ListSection = ListSection {
    name: "Main List",
    description: "The main list section, holding the top hardest challenges. Records here are given a large amounts of points.",
    id: "mainlist",
    numbered: true,
};

// UNUSED
static EXTENDED_SECTION: ListSection = ListSection {
    name: "Extended List",
    description: "These are Challenges that dont qualify for the main section of the list, but are still of high relevance. Only 100% records \
                  are accepted for these Challenges! Note that non-100% that were submitted/approved before a demon fell off the main list \
                  will be retained",
    id: "extended",
    numbered: true,
};

static LEGACY_SECTION: ListSection = ListSection {
    name: "Legacy List",
    description: "These are the challenges that have fallen out of grace, and no longer are eligible to be given points.",
    id: "legacy",
    numbered: false,
};

fn dropdowns(all_demons: &[OverviewDemon], current: Option<&Demon>) -> Markup {
    let (main, extended, legacy) = if all_demons.len() < config::list_size() as usize {;
        (&all_demons[..], Default::default(), Default::default())
    } else {
        let (extended, legacy) = if all_demons.len() < config::extended_list_size() as usize {
            (&all_demons[config::list_size() as usize..], Default::default())
        } else {
            (
                &all_demons[config::list_size() as usize..config::extended_list_size() as usize],
                &all_demons[config::extended_list_size() as usize..],
            )
        };

        (&all_demons[..config::list_size() as usize], extended, legacy)
    };

    html! {
        nav.flex.wrap.m-center.fade#lists style="text-align: center;" {
            // The drop down for the main list:
            (dropdown(&MAIN_SECTION, main, current))
            // The drop down for the extended list:
            /*(dropdown(&EXTENDED_SECTION, extended, current))*/ // later
            // The drop down for the legacy list:
            (dropdown(&LEGACY_SECTION, legacy, current))
        }
    }
}

fn dropdown(section: &ListSection, demons: &[OverviewDemon], current: Option<&Demon>) -> Markup {
    let format = |demon: &OverviewDemon| -> Markup {
        html! {
            a href = {"/challenges/" (demon.position)} {
                @if section.numbered {
                    {"#" (demon.position) " - " (demon.name)}
                    br ;
                    i {
                        (demon.publisher)
                    }
                }
                @else {
                    {(demon.name)}
                    br ;
                    i {
                        (demon.publisher)
                    }
                }
            }
        }
    };

    html! {
        div {
            div.button.white.hover.no-shadow.js-toggle data-toggle-group="0" onclick={"javascript:void(DropDown.toggleDropDown('" (section.id) "'))"} {
                (section.name)
            }

            div.see-through.fade.dropdown#(section.id) {
                div.search.js-search.seperated style = "margin: 10px" {
                    input placeholder = "Filter..." type = "text" {}
                }
                p style = "margin: 10px" {
                    (section.description)
                }
                ul.flex.wrap.space {
                    @for demon in demons {
                        @match current {
                            Some(current) if current.base.position == demon.position =>
                                li.hover.white.active title={"#" (demon.position) " - " (demon.name)} {
                                    (format(demon))
                                },
                            _ =>
                                li.hover.white title={"#" (demon.position) " - " (demon.name)} {
                                    (format(demon))
                                }
                        }
                    }
                }
            }
        }
    }
}

pub fn demon_dropdown<'a>(dropdown_id: &str, demons: impl Iterator<Item = &'a OverviewDemon>) -> Markup {
    html! {
        div.dropdown-menu.js-search#(dropdown_id) {
            input type = "text" name = "demon" required="" autocomplete="off";
            div.menu {
               ul {
                    @for demon in demons {
                        li.white.hover data-value = (demon.id) data-display = (demon.name) {b{"#"(demon.position) " - " (demon.name)} br; {"by "(demon.publisher)}}
                    }
                }
            }
        }
    }
}

pub fn player_selection_dialog(dialog_id: &str, headline: &str, description: &str, button_text: &str) -> Markup {
    html! {
        div.overlay.closable {
            div.dialog#(dialog_id) {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    (headline)
                }
                div.flex.viewer {
                    (crate::view::filtered_paginator(&format!("{}-pagination", dialog_id), "/api/v1/players/"))
                    div {
                        p {
                            (description)
                        }
                        form.flex.col novalidate = "" {
                            p.info-red.output {}
                            p.info-green.output {}
                            span.form-input#{(dialog_id)"-input"} {
                                label for = "player" {"Player name:"}
                                input name = "player" type="text" required = "";
                                p.error {}
                            }
                            input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value = (button_text);
                        }
                    }
                }
            }
        }
    }
}

pub(super) fn submission_panel(demons: &[OverviewDemon]) -> Markup {
    html! {
        section.panel.fade.closable#submitter style = "display: none" {
            span.plus.cross.hover {}
            div.flex {
                form#submission-form novalidate = "" {
                    div.underlined {
                        h2 {"Record Submission"}
                    }
                    p.info-red.output {}
                    p.info-green.output {}
                    h3 {
                        "Challenge:"
                    }
                    p {
                        "The challenge that the record was made on."
                    }
                    span.form-input data-type = "dropdown" {
                        (demon_dropdown("id_demon", demons.iter()))
                        p.error {}
                    }
                    h3 {
                        "Holder:"
                    }
                    p {
                        "The holder of the record. Click the pencil to select a player!"
                    }
                    span.form-input.flex.col#id_player data-type = "html" data-target-id = "selected-holder" data-default = "None Selected" {
                        span {
                            b {
                                i.fa.fa-pencil.clickable#record-submitter-holder-pen aria-hidden = "true" {}
                                " "
                            }
                            i#selected-holder data-name = "player" {"None Selected"}
                        }
                        p.error {}
                    }
                    h3 {
                        "Video: "
                    }
                    p {
                        "A proof video of the legitimacy of the given record. If the record was achieved on stream, but wasn't uploaded anywhere else, please provide a twitch link to that stream."
                        br {}

                        i { "Note: " }
                        "Please pay attention to only submit well-formed URLs!"
                    }
                    span.form-input.flex.col#id_video {
                        input type = "url" name = "video" required = "" placeholder = "e.g. 'https://youtu.be/cHEGAqOgddA'" ;
                        p.error {}
                    }

                    h3 {
                        "Raw Footage: "
                    }
                    p {
                        "The full unedited recording of your submission."
                        br {}

                        i { "Note: " }
                        "Please pay attention to only submit well-formed URLs! Ideally, we would prefer for you to submit Google Drive, Mediafire, or Mega URLs."
                    }
                    span.form-input.flex.col#id_raw_footage {
                        input type = "url" name = "raw_footage" required = "" placeholder = "e.g. 'https://drive.google.com/file/d/1SJxMqUuh0QgbilyrrDIFsQZQV3XhtXpK/view?usp=sharing'" ;
                        p.error {}
                    }
                    h3 {
                        "Notes or comments: "
                    }
                    p {
                        "Provide any additional notes you'd like to pass on to the team member receiving your submission."
                    }
                    span.form-input.flex.col#submit-note {
                        textarea name = "note" placeholder = "Your dreams and hopes for this record... or something like that" {}
                        p.error {}
                    }
                    p {
                        "By submitting the record you acknowledge the " a.link href = "/guidelines" {"submission guidelines"} "."
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value="Submit record";
                }
            }
        }
        (player_selection_dialog(
            "submission-holder-dialog",
            "Select player:",
            "To select the player holding this record, search them up on the left to see if they already have records on the list and click them. In case the player does not exist, fill out only the text field on the right.",
            "Select"
        ))
    }
}

fn stats_viewer(nations: &[Nationality]) -> Markup {
    html! {
        section.panel.fade.closable#statsviewer style = "display:none" {
            span.plus.cross.hover {}
            h2.underlined.pad {
                "Stats Viewer - "
                (super::dropdown("International",
                    html! {
                        li.white.hover.underlined data-value = "International" data-display = "International" {
                            span.em.em-world_map {}
                            (PreEscaped("&nbsp;"))
                            b {"WORLD"}
                            br;
                            span style = "font-size: 90%; font-style: italic" { "International" }
                        }
                    },
                    nations.iter().map(|nation| html! {
                        li.white.hover data-value = {(nation.iso_country_code)} data-display = {(nation.nation)} {
                            span class = {"flag-icon flag-icon-" (nation.iso_country_code.to_lowercase())} {}
                            (PreEscaped("&nbsp;"))
                            b {(nation.iso_country_code)}
                            br;
                            span style = "font-size: 90%; font-style: italic" {(nation.nation)}
                        }
                    })
                ))
            }
            div.flex.viewer {
                (super::filtered_paginator("stats-viewer-pagination", "/api/v1/players/ranking/"))
                p.viewer-welcome {
                    "Click on a player's name on the left to get started!"
                }
                div.viewer-content {
                    div {
                        div.flex.col {
                            h3#player-name style = "font-size:1.4em; overflow: hidden" {}
                            div.stats-container.flex.space {
                                span {
                                    b {
                                        "List Challenges completed:"
                                    }
                                    br;
                                    span#amount-beaten {}
                                }
                                span {
                                    b {
                                        "Legacy Challenges completed:"
                                    }
                                    br;
                                    span#amount-legacy {}
                                }
                                span {
                                    b {
                                        "Challenge List Score:"
                                    }
                                    br;
                                    span#score {}
                                }
                            }
                            div.stats-container.flex.space {
                                span {
                                    b {
                                        "Challenge List Rank:"
                                    }
                                    br;
                                    span#rank {}
                                }
                                span {
                                    b {
                                        "Hardest Challenge:"
                                    }
                                    br;
                                    span#hardest {}
                                }
                            }
                            div.stats-container.flex.space {
                                span {
                                    b {
                                        "Challenges completed:"
                                    }
                                    br;
                                    span#beaten {}
                                }
                            }
                            div.stats-container.flex.space {
                                span {
                                    b {
                                        "List Challenges created:"
                                    }
                                    br;
                                    span#created {}
                                }
                                span {
                                    b {
                                        "List Challenges published:"
                                    }
                                    br;
                                    span#published {}
                                }
                                span {
                                    b {
                                        "List Challenges verified:"
                                    }
                                    br;
                                    span#verified {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn sidebar_ad() -> Markup {
    html! {
    }
}

fn rules_panel() -> Markup {
    html! {
        section#rules.panel.fade.js-scroll-anim data-anim = "fade" {
            h2.underlined.pad.clickable {
                "Guidelines:"
            }
            p {
                "All of the guidelines for the Challenge List, whether it be for challenges added, or submitting one; will all be listed "
                a href = "https://docs.google.com/document/d/1-hhDtCyyNMT4F0IBBidmSs3djbHT6JOt7FRohUUjbbc/edit?usp=drivesdk" {
                    "here"
                }
            }
            a.blue.hover.button href = "https://docs.google.com/document/d/1-hhDtCyyNMT4F0IBBidmSs3djbHT6JOt7FRohUUjbbc/edit?usp=drivesdk" {
                "Read the guidelines!"
            }
            /*a href = "https://docs.google.com/document/d/1-hhDtCyyNMT4F0IBBidmSs3djbHT6JOt7FRohUUjbbc/edit?usp=sharing" {
                h3 style = "color: #5879fc  " {
                    "The Guidelines"
                }
                "All demonlist operations are carried out in accordance to our guidelines. Be sure to check them before submitting a record to ensure a flawless experience!"
            }
            a.blue.hover.button href = "/guidelines/" {
                "Read the guidelines!"
            }*/
        }
    }
}

pub(super) fn submit_panel() -> Markup {
    html! {
        section#submit.panel.fade.js-scroll-anim data-anim = "fade" {
            div.underlined {
                h2 {
                    "Submit Records:"
                }
            }
            p {
                "Note: Please do not submit nonsense, it only makes it harder for us all and will get you banned. Also note that the form rejects duplicate submissions."
            }
            a.blue.hover.button.js-scroll data-destination = "submitter" data-reveal = "true" {
                "Submit a record!"
            }
        }
    }
}

fn stats_viewer_panel() -> Markup {
    html! {
        section#stats.panel.fade.js-scroll-anim data-anim = "fade" {
            div.underlined {
                h2 {
                    "Stats Viewer:"
                }
            }
            p {
                "Get a detailed overview of who completed the most, created the most challenges or beat the hardest ones! There is even a leaderboard to compare yourself to the very best!"
            }
            a.blue.hover.button.js-scroll#show-stats-viewer data-destination = "statsviewer" data-reveal = "true" {
                "Open the stats viewer!"
            }
        }
    }
}

fn discord_panel() -> Markup {
    html! {
        section.panel.fade.js-scroll-anim#discord data-anim = "fade" {
            iframe.js-delay-attr style = "width: 100%; height: 400px;" allowtransparency="true" frameborder = "0" data-attr = "src" data-attr-value = "https://ptb.discord.com/widget?id=689275234418688036&theme=dark" {}
            p {
                "Join the official Challenge List discord server, where you can very easily get in touch with the moderators and staff!    "
            }
        }
    }
}

impl Render for Nationality {
    fn render(&self) -> Markup {
        html! {
            span.flag-icon.{"flag-icon-"(self.iso_country_code.to_lowercase())} title = (self.nation) {}
        }
    }
}
