import axios from 'axios';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:3000',
});

// Add auth interceptor
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export interface User {
  id: string;
  email: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface UploadSummary {
  id: string;
  filename: string;
  total_trips: number;
  cost_actual: number;
  cost_optimized: number;
  savings: number;
  uploaded_at: string;
}

// NOTE: This should match the JSON structure returned by the Rust API's `analyze` handler
// For now we type it loosely as `any` or a partial interface until we finalize the Rust JSON structure
export interface AnalysisResult {
  transponder_plate: string;
  direction: string;
  centroids: any[];
}

export interface AnalysisResponse {
  total_trips: number;
  total_cost: number;
  time_based_savings: number;
  distance_based_savings: number;
  time_analysis: AnalysisResult[];
  distance_analysis: AnalysisResult[];
}

export const endpoints = {
  login: (data: any) => api.post<AuthResponse>('/auth/login', data),
  register: (data: any) => api.post<AuthResponse>('/auth/register', data),
  history: () => api.get<UploadSummary[]>('/api/history'),
  analyze: (file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    return api.post<AnalysisResponse>('/api/analyze', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
  },
};

export default api;
