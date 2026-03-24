<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import type { UserRecord } from '../../stores/appState';

  export let records: UserRecord[] = [];
  export let loading = false;
  export let selectedId: string | null = null;
  export let onInspect: (record: UserRecord) => void = () => {};
  export let pageSize = 25;

  const statusClasses: Record<UserRecord['status'], string> = {
    Active: 'bg-emerald-400/10 text-emerald-300 border border-emerald-400/20',
    Pending: 'bg-amber-400/10 text-amber-300 border border-amber-400/20',
    Archived: 'bg-slate-400/10 text-slate-300 border border-slate-400/20'
  };

  let currentPage = 1;
  let previousKey = '';

  $: recordsKey = records.map((record) => record.id).join(',');
  $: if (recordsKey !== previousKey) {
    previousKey = recordsKey;
    currentPage = 1;
  }

  $: totalPages = Math.max(1, Math.ceil(records.length / pageSize));
  $: currentPage = Math.min(currentPage, totalPages);
  $: paginatedRecords = records.slice((currentPage - 1) * pageSize, currentPage * pageSize);
  $: pageStart = records.length === 0 ? 0 : (currentPage - 1) * pageSize + 1;
  $: pageEnd = Math.min(currentPage * pageSize, records.length);

  function goToPage(page: number) {
    currentPage = Math.min(Math.max(page, 1), totalPages);
  }
</script>

<section
  class="overflow-hidden rounded-[28px] border border-white/10 bg-slate-950/55 shadow-glow backdrop-blur-xl"
  in:fade={{ duration: 220 }}
