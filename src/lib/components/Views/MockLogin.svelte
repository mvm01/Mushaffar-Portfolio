<script lang="ts">
  export let onContinue: () => void = () => {};
</script>

<section class="relative flex min-h-screen items-center justify-center overflow-hidden px-6 py-10">
  <div class="absolute inset-0 bg-mesh-gradient"></div>
  <div class="absolute left-1/2 top-10 h-72 w-72 -translate-x-1/2 rounded-full bg-cyan-400/10 blur-3xl"></div>

  <div class="relative w-full max-w-5xl overflow-hidden rounded-[36px] border border-white/10 bg-slate-950/65 shadow-glow backdrop-blur-2xl">
    <div class="grid min-h-[680px] lg:grid-cols-[1.1fr_0.9fr]">
      <div class="flex flex-col justify-between border-b border-white/10 p-8 lg:border-b-0 lg:border-r lg:p-12">
        <div class="space-y-6">
          <p class="text-sm uppercase tracking-[0.35em] text-cyan-200/70">Desktop UI Prototype</p>
          <h1 class="max-w-lg text-5xl font-semibold leading-tight text-white">
            Manage local records in a dashboard designed to feel premium.
          </h1>
          <p class="max-w-xl text-base text-slate-300">
            This mock login flows directly into a dark-mode control center with responsive cards,
            polished table styling, and local-only state.
          </p>
        </div>

        <div class="grid gap-4 md:grid-cols-3">
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Speed</p>
            <p class="mt-3 text-2xl font-semibold text-white">Tauri v2</p>
          </div>
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Interface</p>
            <p class="mt-3 text-2xl font-semibold text-white">Svelte 5</p>
          </div>
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Storage</p>
            <p class="mt-3 text-2xl font-semibold text-white">SQLite</p>
          </div>
        </div>
      </div>

      <div class="flex items-center p-8 lg:p-12">
        <div class="w-full rounded-[32px] border border-cyan-300/15 bg-white/5 p-8">
          <p class="text-sm uppercase tracking-[0.35em] text-cyan-200/70">Access Portal</p>
          <h2 class="mt-4 text-3xl font-semibold text-white">Local Data Manager</h2>
          <p class="mt-3 text-sm text-slate-400">
            No real authentication is implemented. This button simply demonstrates the app flow for
            presentation and prototyping.
          </p>

          <div class="mt-8 space-y-4">
            <div class="rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-slate-500">
              designer@portfolio.local
            </div>
            <div class="rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-slate-500">
              ••••••••••••••
            </div>
          </div>

          <button
            on:click={onContinue}
            class="mt-8 inline-flex w-full items-center justify-center rounded-2xl bg-gradient-to-r from-teal-400 via-cyan-400 to-sky-500 px-5 py-4 text-sm font-semibold text-slate-950 transition duration-300 hover:scale-[1.01] hover:shadow-glow"
          >
            Enter dashboard
          </button>
        </div>
      </div>
    </div>
  </div>
</section>
