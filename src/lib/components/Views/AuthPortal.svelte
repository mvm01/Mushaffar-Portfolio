<script lang="ts">
  export let loading = false;
  export let hasOperator = false;
  export let operatorEmail = '';
  export let error: string | null = null;
  export let recoveryKey: string | null = null;
  export let onSetup: (email: string, password: string, apiToken: string) => Promise<void> | void =
    () => {};
  export let onUnlock: (email: string, password: string) => Promise<void> | void = () => {};
  export let onResetPassword: (
    email: string,
    recoveryKey: string,
    newPassword: string
  ) => Promise<void> | void = () => {};

  let setupEmail = operatorEmail;
  let setupPassword = '';
  let setupApiToken = '';
  let unlockEmail = operatorEmail;
  let unlockPassword = '';
  let showRecovery = false;
  let recoveryEmail = operatorEmail;
  let recoveryCode = '';
  let recoveryPassword = '';

  async function handleUnlock() {
    await onUnlock(unlockEmail, unlockPassword);
  }

  async function handleSetup() {
    await onSetup(setupEmail, setupPassword, setupApiToken);
  }

  async function handleRecoveryReset() {
    await onResetPassword(recoveryEmail, recoveryCode, recoveryPassword);
    unlockEmail = recoveryEmail;
    unlockPassword = '';
    recoveryCode = '';
    recoveryPassword = '';
    showRecovery = false;
  }
</script>

