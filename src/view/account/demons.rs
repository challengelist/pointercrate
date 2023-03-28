use crate::view::{demonlist, filtered_paginator};
use maud::{html, Markup, PreEscaped};

pub(super) fn page() -> Markup {
    html! {
        div.m-center.flex.tab-content.container data-tab-id = "5"{
            div.left {
                (demon_submitter())
                div.panel.fade {
                    h2.underlined.pad {
                        "Challenge Manager"
                    }
                    div.flex.viewer {
                        (filtered_paginator("demon-pagination", "/api/v2/demons/listed/"))
                        p.viewer-welcome {
                            "Click on a challenge on the left to get started!"
                        }

                        div.viewer-content {
                            div.flex.col{
                                h3 style = "font-size:1.1em; margin: 10px 0" {
                                    "Challenge #"
                                    i#demon-demon-id {}
                                    " - "
                                    i.fa.fa-pencil.clickable#demon-name-pen aria-hidden = "true" {} (PreEscaped("&nbsp;")) i#demon-demon-name {}
                                }

                                iframe."ratio-16-9"#demon-video style="width:90%; margin: 15px 5%" allowfullscreen="" {"Verification Video"}
                                p.info-red.output style = "margin: 10px" {}
                                p.info-green.output style = "margin: 10px" {}
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil.clickable#demon-video-pen aria-hidden = "true" {} " Verification Video:"
                                        }
                                        br;
                                        a.link#demon-video-link target = "_blank" {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil.clickable#demon-position-pen aria-hidden = "true" {} " Position:"
                                        }
                                        br;
                                        span#demon-position {}
                                    }
                                    span{
                                        b {
                                            i.fa.fa-pencil.clickable#demon-publisher-pen aria-hidden = "true" {} " Publisher:"
                                        }
                                        br;
                                        span#demon-publisher {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil.clickable#demon-verifier-pen aria-hidden = "true" {} " Verifier:"
                                        }
                                        br;
                                        span#demon-verifier {}
                                    }
                                    span{
                                        i.fa.fa-plus.clickable#demon-add-creator-pen aria-hidden = "true" {} b {
                                            " Creators:"
                                        }
                                        br;
                                        span#demon-creators {}

                                    }
                                }
                                div.stats-container.flex.space  {
                                    span {
                                        b {
                                            i.fa.fa-pencil.clickable#demon-fps-pen aria-hidden = "true" {} " FPS:"
                                        }
                                        br;
                                        span#demon-fps {}
                                    }
                                    span {
                                        b {
                                            "Hidden:"
                                        }
                                        br;
                                        div.dropdown-menu.js-search#edit-demon-hidden style = "max-width: 50px"{
                                            input type="text" style = "color: #444446; font-weight: bold;";
                                            div.menu {
                                                ul {
                                                    li.white.hover data-value="true" {"yes"}
                                                    li.white.hover data-value="false" {"no"}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div style="height: 50px" {} // to make sure that the footer doesnt float. if it floats, the user page is the only one without a scrollbar at the right, which causes jumpiness when switching tabs.
            }
            div.right {
                (submit_panel())
            }
            (change_name_dialog())
            (change_position_dialog())
            (change_fps_dialog())
            (change_video_dialog())
            (change_verifier_dialog())
            (change_publisher_dialog())
            (add_creator_dialog())
        }
    }
}

pub(super) fn submit_panel() -> Markup {
    html! {
        section.panel.fade.js-scroll-anim data-anim = "fade" {
            div.underlined {
                h2 {
                    "Add Challenge:"
                }
            }
            a.blue.hover.button.js-scroll data-destination = "demon-submitter" data-reveal = "true" {
                "Add a Challenge!"
            }
        }
    }
}

fn change_name_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog#demon-name-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change challenge name:"
                }
                p style = "max-width: 400px"{
                    "Change the name of this challenge. Multiple challenges with the same name ARE supported!"
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input#demon-name-edit {
                        label for = "name" {"Name:"}
                        input name = "name" type = "text" required = "";
                        p.error {}
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_position_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog#demon-position-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change demon position:"
                }
                p style = "max-width: 400px"{
                    "Change the position of this demon. Has be be greater than 0 and be at most the current list size."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input#demon-position-edit {
                        label for = "position" {"Position:"}
                        input name = "position" type = "number" min = "1" required = "";
                        p.error {}
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_fps_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog#demon-fps-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change FPS:"
                }
                p style = "max-width: 400px"{
                    "Change the accepted FPS value of this challenge. Leave it empty to set the value to \"Any\"."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input#demon-fps-edit {
                        label for = "fps" {"FPS:"}
                        input name = "fps" type = "text";
                        p.error {}
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_video_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog#demon-video-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change verification video link:"
                }
                p style = "max-width: 400px"{
                    "Change the verification video link for this record. Leave empty to remove the verification video. ."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input#demon-video-edit {
                        label for = "video" {"Video link:"}
                        input name = "video" type = "url";
                        p.error {}
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_verifier_dialog() -> Markup {
    demonlist::player_selection_dialog(
        "demon-verifier-dialog",
        "Change demon verifier:",
        "Change the verifier of this demon. If the player you want to change the verifier to already exists, search them up on the left \
         and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
         create a new player.",
        "Edit",
    )
}

fn change_publisher_dialog() -> Markup {
    demonlist::player_selection_dialog(
        "demon-publisher-dialog",
        "Change demon publisher:",
        "Change the publisher of this demon. If the player you want to change the publisher to already exists, search them up on the left \
         and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
         create a new player.",
        "Edit",
    )
}

fn add_creator_dialog() -> Markup {
    demonlist::player_selection_dialog(
        "demon-add-creator-dialog",
        "Add creator:",
        "Select a creator to add to this demon. If the player you want to change the publisher to already exists, search them up on the \
         left and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server \
         to create a new player.",
        "Add Creator",
    )
}

fn demon_submitter() -> Markup {
    html! {
        section.panel.fade.closable#demon-submitter style = "display: none" {
            span.plus.cross.hover {}
            div.flex {
                form#demon-submission-form novalidate = "" {
                    div.underlined {
                        h2 {"Add Challenge:"}
                    }
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input.flex.col#demon-add-name {
                        label for = "name" {
                            "Challenge Name:"
                        }
                        input type = "text" name = "name" required="";
                        p.error {}
                    }
                    span.form-input.flex.col#demon-add-position {
                        label for = "position" {
                            "Position:"
                        }
                        input type = "number" name = "position" required="" min="1";
                        p.error {}
                    }
                    span.form-input.flex.col#demon-add-fps {
                        label for = "fps" {
                            "FPS"
                        }
                        input type = "text" name = "fps";
                        p.error {}
                    }
                    span.form-input.flex.col#demon-add-verifier data-type = "html" data-target-id = "selected-verifier" data-default = "None Selected" {
                        label{"Verifier:"}
                        br;
                        span {
                            b {
                                i.fa.fa-pencil.clickable#demon-add-verifier-pen aria-hidden = "true" {}
                                " "
                            }
                            i#selected-verifier data-name = "verifier" {"None Selected"}
                        }
                        p.error {}
                    }
                    span.form-input.flex.col#demon-add-publisher data-type = "html" data-target-id = "selected-publisher" data-default = "None Selected" {
                        label {"Publisher:"}
                        br;
                        span {
                            b {
                                i.fa.fa-pencil.clickable#demon-add-publisher-pen aria-hidden = "true" {}
                                " "
                            }
                            i#selected-publisher data-name = "publisher" {"None Selected"}
                        }
                        p.error {}
                    }
                    span.form-input.flex.col#demon-add-video {
                        label for = "video" {
                            "Verification Video:"
                        }
                        input type = "url" name = "video";
                        p.error {}
                    }
                    span {
                        i.fa.fa-plus.clickable#add-demon-add-creator-pen aria-hidden = "true" {} i {
                            " Creators: "
                        }
                        span#demon-add-creators {}
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value="Add Challenge";
                }
            }
        }
        (demonlist::player_selection_dialog(
            "demon-add-verifier-dialog",
            "Set demon verifier:",
            "Set the verifier of this demon. If the player you want to set as verifier already exists, search them up on the left \
             and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
             create a new player.",
            "Select",
        ))
        (demonlist::player_selection_dialog(
            "demon-add-publisher-dialog",
            "Set demon publisher:",
            "Set the publisher of this demon. If the player you want to set as publisher already exists, search them up on the left \
             and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
             create a new player.",
            "Select",
        ))
    }
}
