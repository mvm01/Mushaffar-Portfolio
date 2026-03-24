<script lang="ts">
  export let operatorEmail = '';
  export let hasApiToken = false;
  export let localApi:
    | {
        enabled: boolean;
        host: string;
        port: number;
        base_url: string;
        auth_header: string;
        token_hint: string | null;
        key_source: string;
      }
    | null = null;
  export let generatedLocalApiKey: string | null = null;
  export let loading = false;
  export let syncing = false;
  export let onSaveApiToken: (token: string) => Promise<void> | void = () => {};
  export let onSyncCustomers: () => Promise<void> | void = () => {};
  export let onGenerateLocalApiKey: () => Promise<void> | void = () => {};
  export let onRevokeLocalApiKey: () => Promise<void> | void = () => {};
  export let onDismissGeneratedLocalApiKey: () => void = () => {};

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
  let apiToken = '';
  let copiedLabel: 'customers' | 'sync' | null = null;

  function togglePreference(id: string) {
    preferences = preferences.map((preference) =>
      preference.id === id
        ? { ...preference, enabled: !preference.enabled }
        : preference
    );
  }

  $: localApiBaseUrl = localApi?.base_url ?? 'http://127.0.0.1:18432';
  $: activeApiKey = generatedLocalApiKey ?? '<generate-a-new-key-to-reveal-it>';
  $: curlCustomersExample = `curl.exe -H "Authorization: Bearer ${activeApiKey}" ${localApiBaseUrl}/v1/customers`;
  $: curlSyncExample = `curl.exe -X POST -H "Authorization: Bearer ${activeApiKey}" ${localApiBaseUrl}/v1/customers/sync`;

  async function copyExample(kind: 'customers' | 'sync') {
    const value = kind === 'customers' ? curlCustomersExample : curlSyncExample;
    await navigator.clipboard.writeText(value);
    copiedLabel = kind;
    setTimeout(() => {
      if (copiedLabel === kind) copiedLabel = null;
    }, 1800);
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
            aria-label={`Toggle ${preference.name}`}
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

    <div class="mt-6 rounded-[28px] border border-white/10 bg-white/[0.03] p-5">
      <p class="text-xs uppercase tracking-[0.24em] text-slate-500">CRM Token</p>
      <p class="mt-2 text-sm text-slate-400">
        Operator: <span class="text-white">{operatorEmail || 'Not configured'}</span>
      </p>
      <p class="mt-2 text-sm text-slate-400">
        Token status:
        <span class={hasApiToken ? 'text-emerald-300' : 'text-amber-300'}>
          {hasApiToken ? ' Stored securely' : ' Missing'}
        </span>
      </p>

      <textarea
        bind:value={apiToken}
        class="mt-4 min-h-24 w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
        placeholder="Paste a replacement external API token"
      ></textarea>

      <div class="mt-4 flex flex-col gap-3 sm:flex-row">
        <button
          disabled={loading || !apiToken.trim()}
          on:click={() => onSaveApiToken(apiToken)}
          class="inline-flex items-center justify-center rounded-2xl border border-cyan-300/20 bg-cyan-400/10 px-4 py-3 text-sm font-medium text-cyan-100 transition hover:border-cyan-300/30 hover:bg-cyan-400/15 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {loading ? 'Saving...' : 'Save token'}
        </button>
        <button
          disabled={syncing || !hasApiToken}
          on:click={onSyncCustomers}
          class="inline-flex items-center justify-center rounded-2xl border border-emerald-300/20 bg-emerald-400/10 px-4 py-3 text-sm font-medium text-emerald-100 transition hover:border-emerald-300/30 hover:bg-emerald-400/15 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {syncing ? 'Syncing customers...' : 'Sync customers now'}
        </button>
      </div>
    </div>
  </article>

  <article class="grid gap-4 rounded-[32px] border border-white/10 bg-white/[0.04] p-5 shadow-glow backdrop-blur-xl">
    <div class="rounded-[28px] border border-cyan-400/15 bg-cyan-400/10 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-cyan-100/70">Local API</p>
      <p class="mt-2 text-3xl font-semibold text-white">
        {localApi?.enabled ? 'Loopback integration enabled' : 'Loopback integration disabled'}
      </p>
      <p class="mt-3 text-sm text-cyan-50/80">
        The desktop app can expose a bearer-token protected API on localhost so your other apps can
        read local customer data or trigger sync without touching the webview directly.
      </p>

      <div class="mt-4 space-y-3 text-sm text-cyan-50/90">
        <div class="rounded-2xl border border-cyan-300/15 bg-slate-950/30 px-4 py-3">
          Base URL: <span class="font-mono text-white">{localApi?.base_url ?? 'Unavailable'}</span>
        </div>
        <div class="rounded-2xl border border-cyan-300/15 bg-slate-950/30 px-4 py-3">
          Auth: <span class="font-mono text-white">{localApi?.auth_header ?? 'Authorization: Bearer <token>'}</span>
        </div>
        <div class="rounded-2xl border border-cyan-300/15 bg-slate-950/30 px-4 py-3">
          Token source:
          <span class="font-mono text-white">{localApi?.key_source ?? 'LDM_LOCAL_API_KEY'}</span>
          {#if localApi?.token_hint}
            <span class="ml-2 text-cyan-100/70">({localApi.token_hint})</span>
          {/if}
        </div>
      </div>

      <div class="mt-4 flex flex-col gap-3 sm:flex-row">
        <button
          type="button"
          disabled={loading}
          on:click={onGenerateLocalApiKey}
          class="inline-flex items-center justify-center rounded-2xl border border-cyan-300/20 bg-cyan-400/10 px-4 py-3 text-sm font-medium text-cyan-100 transition hover:border-cyan-300/30 hover:bg-cyan-400/15 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {loading ? 'Generating...' : localApi?.enabled ? 'Rotate local API key' : 'Generate local API key'}
        </button>
        <button
          type="button"
          disabled={loading || !localApi?.enabled}
          on:click={onRevokeLocalApiKey}
          class="inline-flex items-center justify-center rounded-2xl border border-rose-300/20 bg-rose-400/10 px-4 py-3 text-sm font-medium text-rose-100 transition hover:border-rose-300/30 hover:bg-rose-400/15 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {loading ? 'Working...' : 'Revoke local API key'}
        </button>
      </div>

      {#if generatedLocalApiKey}
        <div class="mt-4 rounded-[24px] border border-amber-300/20 bg-amber-300/10 p-4 text-sm text-amber-50">
          <p class="font-semibold">New local API key</p>
          <p class="mt-2 text-amber-100/80">
            This full key is shown once after generation. Save it in the app or service that will connect to this desktop workspace.
          </p>
          <p class="mt-3 break-all rounded-2xl border border-amber-300/20 bg-slate-950/40 px-4 py-3 font-mono text-xs text-white">
            {generatedLocalApiKey}
          </p>
          <button
            type="button"
            on:click={onDismissGeneratedLocalApiKey}
            class="mt-3 inline-flex items-center justify-center rounded-2xl border border-amber-300/20 bg-amber-300/10 px-4 py-2 text-sm font-medium text-amber-50 transition hover:border-amber-200/40 hover:bg-amber-300/15"
          >
            I saved it
          </button>
        </div>
      {/if}
    </div>

    <div class="rounded-[28px] border border-white/10 bg-slate-950/65 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Storage</p>
      <div class="mt-4 space-y-3 text-sm text-slate-300">
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Driver: `rusqlite`</div>
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Mode: SQLCipher-encrypted local workspace</div>
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">IPC: Tauri command bridge</div>
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] px-4 py-3">Routes: `/health`, `/v1/operator`, `/v1/customers`, `/v1/customers/sync`</div>
      </div>
    </div>

    <div class="rounded-[28px] border border-white/10 bg-slate-950/65 p-5">
      <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Curl Examples</p>
      <p class="mt-2 text-sm text-slate-400">
        Use these commands from another app or terminal. Generate or rotate a key first if you want the full bearer token to appear here.
      </p>

      <div class="mt-4 space-y-4">
        <div class="rounded-2xl border border-white/8 bg-white/[0.03] p-4">
          <div class="flex items-center justify-between gap-3">
            <p class="text-sm font-medium text-white">List customers</p>
            <button
              type="button"
              on:click={() => copyExample('customers')}
              class="inline-flex items-center justify-center rounded-xl border border-white/10 bg-white/[0.04] px-3 py-2 text-xs font-medium text-slate-200 transition hover:border-white/20 hover:bg-white/[0.08]"
            >
              {copiedLabel === 'customers' ? 'Copied' : 'Copy'}
            </button>
          </div>
          <pre class="mt-3 overflow-x-auto rounded-2xl border border-white/8 bg-slate-950/70 px-4 py-3 text-xs text-cyan-100"><code>{curlCustomersExample}</code></pre>
        </div>

        <div class="rounded-2xl border border-white/8 bg-white/[0.03] p-4">
          <div class="flex items-center justify-between gap-3">
            <p class="text-sm font-medium text-white">Trigger sync</p>
            <button
              type="button"
              on:click={() => copyExample('sync')}
              class="inline-flex items-center justify-center rounded-xl border border-white/10 bg-white/[0.04] px-3 py-2 text-xs font-medium text-slate-200 transition hover:border-white/20 hover:bg-white/[0.08]"
            >
              {copiedLabel === 'sync' ? 'Copied' : 'Copy'}
            </button>
          </div>
          <pre class="mt-3 overflow-x-auto rounded-2xl border border-white/8 bg-slate-950/70 px-4 py-3 text-xs text-cyan-100"><code>{curlSyncExample}</code></pre>
        </div>
      </div>
    </div>
  </article>
</section>
