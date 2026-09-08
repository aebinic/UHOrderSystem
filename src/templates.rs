use maud::{html, Markup, PreEscaped, DOCTYPE};

const STYLE: &str = r#"
    :root {
        --bg: #0f172a;
        --panel: #ffffff;
        --accent: #4f46e5;
        --accent-dark: #4338ca;
        --text: #1e293b;
        --muted: #64748b;
        --border: #e2e8f0;
    }
    * { box-sizing: border-box; }
    body {
        margin: 0;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        background: linear-gradient(135deg, #0f172a 0%, #4338ca 100%);
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2rem;
    }
    .layout {
        display: flex;
        max-width: 960px;
        width: 100%;
        background: var(--panel);
        border-radius: 16px;
        overflow: hidden;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.35);
    }
    .hero {
        flex: 1.1;
        background: radial-gradient(circle at 30% 20%, #6366f1, #312e81);
        color: white;
        padding: 3rem 2.5rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
    .hero h1 {
        font-size: 2.25rem;
        margin: 0 0 1rem;
        line-height: 1.2;
    }
    .hero p {
        color: #c7d2fe;
        line-height: 1.6;
        margin: 0 0 1.5rem;
    }
    .hero ul {
        list-style: none;
        padding: 0;
        margin: 0;
        display: grid;
        gap: 0.75rem;
    }
    .hero li {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        color: #e0e7ff;
        font-size: 0.95rem;
    }
    .hero li::before {
        content: "✓";
        color: #a5b4fc;
        font-weight: bold;
    }
    .auth {
        flex: 1;
        padding: 3rem 2.5rem;
        display: flex;
        flex-direction: column;
    }
    .tabs {
        display: flex;
        border-bottom: 1px solid var(--border);
        margin-bottom: 1.75rem;
    }
    .tabs label {
        flex: 1;
        text-align: center;
        padding: 0.75rem 0;
        cursor: pointer;
        color: var(--muted);
        font-weight: 600;
        border-bottom: 2px solid transparent;
        transition: color 0.15s, border-color 0.15s;
    }
    input.tab-toggle {
        display: none;
    }
    #tab-login:checked ~ .tabs label[for="tab-login"],
    #tab-signup:checked ~ .tabs label[for="tab-signup"] {
        color: var(--accent);
        border-color: var(--accent);
    }
    .panel {
        display: none;
    }
    #tab-login:checked ~ .panels #panel-login,
    #tab-signup:checked ~ .panels #panel-signup {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
    .field {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }
    .field label {
        font-size: 0.85rem;
        font-weight: 600;
        color: var(--text);
    }
    .field input {
        padding: 0.7rem 0.85rem;
        border: 1px solid var(--border);
        border-radius: 8px;
        font-size: 0.95rem;
        outline: none;
        transition: border-color 0.15s, box-shadow 0.15s;
    }
    .field input:focus {
        border-color: var(--accent);
        box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.15);
    }
    .submit {
        margin-top: 0.5rem;
        padding: 0.75rem;
        border: none;
        border-radius: 8px;
        background: var(--accent);
        color: white;
        font-size: 0.95rem;
        font-weight: 600;
        cursor: pointer;
        transition: background 0.15s;
    }
    .submit:hover {
        background: var(--accent-dark);
    }
    .hint {
        text-align: center;
        font-size: 0.85rem;
        color: var(--muted);
        margin-top: 1rem;
    }
    @media (max-width: 720px) {
        .layout { flex-direction: column; }
        .hero { padding: 2.5rem; }
    }
"#;

pub fn landing_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "UH Order System" }
                style { (PreEscaped(STYLE)) }
            }
            body {
                div class="layout" {
                    div class="hero" {
                        h1 { "UH Order System" }
                        p { "Manage orders end to end in one place, built for speed and clarity." }
                        ul {
                            li { "Real-time order tracking" }
                            li { "Simple, fast checkout" }
                            li { "Built for teams of any size" }
                        }
                    }
                    div class="auth" {
                        input type="radio" name="auth-tab" id="tab-login" class="tab-toggle" checked;
                        input type="radio" name="auth-tab" id="tab-signup" class="tab-toggle";
                        div class="tabs" {
                            label for="tab-login" { "Log In" }
                            label for="tab-signup" { "Sign Up" }
                        }
                        div class="panels" {
                            form id="panel-login" class="panel" action="/login" method="post" {
                                div class="field" {
                                    label for="login-email" { "Email" }
                                    input id="login-email" type="email" name="email" placeholder="you@example.com" required;
                                }
                                div class="field" {
                                    label for="login-password" { "Password" }
                                    input id="login-password" type="password" name="password" placeholder="••••••••" required;
                                }
                                button type="submit" class="submit" { "Log In" }
                            }
                            form id="panel-signup" class="panel" action="/signup" method="post" {
                                div class="field" {
                                    label for="signup-name" { "Full name" }
                                    input id="signup-name" type="text" name="name" placeholder="Jane Doe" required;
                                }
                                div class="field" {
                                    label for="signup-email" { "Email" }
                                    input id="signup-email" type="email" name="email" placeholder="you@example.com" required;
                                }
                                div class="field" {
                                    label for="signup-password" { "Password" }
                                    input id="signup-password" type="password" name="password" placeholder="••••••••" required;
                                }
                                button type="submit" class="submit" { "Create Account" }
                            }
                        }
                        p class="hint" { "Use the tabs above to switch between logging in and creating an account." }
                    }
                }
            }
        }
    }
}
