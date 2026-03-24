import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export interface LocalCustomer {
  id: string;
  first_name: string;
  email: string;
  status: string;
  last_synced_at: string;
}

export interface UserRecord {
  id: string;
  name: string;
  email: string;
  status: 'Active' | 'Pending' | 'Archived';
  date: string;
}

export type AppView = 'Overview' | 'Records' | 'Analytics' | 'Audit Trail' | 'Settings';

export interface OperatorProfile {
  id: string;
  email: string;
  has_api_token: boolean;
  created_at: string;
  updated_at: string;
}

export interface OperatorStatus {
  has_operator: boolean;
  operator: OperatorProfile | null;
}

export interface SyncCustomersResponse {
  synced_count: number;
  synced_at: string;
  customers: LocalCustomer[];
}

export interface LocalApiStatus {
  enabled: boolean;
  host: string;
  port: number;
  base_url: string;
  auth_header: string;
  token_hint: string | null;
  key_source: string;
}

export interface SetupOperatorResponse {
  operator: OperatorProfile;
  recovery_key: string;
}

export interface DashboardState {
  initialized: boolean;
  isAuthenticated: boolean;
  loading: boolean;
  syncingCustomers: boolean;
  records: UserRecord[];
  query: string;
  activeView: AppView;
  operator: OperatorProfile | null;
  error: string | null;
  recoveryKey: string | null;
  localApi: LocalApiStatus | null;
  generatedLocalApiKey: string | null;
}

const initialState: DashboardState = {
  initialized: false,
  isAuthenticated: false,
  loading: false,
  syncingCustomers: false,
  records: [],
  query: '',
  activeView: 'Overview',
  operator: null,
  error: null,
  recoveryKey: null,
  localApi: null,
  generatedLocalApiKey: null
};

function mapCustomerToRecord(customer: LocalCustomer): UserRecord {
  const normalizedStatus = customer.status.toLowerCase();
  const statusMap: Record<string, UserRecord['status']> = {
    active: 'Active',
    prospect: 'Pending',
    pending: 'Pending',
    paused: 'Archived',
    archived: 'Archived'
  };

  return {
    id: customer.id,
    name: customer.first_name,
    email: customer.email,
    status: statusMap[normalizedStatus] ?? 'Pending',
    date: new Date(customer.last_synced_at).toLocaleDateString(undefined, {
      month: 'short',
      day: '2-digit',
      year: 'numeric'
    })
  };
}

function createAppState() {
  const { subscribe, update } = writable<DashboardState>(initialState);
  const fetchRecords = async () =>
    (await invoke<LocalCustomer[]>('get_local_customers')).map(mapCustomerToRecord);

  return {
    subscribe,
    async initialize() {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const [status, localApi] = await Promise.all([
          invoke<OperatorStatus>('get_operator_status'),
          invoke<LocalApiStatus>('get_local_api_status')
        ]);
        const records = status.operator?.has_api_token
          ? (await invoke<LocalCustomer[]>('get_local_customers')).map(mapCustomerToRecord)
          : [];

        update((state) => ({
          ...state,
          initialized: true,
          loading: false,
          operator: status.operator,
          localApi,
          generatedLocalApiKey: null,
          records,
          error: null
        }));
      } catch (error) {
        update((state) => ({
          ...state,
          initialized: true,
          loading: false,
          localApi: state.localApi,
          error: String(error)
        }));
      }
    },
    async setupOperator(email: string, password: string, apiToken: string) {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
      const response = await invoke<SetupOperatorResponse>('setup_operator', {
        email,
        masterPassword: password,
        apiToken
      });

      update((state) => ({
        ...state,
        initialized: true,
        isAuthenticated: true,
        loading: false,
        operator: response.operator,
        recoveryKey: response.recovery_key,
        error: null
      }));

        const records = await fetchRecords();
        update((state) => ({ ...state, records }));
      } catch (error) {
        update((state) => ({
          ...state,
          initialized: true,
          loading: false,
          error: String(error)
        }));
        throw error;
      }
    },
    async unlock(email: string, password: string) {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const operator = await invoke<OperatorProfile>('unlock_operator', {
          email,
          masterPassword: password
        });

        update((state) => ({
          ...state,
          isAuthenticated: true,
          loading: false,
          operator,
          error: null
        }));

        const records = await fetchRecords();
        update((state) => ({ ...state, records }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    logout() {
      update((state) => ({
        ...state,
        isAuthenticated: false,
        query: '',
        activeView: 'Overview',
        error: null,
        recoveryKey: null,
        generatedLocalApiKey: null
      }));
    },
    dismissRecoveryKey() {
      update((state) => ({ ...state, recoveryKey: null }));
    },
    dismissGeneratedLocalApiKey() {
      update((state) => ({ ...state, generatedLocalApiKey: null }));
    },
    setQuery(query: string) {
      update((state) => ({ ...state, query }));
    },
    setActiveView(activeView: AppView) {
      update((state) => ({ ...state, activeView }));
    },
    async loadRecords() {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const records = await fetchRecords();
        update((state) => ({ ...state, records, loading: false }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    async syncCustomers() {
      update((state) => ({ ...state, syncingCustomers: true, error: null }));
      try {
        const response = await invoke<SyncCustomersResponse>('sync_customers_from_cloud');
        update((state) => ({
          ...state,
          records: response.customers.map(mapCustomerToRecord),
          syncingCustomers: false,
          error: null
        }));
      } catch (error) {
        update((state) => ({ ...state, syncingCustomers: false, error: String(error) }));
        throw error;
      }
    },
    async saveApiToken(apiToken: string) {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const operator = await invoke<OperatorProfile>('save_api_token', { apiToken });
        update((state) => ({ ...state, operator, loading: false, error: null }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    async generateLocalApiKey() {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const response = await invoke<{ status: LocalApiStatus; plain_text_key: string }>(
          'generate_local_api_key'
        );
        update((state) => ({
          ...state,
          loading: false,
          localApi: response.status,
          generatedLocalApiKey: response.plain_text_key,
          error: null
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    async revokeLocalApiKey() {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const localApi = await invoke<LocalApiStatus>('revoke_local_api_key');
        update((state) => ({
          ...state,
          loading: false,
          localApi,
          generatedLocalApiKey: null,
          error: null
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    async resetPasswordWithRecoveryKey(email: string, recoveryKey: string, newPassword: string) {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const operator = await invoke<OperatorProfile>('reset_password_with_recovery_key', {
          email,
          recoveryKey,
          newPassword
        });
        update((state) => ({
          ...state,
          loading: false,
          operator,
          error: null
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false, error: String(error) }));
        throw error;
      }
    },
    setError(error: string | null) {
      update((state) => ({ ...state, error }));
    }
  };
}

export const appState = createAppState();
