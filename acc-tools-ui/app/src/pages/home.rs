use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <main class="container mx-auto max-w-3xl px-4 py-16">
      <h1 class="text-3xl font-bold text-center mb-2">"Apex Predators Sim Racing"</h1>
      <p class="text-center text-muted-foreground mb-12">"Choose a tool to get started"</p>

      <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
        <ToolCard
          href="/session"
          title="ACC Session Viewer"
          description="Load a JSON or PDF qualifying/race session file and explore per-driver statistics."
        />
        <ToolCard
          href="/telemetry"
          title="MoTeC Telemetry Viewer"
          description="Load a MoTeC .ld telemetry file with an optional .ldx lap-marker sidecar and inspect channel data."
        />
      </div>
    </main>
  }
}

#[component]
fn ToolCard(href: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
  view! {
    <a
      href=href
      class="group block rounded-lg border bg-card p-6 shadow-sm transition-colors hover:border-primary hover:bg-card/80"
    >
      <h2 class="text-lg font-semibold text-card-foreground mb-2 group-hover:text-primary transition-colors">
        {title}
      </h2>
      <p class="text-sm text-muted-foreground">{description}</p>
    </a>
  }
}