>
  <div class="flex flex-col gap-4 border-b border-white/10 px-6 py-5 lg:flex-row lg:items-center lg:justify-between">
    <div>
      <h3 class="text-xl font-semibold text-white">Profile Records</h3>
      <p class="mt-1 text-sm text-slate-400">
        Scrollable data grid optimized for desktop presentation and responsive collapse on smaller screens.
      </p>
    </div>
    <div class="inline-flex items-center gap-2 rounded-full border border-cyan-400/20 bg-cyan-400/10 px-3 py-1.5 text-xs uppercase tracking-[0.3em] text-cyan-200">
      <span class="h-2 w-2 rounded-full bg-cyan-300"></span>
      Local state only
    </div>
  </div>

  {#if loading}
    <div class="grid gap-3 p-6">
      {#each Array.from({ length: 8 }) as _, index}
        <div
          class="h-16 animate-pulse rounded-2xl border border-white/5 bg-white/[0.03]"
          in:fly={{ y: 12, duration: 180, delay: index * 20 }}
        ></div>
      {/each}
    </div>
  {:else}
    <div class="hidden overflow-x-auto lg:block">
      <table class="min-w-full border-collapse">
        <thead class="bg-white/[0.03] text-left text-xs uppercase tracking-[0.28em] text-slate-500">
          <tr>
            <th class="px-6 py-4 font-medium">Name</th>
            <th class="px-6 py-4 font-medium">Email</th>
            <th class="px-6 py-4 font-medium">Status</th>
            <th class="px-6 py-4 font-medium">Created</th>
            <th class="px-6 py-4 font-medium text-right">Action</th>
          </tr>
        </thead>
        <tbody>
          {#each paginatedRecords as record, index (record.id)}
            <tr
              class={`border-t border-white/5 text-sm text-slate-300 transition hover:bg-white/[0.03] ${
                selectedId === record.id ? 'bg-cyan-400/[0.06]' : ''
              }`}
              in:fly={{ y: 10, duration: 180, delay: index * 12 }}
            >
              <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                  <div class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-teal-400/25 to-sky-500/25 text-sm font-semibold text-cyan-100">
                    {record.name.slice(0, 1)}
                  </div>
                  <div>
                    <p class="font-medium text-white">{record.name}</p>
                    <p class="text-xs uppercase tracking-[0.24em] text-slate-500">{record.id}</p>
                  </div>
                </div>
              </td>
              <td class="px-6 py-4 text-slate-300">{record.email}</td>
              <td class="px-6 py-4">
                <span class={`inline-flex rounded-full px-3 py-1 text-xs font-medium ${statusClasses[record.status]}`}>
                  {record.status}
                </span>
              </td>
              <td class="px-6 py-4 text-slate-400">{record.date}</td>
              <td class="px-6 py-4 text-right">
                <button
                  on:click={() => onInspect(record)}
                  class={`rounded-full px-3 py-2 text-xs uppercase tracking-[0.24em] transition ${
                    selectedId === record.id
                      ? 'border border-cyan-300/30 bg-cyan-400/10 text-cyan-100'
                      : 'border border-white/10 bg-white/[0.04] text-slate-300 hover:border-white/20 hover:bg-white/[0.08]'
                  }`}
                >
                  Inspect
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <div class="grid gap-4 p-4 lg:hidden">
      {#each paginatedRecords as record, index (record.id)}
        <article
          class={`rounded-3xl border p-4 ${
            selectedId === record.id
              ? 'border-cyan-300/25 bg-cyan-400/[0.06]'
              : 'border-white/10 bg-white/[0.03]'
          }`}
          in:fly={{ y: 10, duration: 180, delay: index * 18 }}
        >
          <div class="flex items-start justify-between gap-4">
            <div>
              <h4 class="text-base font-semibold text-white">{record.name}</h4>
              <p class="mt-1 text-sm text-slate-400">{record.email}</p>
            </div>
            <span class={`inline-flex rounded-full px-3 py-1 text-xs font-medium ${statusClasses[record.status]}`}>
              {record.status}
            </span>
          </div>
          <p class="mt-4 text-xs uppercase tracking-[0.28em] text-slate-500">Created</p>
          <p class="mt-2 text-sm text-slate-300">{record.date}</p>
          <button
            on:click={() => onInspect(record)}
            class="mt-4 inline-flex rounded-full border border-white/10 bg-white/[0.04] px-3 py-2 text-xs uppercase tracking-[0.24em] text-slate-200 transition hover:border-white/20 hover:bg-white/[0.08]"
          >
            Inspect record
          </button>
        </article>
      {/each}
    </div>

    <div class="flex flex-col gap-4 border-t border-white/10 px-4 py-4 sm:px-6 lg:flex-row lg:items-center lg:justify-between">
      <div class="text-sm text-slate-400">
        Showing <span class="font-medium text-white">{pageStart}-{pageEnd}</span> of
        <span class="font-medium text-white">{records.length}</span> records
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <button
          on:click={() => goToPage(currentPage - 1)}
          disabled={currentPage === 1}
          class="rounded-full border border-white/10 bg-white/[0.04] px-3 py-2 text-xs uppercase tracking-[0.24em] text-slate-300 transition hover:border-white/20 hover:bg-white/[0.08] disabled:cursor-not-allowed disabled:opacity-40"
        >
          Prev
        </button>

        {#each Array.from({ length: totalPages }, (_, index) => index + 1) as page}
          <button
            on:click={() => goToPage(page)}
            class={`h-9 min-w-9 rounded-full border px-3 text-xs font-medium transition ${
              currentPage === page
                ? 'border-cyan-300/30 bg-cyan-400/10 text-cyan-100'
                : 'border-white/10 bg-white/[0.04] text-slate-300 hover:border-white/20 hover:bg-white/[0.08]'
            }`}
          >
            {page}
          </button>
        {/each}

        <button
          on:click={() => goToPage(currentPage + 1)}
          disabled={currentPage === totalPages}
          class="rounded-full border border-white/10 bg-white/[0.04] px-3 py-2 text-xs uppercase tracking-[0.24em] text-slate-300 transition hover:border-white/20 hover:bg-white/[0.08] disabled:cursor-not-allowed disabled:opacity-40"
        >
          Next
        </button>
      </div>
    </div>
  {/if}
</section>
