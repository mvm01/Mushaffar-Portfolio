<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import type { UserRecord } from '../../stores/appState';

  export let record: UserRecord | null = null;

  $: timeline = record
    ? [
        {
          title: 'Profile imported',
          detail: `${record.name} was staged into the local catalog.`,
          time: record.date,
          tone: 'bg-cyan-400'
        },
        {
          title: 'Integrity scan completed',
          detail:
            record.status === 'Pending'
              ? 'A follow-up review is recommended before promotion.'
              : 'Profile metadata passed the mock validation routine.',
          time: 'Today, 09:14',
          tone: record.status === 'Pending' ? 'bg-amber-400' : 'bg-emerald-400'
        },
        {
          title: 'Desktop preview refreshed',
          detail: 'UI state synchronized with the selected record drawer.',
          time: 'Today, 09:18',
          tone: 'bg-sky-400'
        }
      ]
    : [];
</script>

<section
  class="rounded-[32px] border border-white/10 bg-slate-950/65 p-5 shadow-glow backdrop-blur-xl"
  in:fade={{ duration: 220 }}
>
  <div class="flex items-center justify-between gap-4">
    <div>
      <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Audit Trail</p>
      <h3 class="mt-2 text-xl font-semibold text-white">Recent timeline</h3>
    </div>
    {#if record}
      <span class="rounded-full border border-white/10 bg-white/[0.04] px-3 py-1 text-xs uppercase tracking-[0.24em] text-slate-300">
        {record.id}
      </span>
    {/if}
  </div>

  {#if record}
    <div class="mt-6 space-y-4">
      {#each timeline as item, index}
        <div class="flex gap-4" in:fly={{ y: 12, duration: 180, delay: index * 35 }}>
          <div class="flex flex-col items-center">
            <span class={`mt-1 h-3 w-3 rounded-full ${item.tone}`}></span>
            {#if index !== timeline.length - 1}
              <span class="mt-2 h-full w-px bg-white/10"></span>
            {/if}
          </div>
          <div class="flex-1 rounded-2xl border border-white/10 bg-white/[0.03] px-4 py-4">
            <div class="flex items-start justify-between gap-3">
              <div>
                <p class="text-sm font-medium text-white">{item.title}</p>
                <p class="mt-2 text-sm text-slate-400">{item.detail}</p>
              </div>
              <p class="text-xs uppercase tracking-[0.24em] text-slate-500">{item.time}</p>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="mt-6 rounded-[28px] border border-dashed border-white/10 bg-white/[0.02] px-5 py-8 text-sm text-slate-400">
      Select a record to populate the timeline with contextual audit events.
    </div>
  {/if}
</section>
