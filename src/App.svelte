<script lang="ts">
  import { onMount } from 'svelte';
  import Header from './lib/components/Layout/Header.svelte';
  import Sidebar from './lib/components/Layout/Sidebar.svelte';
  import AnalyticsPanel from './lib/components/Views/AnalyticsPanel.svelte';
  import AuditTimeline from './lib/components/Views/AuditTimeline.svelte';
  import DataGrid from './lib/components/Views/DataGrid.svelte';
  import DetailsDrawer from './lib/components/Views/DetailsDrawer.svelte';
  import MockLogin from './lib/components/Views/MockLogin.svelte';
  import SettingsPanel from './lib/components/Views/SettingsPanel.svelte';
  import type { AppView, UserRecord } from './lib/stores/appState';
  import { appState } from './lib/stores/appState';

  let selectedRecord: UserRecord | null = null;
  let drawerVisible = false;

  $: state = $appState;
  $: filteredRecords = (() => {
    const query = state.query.trim().toLowerCase();

    if (!query) {
      return state.records;
    }

    return state.records.filter((record) =>
      [record.name, record.email, record.status].some((value) =>
        value.toLowerCase().includes(query)
      )
    );
  })();

  onMount(() => {
    if (state.isAuthenticated && state.records.length === 0) {
      void appState.loadRecords();
    }
  });

  $: if (filteredRecords.length === 0) {
    selectedRecord = null;
    drawerVisible = false;
  } else if (!selectedRecord || !filteredRecords.some((record) => record.id === selectedRecord?.id)) {
    selectedRecord = filteredRecords[0];
    drawerVisible = true;
  }

  async function enterDashboard() {
    appState.login();
    await appState.loadRecords();
  }

  function inspectRecord(record: UserRecord) {
    selectedRecord = record;
    drawerVisible = true;
  }

  function clearSelection() {
    drawerVisible = false;
  }

  function navigateTo(view: AppView) {
    appState.setActiveView(view);
  }

  $: viewMeta = {
    Overview: {
      eyebrow: 'Dashboard',
      title: 'Operational overview',
      description:
        'Scan the entire local workspace through a polished desktop dashboard with high-level metrics, analytics, and fast record inspection.'
    },
    Records: {
      eyebrow: 'Records',
      title: 'Profile registry',
      description:
        'Search, browse, and inspect local profiles with a responsive data grid and side-by-side detail context.'
    },
    Analytics: {
      eyebrow: 'Analytics',
      title: 'Signal and throughput',
      description:
        'Presentation-first analytics surfaces help this prototype feel like a premium desktop monitoring tool.'
    },
    'Audit Trail': {
      eyebrow: 'Audit Trail',
      title: 'Timeline and activity',
      description:
        'Follow a focused event history around the currently selected record and review mock activity updates.'
    },
    Settings: {
      eyebrow: 'Settings',
      title: 'Workspace preferences',
      description:
        'Round out the concept with believable local controls, environment details, and desktop-style configuration panels.'
    }
  }[state.activeView];
</script>