<section class="relative flex min-h-screen items-center justify-center overflow-hidden px-6 py-10">
  <div class="absolute inset-0 bg-mesh-gradient"></div>
  <div class="absolute left-1/2 top-10 h-72 w-72 -translate-x-1/2 rounded-full bg-cyan-400/10 blur-3xl"></div>

  <div class="relative w-full max-w-6xl overflow-hidden rounded-[36px] border border-white/10 bg-slate-950/65 shadow-glow backdrop-blur-2xl">
    <div class="grid min-h-[720px] lg:grid-cols-[1.05fr_0.95fr]">
      <div class="flex flex-col justify-between border-b border-white/10 p-8 lg:border-b-0 lg:border-r lg:p-12">
        <div class="space-y-6">
          <p class="text-sm uppercase tracking-[0.35em] text-cyan-200/70">Secure Local Workspace</p>
          <h1 class="max-w-lg text-5xl font-semibold leading-tight text-white">
            Protect operators, encrypt API access, and sync CRM data from a desktop dashboard.
          </h1>
          <p class="max-w-xl text-base text-slate-300">
            The local operator unlocks the workspace with a master password, while the external CRM
            token is stored encrypted and used only for customer sync.
          </p>
        </div>

        <div class="grid gap-4 md:grid-cols-3">
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Auth</p>
            <p class="mt-3 text-2xl font-semibold text-white">Argon2</p>
          </div>
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Tokens</p>
            <p class="mt-3 text-2xl font-semibold text-white">Encrypted</p>
          </div>
          <div class="rounded-3xl border border-white/10 bg-white/5 p-5">
            <p class="text-xs uppercase tracking-[0.25em] text-slate-400">Storage</p>
            <p class="mt-3 text-2xl font-semibold text-white">SQLite</p>
          </div>
        </div>
      </div>

      <div class="flex items-center p-8 lg:p-12">
        <div class="w-full rounded-[32px] border border-cyan-300/15 bg-white/5 p-8">
          <p class="text-sm uppercase tracking-[0.35em] text-cyan-200/70">
            {hasOperator ? 'Unlock Workspace' : 'Initial Operator Setup'}
          </p>
          <h2 class="mt-4 text-3xl font-semibold text-white">Local Data Manager</h2>
          <p class="mt-3 text-sm text-slate-400">
            {#if hasOperator}
              Unlock the operator workspace to access encrypted sync credentials and cached customer records.
            {:else}
              Create the first operator, define a master password, and store the external CRM token securely.
            {/if}
          </p>

          {#if error}
            <div class="mt-6 rounded-2xl border border-rose-400/20 bg-rose-400/10 px-4 py-3 text-sm text-rose-200">
              {error}
            </div>
          {/if}

          {#if recoveryKey}
            <div class="mt-6 rounded-2xl border border-amber-400/20 bg-amber-400/10 px-4 py-4 text-sm text-amber-100">
              <p class="font-semibold">Save this recovery key now.</p>
              <p class="mt-2 break-all font-mono text-amber-50">{recoveryKey}</p>
              <p class="mt-2 text-amber-100/80">
                It is only shown once. You can use it later to reset the master password.
              </p>
            </div>
          {/if}

          {#if hasOperator}
            {#if !showRecovery}
              <form class="mt-8 space-y-4" on:submit|preventDefault={handleUnlock}>
                <input
                  bind:value={unlockEmail}
                  class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                  placeholder="operator@company.com"
                  type="email"
                />
                <input
                  bind:value={unlockPassword}
                  class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                  placeholder="Master password"
                  type="password"
                />
                <button
                  disabled={loading}
                  class="mt-2 inline-flex w-full items-center justify-center rounded-2xl bg-gradient-to-r from-teal-400 via-cyan-400 to-sky-500 px-5 py-4 text-sm font-semibold text-slate-950 transition duration-300 hover:scale-[1.01] hover:shadow-glow disabled:cursor-not-allowed disabled:opacity-70"
                >
                  {loading ? 'Unlocking...' : 'Unlock dashboard'}
                </button>
              </form>

              <button
                class="mt-4 text-sm text-cyan-200 transition hover:text-white"
                on:click={() => (showRecovery = true)}
              >
                Forgot password?
              </button>
            {:else}
              <form
                class="mt-8 space-y-4"
                on:submit|preventDefault={handleRecoveryReset}
              >
                <input
                  bind:value={recoveryEmail}
                  class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                  placeholder="operator@company.com"
                  type="email"
                />
                <input
                  bind:value={recoveryCode}
                  class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                  placeholder="Recovery key"
                  type="text"
                />
                <input
                  bind:value={recoveryPassword}
                  class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                  placeholder="New master password"
                  type="password"
                />
                <button
                  disabled={loading}
                  class="mt-2 inline-flex w-full items-center justify-center rounded-2xl bg-gradient-to-r from-teal-400 via-cyan-400 to-sky-500 px-5 py-4 text-sm font-semibold text-slate-950 transition duration-300 hover:scale-[1.01] hover:shadow-glow disabled:cursor-not-allowed disabled:opacity-70"
                >
                  {loading ? 'Resetting password...' : 'Reset password'}
                </button>
              </form>

              <button
                class="mt-4 text-sm text-cyan-200 transition hover:text-white"
                on:click={() => (showRecovery = false)}
              >
                Back to unlock
              </button>
            {/if}
          {:else}
            <form
              class="mt-8 space-y-4"
              on:submit|preventDefault={handleSetup}
            >
              <input
                bind:value={setupEmail}
                class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                placeholder="operator@company.com"
                type="email"
              />
              <input
                bind:value={setupPassword}
                class="w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                placeholder="Master password (10+ characters)"
                type="password"
              />
              <textarea
                bind:value={setupApiToken}
                class="min-h-28 w-full rounded-2xl border border-white/10 bg-slate-950/60 px-4 py-3 text-sm text-white outline-none placeholder:text-slate-500"
                placeholder="External CRM Bearer token"
              ></textarea>
              <button
                disabled={loading}
                class="mt-2 inline-flex w-full items-center justify-center rounded-2xl bg-gradient-to-r from-teal-400 via-cyan-400 to-sky-500 px-5 py-4 text-sm font-semibold text-slate-950 transition duration-300 hover:scale-[1.01] hover:shadow-glow disabled:cursor-not-allowed disabled:opacity-70"
              >
                {loading ? 'Creating operator...' : 'Create secure workspace'}
              </button>
            </form>
          {/if}
        </div>
      </div>
    </div>
  </div>
</section>
