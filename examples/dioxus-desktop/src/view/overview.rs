use dioxus::prelude::*;
use dioxus_free_icons::icons::go_icons::*;
use zino_dioxus::prelude::*;

pub fn Overview() -> Element {
    let core_crates = [
        ("zino", "Framework integrations"),
        ("zino-core", "Core types and traits"),
        ("zino-derive", "Derived traits"),
        ("zino-model", "Domain models"),
    ];
    let server_crates = [
        ("zino-server", "A HTTP server"),
        ("zino-router", "A flexible router"),
        ("zino-middleware", "Middlewares"),
        ("zino-rpc", "RPC support"),
    ];
    let extra_crates = [
        ("zino-extra", "Extra utilities"),
        ("zino-dioxus", "Dioxus components"),
        ("zino-cli", "CLI tools"),
    ];
    rsx! {
        div {
            class: "columns is-6",
            div {
                class: "column",
                div {
                    class: "card",
                    header {
                        class: "card-header",
                        div {
                            class: "card-header-title",
                            SvgIcon { shape: GoMarkGithub, width: 14, intrinsic: true }
                            span {
                                class: "ml-1",
                                "GitHub status"
                            }
                        }
                    }
                    div {
                        class: "card-content",
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/languages/top/photino/zino",
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/badge/rustc-1.75+-blue.svg",
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/tags",
                            img {
                                src: "https://img.shields.io/github/v/tag/photino/zino",
                            }
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/repo-size/photino/zino",
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/languages/code-size/photino/zino",
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/stargazers",
                            img {
                                src: "https://img.shields.io/github/stars/photino/zino?style=flat",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/watchers",
                            img {
                                src: "https://img.shields.io/github/watchers/photino/zino?style=flat",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/forks",
                            img {
                                src: "https://img.shields.io/github/forks/photino/zino?style=flat",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/graphs/contributors",
                            img {
                                src: "https://img.shields.io/github/contributors/photino/zino",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/commits/main",
                            img {
                                src: "https://img.shields.io/github/last-commit/photino/zino/main",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/pulls",
                            img {
                                src: "https://img.shields.io/github/issues-pr/photino/zino"
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/actions/workflows/rust.yml",
                            img {
                                src: "https://img.shields.io/github/actions/workflow/status/photino/zino/rust.yml",
                            }
                        }
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/blob/main/LICENSE",
                            img {
                                src: "https://img.shields.io/github/license/photino/zino",
                            }
                        }
                    }
                }
            }
        }
        div {
            class: "columns is-6",
            div {
                class: "column",
                div {
                    class: "card",
                    header {
                        class: "card-header",
                        div {
                            class: "card-header-title",
                            SvgIcon { shape: GoIssueOpened, width: 14, intrinsic: true }
                            span {
                                class: "ml-1",
                                "GitHub issues"
                            }
                        }
                    }
                    div {
                        class: "card-content",
                        a {
                            class: "mr-2",
                            href: "https://github.com/zino-rs/zino/issues",
                            img {
                                src: "https://img.shields.io/github/issues/photino/zino",
                            }
                        }
                        for label in ["bug", "enhancement", "dependencies"] {
                            a {
                                class: "mr-2",
                                href: "https://github.com/zino-rs/zino/labels/{label}",
                                img {
                                    src: "https://img.shields.io/github/issues/photino/zino/{label}",
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "card",
                    header {
                        class: "card-header",
                        div {
                            class: "card-header-title",
                            SvgIcon { shape: GoHistory, width: 14, intrinsic: true }
                            span {
                                class: "ml-1",
                                "GitHub commits"
                            }
                        }
                    }
                    div {
                        class: "card-content",
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/commit-activity/t/photino/zino",
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/commit-activity/y/photino/zino",
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/commit-activity/m/photino/zino",
                        }
                        img {
                            class: "mr-2",
                            src: "https://img.shields.io/github/commit-activity/w/photino/zino",
                        }
                    }
                }
            }
        }
        div {
            class: "columns is-6",
            for d in core_crates {
                CrateListing {
                    name: d.0,
                    description: d.1,
                }
            }
        }
        div {
            class: "columns is-6",
            for d in server_crates {
                CrateListing {
                    name: d.0,
                    description: d.1,
                }
            }
        }
        div {
            class: "columns is-6",
            for d in extra_crates {
                CrateListing {
                    name: d.0,
                    description: d.1,
                }
            }
        }
    }
}

#[component]
fn CrateListing(name: String, description: String) -> Element {
    rsx! {
        div {
            class: "column is-one-quarter",
            div {
                class: "card",
                header {
                    class: "card-header",
                    div {
                        class: "card-header-title",
                        span {
                            class: "tag is-warning is-light mr-1",
                            "{name}"
                        }
                        span { "{description}" }
                    }
                }
                div {
                    class: "card-content",
                    a {
                        class: "mr-2",
                        href: "https://crates.io/crates/{name}",
                        img {
                            src: "https://img.shields.io/crates/v/{name}",
                        }
                    }
                    a {
                        class: "mr-2",
                        href: "https://docs.rs/{name}",
                        img {
                            src: "https://shields.io/docsrs/{name}",
                        }
                    }
                    img {
                        class: "mr-2",
                        src: "https://img.shields.io/crates/l/{name}",
                    }
                    img {
                        class: "mr-2",
                        src: "https://img.shields.io/crates/d/{name}"
                    }
                    img {
                        class: "mr-2",
                        src: "https://img.shields.io/crates/dr/{name}"
                    }
                }
            }
        }
    }
}
