<script lang="ts">
  export let eyebrow = 'Dashboard';
  export let title = 'Records at a glance';
  export let description =
    'Browse local records, monitor statuses, and highlight a polished desktop UI without exposing any real data flows.';
  export let query = '';
  export let onSearch: (value: string) => void = () => {};
</script>

<header class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
  <div>
    <p class="text-sm uppercase tracking-[0.3em] text-cyan-200/60">{eyebrow}</p>
    <h2 class="mt-2 text-3xl font-semibold text-white">{title}</h2>
    <p class="mt-2 max-w-2xl text-sm text-slate-400">{description}</p>
  </div>

  <label class="flex items-center gap-3 rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 shadow-glow">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      class="h-5 w-5 text-slate-400"
    >
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m21 21-4.35-4.35M10.5 18a7.5 7.5 0 1 1 0-15 7.5 7.5 0 0 1 0 15Z" />
    </svg>
    <input
      value={query}
      on:input={(event) => onSearch((event.currentTarget as HTMLInputElement).value)}
      class="w-full border-none bg-transparent text-sm text-white outline-none placeholder:text-slate-500 md:w-72"
      placeholder="Search records by name or email"
    />
  </label>
</header>
