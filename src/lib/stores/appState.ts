import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface UserRecord {
  id: number;
  name: string;
  email: string;
  status: 'Active' | 'Pending' | 'Archived';
  date: string;
}

export type AppView = 'Overview' | 'Records' | 'Analytics' | 'Audit Trail' | 'Settings';

export interface DashboardState {
  isAuthenticated: boolean;
  loading: boolean;
  records: UserRecord[];
  query: string;
  activeView: AppView;
}

const initialState: DashboardState = {
  isAuthenticated: false,
  loading: false,
  records: [],
  query: '',
  activeView: 'Overview'
};

function createAppState() {
  const { subscribe, update, set } = writable<DashboardState>(initialState);

  return {
    subscribe,
    login() {
      update((state) => ({ ...state, isAuthenticated: true }));
    },
    logout() {
      set(initialState);
    },
    setQuery(query: string) {
      update((state) => ({ ...state, query }));
    },
    setActiveView(activeView: AppView) {
      update((state) => ({ ...state, activeView }));
    },
    async loadRecords() {
      update((state) => ({ ...state, loading: true }));
      const records = await invoke<UserRecord[]>('generate_mock_data');
      update((state) => ({ ...state, records, loading: false }));
    }
  };
}

export const appState = createAppState();
