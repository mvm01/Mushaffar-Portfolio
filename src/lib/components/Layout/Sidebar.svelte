<script lang="ts">
  import type { AppView, UserRecord } from '../../stores/appState';

  export let records: UserRecord[] = [];
  export let activeView: AppView = 'Overview';
  export let onNavigate: (view: AppView) => void = () => {};

  $: sections = [
    { name: 'Overview' as AppView, badge: 'Live' },
    { name: 'Records' as AppView, badge: String(records.length || 50) },
    { name: 'Analytics' as AppView, badge: 'New' },
    { name: 'Audit Trail' as AppView, badge: 'Logs' },
    { name: 'Settings' as AppView, badge: null }
  ];

  const statusCount = (status: UserRecord['status']) =>
    records.filter((record) => record.status === status).length;
</script>

<aside
  class="flex w-full max-w-xs flex-col gap-6 rounded-[28px] border border-white/10 bg-white/5 p-6 shadow-glow backdrop-blur-xl"
>
  <div class="space-y-2">
    <p class="text-xs uppercase tracking-[0.35em] text-cyan-200/60">Portfolio Prototype</p>
    <h1 class="text-2xl font-semibold text-white">Local Data Manager</h1>
    <p class="text-sm text-slate-400">
      Desktop-first dashboard concept built with Tauri, Svelte, and Tailwind.
    </p>
  </div>

  <nav class="space-y-3">
    {#each sections as section}
      <button
        on:click={() => onNavigate(section.name)}
        class={`flex w-full items-center justify-between rounded-2xl border px-4 py-3 text-left transition duration-300 ${
          activeView === section.name
            ? 'border-cyan-300/30 bg-cyan-400/10 text-white'
            : 'border-white/5 bg-slate-950/40 text-slate-300 hover:border-white/15 hover:bg-white/5'
        }`}
      >
        <span>{section.name}</span>
        {#if section.badge}
          <span class="rounded-full bg-white/10 px-2.5 py-1 text-xs text-slate-300">{section.badge}</span>
        {/if}
      </button>
    {/each}
  </nav>

  <div class="mt-auto grid grid-cols-3 gap-3">
    <div class="rounded-2xl border border-emerald-400/15 bg-emerald-400/10 p-3">
      <p class="text-xs uppercase text-emerald-200/70">Active</p>
      <p class="mt-2 text-xl font-semibold text-white">{statusCount('Active')}</p>
    </div>
    <div class="rounded-2xl border border-amber-400/15 bg-amber-400/10 p-3">
      <p class="text-xs uppercase text-amber-200/70">Pending</p>
      <p class="mt-2 text-xl font-semibold text-white">{statusCount('Pending')}</p>
    </div>
    <div class="rounded-2xl border border-slate-400/15 bg-slate-400/10 p-3">
      <p class="text-xs uppercase text-slate-200/70">Archived</p>
      <p class="mt-2 text-xl font-semibold text-white">{statusCount('Archived')}</p>
    </div>
  </div>
</aside>
