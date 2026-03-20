<script lang="ts">
  let preferences = [
    {
      id: 'compact-density',
      name: 'Compact density',
      detail: 'Tighten row spacing for higher information density.',
      enabled: true
    },
    {
      id: 'desktop-notifications',
      name: 'Desktop notifications',
      detail: 'Show local toasts for sync, review, and archive events.',
      enabled: false
    },
    {
      id: 'auto-focus-drawer',
      name: 'Auto-focus drawer',
      detail: 'Open the inspector panel automatically after selecting a record.',
      enabled: true
    }
  ];

  function togglePreference(id: string) {
    preferences = preferences.map((preference) =>
      preference.id === id
        ? { ...preference, enabled: !preference.enabled }
        : preference
    );
  }
</script>

<section class="grid gap-4 xl:grid-cols-[1.1fr_0.9fr]">
  <article class="rounded-[32px] border border-white/10 bg-slate-950/65 p-6 shadow-glow backdrop-blur-xl">
    <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Preferences</p>
    <h3 class="mt-2 text-2xl font-semibold text-white">Workspace settings</h3>
    <p class="mt-2 max-w-2xl text-sm text-slate-400">
      This mock settings view rounds out the portfolio concept with believable controls and a
      stronger sense of application depth.
    </p>

    <div class="mt-6 space-y-4">
      {#each preferences as preference}
        <div class="flex items-center justify-between gap-4 rounded-[28px] border border-white/10 bg-white/[0.03] px-5 py-4">
          <div>
            <p class="text-sm font-medium text-white">{preference.name}</p>
            <p class="mt-1 text-sm text-slate-400">{preference.detail}</p>
          </div>
          <button
            type="button"
            aria-pressed={preference.enabled}
            on:click={() => togglePreference(preference.id)}
            class={`relative h-7 w-12 rounded-full border transition ${
              preference.enabled
                ? 'border-cyan-300/30 bg-cyan-400/20'
                : 'border-white/10 bg-slate-900'
            }`}
          >
            <span class={`absolute top-1 h-5 w-5 rounded-full bg-white transition ${
              preference.enabled ? 'left-6' : 'left-1'
            }`}></span>
          </button>
        </div>
      {/each}
    </div>
  </article>

  <article class="grid gap-4 rounded-[32px] border border-white/10 bg-white/[0.04] p-5 shadow-glow backdrop-blur-xl">
    <div class="rounded-[28px] border border-cyan-400/15 bg-cyan-400/10 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-cyan-100/70">Environment</p>
      <p class="mt-2 text-3xl font-semibold text-white">Local-only mode</p>
      <p class="mt-3 text-sm text-cyan-50/80">
        No real remote sync or encryption is active here. The panel exists to sell the desktop app
        concept and interaction quality.
      </p>
    </div>

    <div class="rounded-[28px] border border-white/10 bg-slate-950/65 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Storage</p>
      <div class="mt-4 space-y-3 text-sm text-slate-300">
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Driver: `rusqlite`</div>
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Mode: local prototype database</div>
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">IPC: Tauri command bridge</div>
      </div>
    </div>
  </article>
</section>