{#if !state.isAuthenticated}
  <MockLogin onContinue={enterDashboard} />
{:else}
  <main class="min-h-screen bg-mesh-gradient px-4 py-4 text-white lg:px-6 lg:py-6">
    <div class="mx-auto grid max-w-[1600px] gap-4 lg:grid-cols-[300px_1fr]">
      <Sidebar records={state.records} activeView={state.activeView} onNavigate={navigateTo} />

      <section class="rounded-[32px] border border-white/10 bg-white/[0.04] p-5 backdrop-blur-xl lg:p-6">
        <Header
          eyebrow={viewMeta.eyebrow}
          title={viewMeta.title}
          description={viewMeta.description}
          query={state.query}
          onSearch={(value) => appState.setQuery(value)}
        />

        <div class="mt-6 grid gap-4 md:grid-cols-3">
          <div class="rounded-[28px] border border-white/10 bg-slate-950/60 p-5">
            <p class="text-sm uppercase tracking-[0.3em] text-slate-500">Total Records</p>
            <p class="mt-4 text-4xl font-semibold text-white">{state.records.length}</p>
          </div>
          <div class="rounded-[28px] border border-white/10 bg-slate-950/60 p-5">
            <p class="text-sm uppercase tracking-[0.3em] text-slate-500">Visible</p>
            <p class="mt-4 text-4xl font-semibold text-white">{filteredRecords.length}</p>
          </div>
          <div class="rounded-[28px] border border-white/10 bg-gradient-to-br from-cyan-400/20 to-sky-500/10 p-5">
            <p class="text-sm uppercase tracking-[0.3em] text-cyan-100/70">Storage Mode</p>
            <p class="mt-4 text-4xl font-semibold text-white">SQLite</p>
          </div>
        </div>

        {#if state.activeView === 'Overview'}
          <div class="mt-6">
            <AnalyticsPanel records={filteredRecords} />
          </div>

          <div class="mt-6 grid gap-4 2xl:grid-cols-[1.2fr_0.8fr]">
            <DataGrid
              records={filteredRecords}
              loading={state.loading}
              selectedId={selectedRecord?.id ?? null}
              onInspect={inspectRecord}
            />

            <div class="grid gap-4">
              <DetailsDrawer
                record={selectedRecord}
                visible={drawerVisible && selectedRecord !== null}
                onClose={clearSelection}
              />
              <AuditTimeline record={selectedRecord} />
            </div>
          </div>
        {:else if state.activeView === 'Records'}
          <div class="mt-6 grid gap-4 2xl:grid-cols-[1.2fr_0.8fr]">
            <DataGrid
              records={filteredRecords}
              loading={state.loading}
              selectedId={selectedRecord?.id ?? null}
              onInspect={inspectRecord}
            />

            <div class="grid gap-4">
              <DetailsDrawer
                record={selectedRecord}
                visible={drawerVisible && selectedRecord !== null}
                onClose={clearSelection}
              />
              <AuditTimeline record={selectedRecord} />
            </div>
          </div>
        {:else if state.activeView === 'Analytics'}
          <div class="mt-6">
            <AnalyticsPanel records={filteredRecords} />
          </div>
        {:else if state.activeView === 'Audit Trail'}
          <div class="mt-6 grid gap-4 xl:grid-cols-[0.9fr_1.1fr]">
            <AuditTimeline record={selectedRecord} />

            <section class="rounded-[32px] border border-white/10 bg-slate-950/65 p-5 shadow-glow backdrop-blur-xl">
              <p class="text-xs uppercase tracking-[0.28em] text-slate-500">Activity Feed</p>
              <h3 class="mt-2 text-xl font-semibold text-white">Record snapshots</h3>
              <div class="mt-5 grid gap-4 md:grid-cols-2">
                {#each filteredRecords.slice(0, 6) as record}
                  <button
                    on:click={() => inspectRecord(record)}
                    class={`rounded-[28px] border p-4 text-left transition ${
                      selectedRecord?.id === record.id
                        ? 'border-cyan-300/25 bg-cyan-400/[0.06]'
                        : 'border-white/10 bg-white/[0.03] hover:border-white/20 hover:bg-white/[0.05]'
                    }`}
                  >
                    <p class="text-sm font-medium text-white">{record.name}</p>
                    <p class="mt-1 text-sm text-slate-400">{record.email}</p>
                    <div class="mt-4 flex items-center justify-between">
                      <span class="text-xs uppercase tracking-[0.24em] text-slate-500">{record.date}</span>
                      <span class="rounded-full border border-white/10 bg-white/[0.04] px-3 py-1 text-xs text-slate-300">
                        {record.status}
                      </span>
                    </div>
                  </button>
                {/each}
              </div>
            </section>
          </div>
        {:else if state.activeView === 'Settings'}
          <div class="mt-6">
            <SettingsPanel />
          </div>
        {/if}
      </section>
    </div>
  </main>
{/if}
