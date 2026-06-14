<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import type { UserRecord } from '../../stores/appState';

  export let record: UserRecord | null = null;
  export let visible = false;
  export let onClose: () => void = () => {};

  $: profileStrength =
    record?.status === 'Active' ? 'High integrity' : record?.status === 'Pending' ? 'Needs review' : 'Cold storage';

  $: statusTone =
    record?.status === 'Active'
      ? 'border-emerald-400/20 bg-emerald-400/10 text-emerald-300'
      : record?.status === 'Pending'
        ? 'border-amber-400/20 bg-amber-400/10 text-amber-300'
        : 'border-slate-400/20 bg-slate-400/10 text-slate-300';
</script>

{#if visible && record}
  <aside
    class="rounded-[32px] border border-white/10 bg-slate-950/70 p-5 shadow-glow backdrop-blur-xl"
    in:fly={{ x: 20, duration: 220 }}
    out:fade={{ duration: 180 }}
  >
    <div class="flex items-start justify-between gap-4">
      <div>
        <p class="text-xs uppercase tracking-[0.28em] text-cyan-200/60">Record Drawer</p>
        <h3 class="mt-2 text-2xl font-semibold text-white">{record.name}</h3>
        <p class="mt-2 text-sm text-slate-400">{record.email}</p>
      </div>
      <button
        on:click={onClose}
        class="rounded-full border border-white/10 bg-white/[0.04] px-3 py-2 text-xs uppercase tracking-[0.24em] text-slate-300 transition hover:border-white/20 hover:bg-white/[0.08]"
      >
        Close
      </button>
    </div>

    <div class="mt-6 grid gap-4 sm:grid-cols-2">
      <div class="rounded-[28px] border border-white/10 bg-white/[0.04] p-4">
        <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Profile ID</p>
        <p class="mt-3 text-2xl font-semibold text-white">UID-{record.id.toString().padStart(3, '0')}</p>
      </div>
      <div class="rounded-[28px] border border-white/10 bg-white/[0.04] p-4">
        <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Status</p>
        <div class={`mt-3 inline-flex rounded-full px-3 py-1 text-xs font-medium ${statusTone}`}>
          {record.status}
        </div>
      </div>
    </div>

    <div class="mt-4 rounded-[28px] border border-cyan-400/15 bg-gradient-to-br from-cyan-400/10 to-sky-500/10 p-5">
      <p class="text-xs uppercase tracking-[0.24em] text-cyan-100/70">Data Health</p>
      <p class="mt-3 text-3xl font-semibold text-white">{profileStrength}</p>
      <p class="mt-3 text-sm text-cyan-50/80">
        This mock side panel adds richer application storytelling by presenting contextual metadata,
        action affordances, and a sense of focused record review.
      </p>
    </div>

    <div class="mt-4 space-y-3">
      <div class="rounded-2xl border border-white/10 bg-white/[0.03] px-4 py-3">
        <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Created</p>
        <p class="mt-2 text-sm text-slate-200">{record.date}</p>
      </div>
      <div class="rounded-2xl border border-white/10 bg-white/[0.03] px-4 py-3">
        <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Workspace Route</p>
        <p class="mt-2 text-sm text-slate-200">Local Vault / Profiles / {record.id}</p>
      </div>
      <div class="rounded-2xl border border-white/10 bg-white/[0.03] px-4 py-3">
        <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Suggested Action</p>
        <p class="mt-2 text-sm text-slate-200">
          {record.status === 'Pending'
            ? 'Review and mark ready for sync'
            : record.status === 'Archived'
              ? 'Restore to active workspace if needed'
              : 'Maintain and monitor for changes'}
        </p>
      </div>
    </div>
  </aside>
{/if}
