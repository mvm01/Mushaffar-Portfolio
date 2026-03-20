<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import type { UserRecord } from '../../stores/appState';

  export let records: UserRecord[] = [];

  const monthlyLabels = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul'];
  const barHeights = [38, 52, 44, 68, 61, 82, 74];

  $: total = records.length;
  $: activeCount = records.filter((record) => record.status === 'Active').length;
  $: pendingCount = records.filter((record) => record.status === 'Pending').length;
  $: archivedCount = records.filter((record) => record.status === 'Archived').length;
  $: activePercent = total ? Math.round((activeCount / total) * 100) : 0;
  $: pendingPercent = total ? Math.round((pendingCount / total) * 100) : 0;
  $: archivedPercent = total ? Math.round((archivedCount / total) * 100) : 0;
  $: recentRecords = [...records].slice(0, 4);
</script>

<section class="grid gap-4 xl:grid-cols-[1.45fr_0.95fr]">
  <article
    class="overflow-hidden rounded-[32px] border border-white/10 bg-slate-950/65 p-6 shadow-glow backdrop-blur-xl"
    in:fade={{ duration: 220 }}
  >
    <div class="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
      <div>
        <p class="text-sm uppercase tracking-[0.3em] text-cyan-200/60">Insights</p>
        <h3 class="mt-2 text-2xl font-semibold text-white">Record activity trend</h3>
        <p class="mt-2 max-w-xl text-sm text-slate-400">
          A visual-only analytics section for the portfolio presentation, designed to suggest healthy
          throughput and a premium desktop monitoring experience.
        </p>
      </div>

      <div class="grid w-full max-w-md grid-cols-1 gap-3 sm:grid-cols-3 xl:w-[340px] xl:grid-cols-1 2xl:grid-cols-3">
        <div class="min-w-0 rounded-2xl border border-white/10 bg-white/[0.04] px-4 py-3">
          <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Growth</p>
          <p class="mt-2 text-2xl font-semibold text-white">+18%</p>
        </div>
        <div class="min-w-0 rounded-2xl border border-white/10 bg-white/[0.04] px-4 py-3">
          <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Latency</p>
          <p class="mt-2 text-2xl font-semibold text-white">12ms</p>
        </div>
        <div class="min-w-0 rounded-2xl border border-cyan-400/20 bg-cyan-400/10 px-4 py-3 sm:col-span-1">
          <p class="text-xs uppercase tracking-[0.24em] text-cyan-100/70">Signal</p>
          <p class="mt-2 text-2xl font-semibold text-white">Stable</p>
        </div>
      </div>
    </div>

    <div class="mt-8 grid gap-4 lg:grid-cols-[1.15fr_0.85fr]">
      <div class="rounded-[28px] border border-white/10 bg-white/[0.03] p-5">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Volume Snapshot</p>
            <p class="mt-2 text-3xl font-semibold text-white">{total}</p>
          </div>
          <div class="rounded-full border border-emerald-400/20 bg-emerald-400/10 px-3 py-1 text-xs uppercase tracking-[0.24em] text-emerald-300">
            +6.4%
          </div>
        </div>

        <div class="mt-8 flex h-64 items-end gap-3">
          {#each monthlyLabels as label, index}
            <div class="flex flex-1 flex-col items-center gap-3" in:fly={{ y: 16, duration: 220, delay: index * 40 }}>
              <div class="relative flex h-52 w-full items-end overflow-hidden rounded-[22px] border border-white/8 bg-slate-950/80 px-2 py-2">
                <div
                  class="w-full rounded-[16px] bg-gradient-to-t from-teal-400 via-cyan-400 to-sky-400 shadow-[0_0_25px_rgba(34,211,238,0.2)]"
                  style={`height: ${barHeights[index]}%;`}
                ></div>
                <div class="absolute inset-x-2 top-2 h-px bg-white/5"></div>
                <div class="absolute inset-x-2 top-1/2 h-px bg-white/5"></div>
              </div>
              <span class="text-xs uppercase tracking-[0.24em] text-slate-500">{label}</span>
            </div>
          {/each}
        </div>
      </div>

      <div class="space-y-4">
        <div class="rounded-[28px] border border-white/10 bg-white/[0.03] p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Status Mix</p>
              <h4 class="mt-2 text-xl font-semibold text-white">Distribution</h4>
            </div>
            <div class="text-right">
              <p class="text-xs uppercase tracking-[0.24em] text-slate-500">Active Share</p>
              <p class="mt-2 text-2xl font-semibold text-white">{activePercent}%</p>
            </div>
          </div>

          <div class="mt-6 space-y-4">
            <div>
              <div class="mb-2 flex items-center justify-between text-sm">
                <span class="text-slate-300">Active</span>
                <span class="text-emerald-300">{activeCount}</span>
              </div>
              <div class="h-2 rounded-full bg-white/5">
                <div class="h-2 rounded-full bg-emerald-400" style={`width: ${activePercent}%;`}></div>
              </div>
            </div>
            <div>
              <div class="mb-2 flex items-center justify-between text-sm">
                <span class="text-slate-300">Pending</span>
                <span class="text-amber-300">{pendingCount}</span>
              </div>
              <div class="h-2 rounded-full bg-white/5">
                <div class="h-2 rounded-full bg-amber-400" style={`width: ${pendingPercent}%;`}></div>
              </div>
            </div>
            <div>
              <div class="mb-2 flex items-center justify-between text-sm">
                <span class="text-slate-300">Archived</span>
                <span class="text-slate-300">{archivedCount}</span>
              </div>
              <div class="h-2 rounded-full bg-white/5">
                <div class="h-2 rounded-full bg-slate-400" style={`width: ${archivedPercent}%;`}></div>
              </div>
            </div>
          </div>
        </div>

        <div class="rounded-[28px] border border-white/10 bg-gradient-to-br from-sky-500/10 via-transparent to-teal-400/10 p-5">
          <p class="text-xs uppercase tracking-[0.28em] text-cyan-200/60">Recent Activity</p>
          <div class="mt-4 space-y-3">
            {#each recentRecords as record, index (record.id)}
              <div
                class="flex items-center justify-between rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3"
                in:fly={{ y: 10, duration: 180, delay: index * 35 }}
              >
                <div>
                  <p class="text-sm font-medium text-white">{record.name}</p>
                  <p class="mt-1 text-xs uppercase tracking-[0.22em] text-slate-500">{record.date}</p>
                </div>
                <span
                  class={`inline-flex rounded-full px-3 py-1 text-xs font-medium ${
                    record.status === 'Active'
                      ? 'border border-emerald-400/20 bg-emerald-400/10 text-emerald-300'
                      : record.status === 'Pending'
                        ? 'border border-amber-400/20 bg-amber-400/10 text-amber-300'
                        : 'border border-slate-400/20 bg-slate-400/10 text-slate-300'
                  }`}
                >
                  {record.status}
                </span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
  </article>

  <article
    class="grid gap-4 rounded-[32px] border border-white/10 bg-white/[0.04] p-5 shadow-glow backdrop-blur-xl"
    in:fade={{ duration: 240 }}
  >
    <div class="rounded-[28px] border border-white/10 bg-slate-950/65 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Performance Layer</p>
      <h3 class="mt-2 text-xl font-semibold text-white">Desktop-ready feel</h3>
      <p class="mt-2 text-sm text-slate-400">
        The layout prioritizes scanability with spacious cards, soft glow accents, and responsive
        collapse behavior for narrower windows.
      </p>
    </div>

    <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-1">
      <div class="rounded-[28px] border border-cyan-400/15 bg-cyan-400/10 p-5">
        <p class="text-xs uppercase tracking-[0.28em] text-cyan-100/70">Session Mood</p>
        <p class="mt-2 text-3xl font-semibold text-white">Focused</p>
        <p class="mt-3 text-sm text-cyan-50/80">
          Designed to suggest calm control over local datasets without clutter or visual fatigue.
        </p>
      </div>

      <div class="rounded-[28px] border border-white/10 bg-slate-950/65 p-5">
        <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Highlights</p>
        <ul class="mt-4 space-y-3 text-sm text-slate-300">
          <li class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Searchable table with graceful mobile cards</li>
          <li class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Mock IPC flow from Rust into local Svelte state</li>
          <li class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Presentation-first analytics panel for portfolio storytelling</li>
        </ul>
      </div>
    </div>
  </article>
</section>
